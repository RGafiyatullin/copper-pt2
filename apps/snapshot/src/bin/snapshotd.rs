pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, structopt::StructOpt)]
struct Args {
    #[structopt(flatten)]
    db: snapshot::args::ArgsDb,

    #[structopt(flatten)]
    hn_api: snapshot::args::ArgsHNApi,

    #[structopt(long, short = "i", env = "SNAPSHOT_INTERVAL", default_value = "1m")]
    poll_interval: snapshot::args::ArgDuration,

    #[structopt(long, env = "MAX_FAILURES_IN_A_ROW", default_value = "3")]
    max_failures_in_a_row: usize,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AnyError> {
    let _ = dotenv::dotenv();
    pretty_env_logger::init_timed();

    let args: Args = structopt::StructOpt::from_args();
    let args = &args;

    snapshot::with_db(&args.db.db, |mut db| async move {
        snapshot::with_hn_api(&args.hn_api.hn_api_base_url, move |hn_api| async move {
            let mut failures_left = args.max_failures_in_a_row;
            loop {
                match snapshot::single_snapshot(args.hn_api.hn_api_concurrency, &mut db, &hn_api)
                    .await
                {
                    Ok(snapshot_id) => {
                        failures_left = args.max_failures_in_a_row;
                        log::info!("successfully made a snapshot [id: {}]", snapshot_id)
                    },
                    Err(reason) => {
                        log::error!("failed to make a snapshot: {}", reason);
                        if let Some(left) = failures_left.checked_sub(1) {
                            log::info!("retries left: {}", left);
                            failures_left = left;
                        } else {
                            break Err(reason)
                        }
                    },
                }
                tokio::time::sleep(args.poll_interval.into()).await;
            }
        })
        .await
    })
    .await
}
