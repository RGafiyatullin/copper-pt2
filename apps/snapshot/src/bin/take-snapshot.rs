pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, structopt::StructOpt)]
struct Args {
    #[structopt(flatten)]
    db: snapshot::args::ArgsDb,

    #[structopt(flatten)]
    hn_api: snapshot::args::ArgsHNApi,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AnyError> {
    let _ = dotenv::dotenv();
    pretty_env_logger::init_timed();

    let args: Args = structopt::StructOpt::from_args();
    let args = &args;

    let snapshot_id = snapshot::with_db(&args.db.db, |mut db| async move {
        snapshot::with_hn_api(&args.hn_api.hn_api_base_url, move |hn_api| async move {
            snapshot::single_snapshot(args.hn_api.hn_api_concurrency, &mut db, &hn_api).await
        })
        .await
    })
    .await?;

    println!("SNAPSHOT: {:?}", snapshot_id);

    Ok(())
}
