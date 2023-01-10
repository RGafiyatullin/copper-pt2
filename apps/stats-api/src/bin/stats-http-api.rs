type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, structopt::StructOpt)]
struct Args {
    #[structopt(flatten)]
    db: stats_api::args::ArgsDb,
    #[structopt(flatten)]
    http: stats_api::args::ArgsHttp,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), AnyError> {
    let _ = dotenv::dotenv();
    pretty_env_logger::init_timed();

    let args: Args = structopt::StructOpt::from_args();
    let args = &args;

    stats_api::with_db(&args.db.db, |db| async move { run(args, db).await }).await
}

async fn run(args: &Args, db_pool: stats_api::PgPool) -> Result<(), AnyError> {
    let app = axum::Router::new()
        .route("/current-top", axum::routing::get(stats_api::http_api::current_top))
        .route("/posts-by-user/:user_name", axum::routing::get(stats_api::http_api::posts_by_user))
        .route(
            "/users-reaching-top/:rank_below",
            axum::routing::get(stats_api::http_api::users_reaching_top),
        )
        .with_state(db_pool);

    log::info!("starting http-server at {:?} ...", args.http.bind_addr);
    axum::Server::bind(&args.http.bind_addr).serve(app.into_make_service()).await?;

    Ok(())
}
