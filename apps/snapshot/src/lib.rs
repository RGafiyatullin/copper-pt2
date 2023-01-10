use std::future::Future;

pub type PgClient = tokio_postgres::Client;
pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

use chrono::Offset;
use futures::StreamExt;
pub use hn_api::HnApiClient;

pub mod args;

pub async fn with_db<V, E, F, Fut>(db_config: &str, f: F) -> Result<V, AnyError>
where
    F: FnOnce(PgClient) -> Fut,
    Fut: Future<Output = Result<V, E>>,
    E: Into<AnyError>,
{
    let (client, conn) = tokio_postgres::connect(db_config, tokio_postgres::NoTls).await?;
    let client_running = async move { f(client).await.map_err(Into::into) };
    let conn_running = async move { conn.await.map_err(Into::into) };

    let (value, ()) = futures::future::try_join(client_running, conn_running).await?;

    Ok(value)
}

pub async fn with_hn_api<V, E, F, Fut>(base_url: &str, f: F) -> Result<V, AnyError>
where
    F: FnOnce(HnApiClient) -> Fut,
    Fut: Future<Output = Result<V, E>>,
    E: Into<AnyError>,
{
    let client = HnApiClient::new(base_url)?;
    f(client).await.map_err(Into::into)
}

pub async fn single_snapshot(
    hn_api_concurrency: usize,
    db: &mut PgClient,
    hn_api: &HnApiClient,
) -> Result<i64, AnyError> {
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
        futures::stream::iter(item_fetch_workers).buffer_unordered(hn_api_concurrency);

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
