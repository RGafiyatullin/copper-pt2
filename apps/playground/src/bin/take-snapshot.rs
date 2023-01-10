use chrono::Offset;
use futures::StreamExt;
use playground::{HnApiClient, PgClient};

pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, structopt::StructOpt)]
struct Args {
    #[structopt(
        long,
        env = "DB",
        default_value = "host=127.0.0.1 port=5432 user=dev password=dev dbname=dev"
    )]
    db: String,

    #[structopt(long, env = "HN_API_CONCURRENCY", default_value = "1")]
    hn_api_concurrency: usize,

    #[structopt(long, env = "HN_API_BASE_URL", default_value = "https://hacker-news.firebaseio.com/")]
    hn_api_base_url: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AnyError> {
    let _ = dotenv::dotenv();
    let () = pretty_env_logger::init_timed();

    let args: Args = structopt::StructOpt::from_args();
    let args = &args;

    let snapshot_id = playground::with_db(&args.db, |db| async move {
        playground::with_hn_api(&args.hn_api_base_url, move |hn_api| async move {
            run(&args, db, hn_api).await
        })
        .await
    })
    .await?;

    println!("SNAPSHOT: {:?}", snapshot_id);

    Ok(())
}

async fn run(args: &Args, mut db: PgClient, hn_api: HnApiClient) -> Result<i64, AnyError> {
    let top_ids = hn_api.top().await?;

    let now: db::DateTime = chrono::Utc::now().into();
    let tran = db.transaction().await?;

    let snapshot_id = db::snapshots::insert_snapshot(&tran, now).await?;
    log::debug!("TOP-STORIES: {:?}", top_ids);

    let item_fetch_workers = top_ids.into_iter().enumerate().map(|(rank, item_id)| {
        let hn_api = &hn_api;
        async move {
            let item_info = hn_api.item(item_id).await?;
            log::trace!("item-info: {:?}", item_info);
            Result::<_, hn_api::Error>::Ok((item_id, rank, item_info))
        }
    });

    let mut items_fetched =
        futures::stream::iter(item_fetch_workers).buffer_unordered(args.hn_api_concurrency);

    while let Some(item_fetch_result) = items_fetched.next().await {
        let (item_id, rank, item_info) = item_fetch_result?;

        let user_id = db::users::get_or_insert(&tran, &item_info.common.by).await?;
        log::trace!("user {:?} -> {}", item_info.common.by, user_id);

        let posted_at = chrono::DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp_millis(item_info.common.time * 1_000).unwrap(),
            chrono::Utc.fix(),
        );
        let item_db_id = db::items::get_or_insert(&tran, item_id, user_id, posted_at).await?;
        log::trace!("item {} -> {}", item_id, item_db_id);

        let rank_entry_id =
            db::snapshots::insert_top_rank(&tran, snapshot_id, item_db_id, rank as i32).await?;
        log::trace!("rank-entry {}", rank_entry_id);

        log::debug!(
            "- {}@{} by {:?}({}) -> {}",
            item_id,
            rank,
            user_id,
            item_info.common.by,
            item_db_id
        );
    }

    tran.commit().await?;

    Ok(snapshot_id)
}
