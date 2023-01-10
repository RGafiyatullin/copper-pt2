use std::collections::HashMap;
use std::future::Future;

pub type PgPool = deadpool_postgres::Pool;
pub type PgClient = tokio_postgres::Client;
pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod args;
pub mod http_api;

mod utils;

pub async fn with_db<V, E, F, Fut>(db_config: &str, f: F) -> Result<V, AnyError>
where
    F: FnOnce(PgPool) -> Fut,
    Fut: Future<Output = Result<V, E>>,
    E: Into<AnyError>,
{
    let db_config_props = db_config
        .split_ascii_whitespace()
        .filter_map(|pair| pair.split_once('='))
        .collect::<HashMap<_, _>>();

    let mut db_config = deadpool_postgres::Config::new();
    db_config.user = db_config_props.get("user").copied().map(ToOwned::to_owned);
    db_config.password = db_config_props.get("password").copied().map(ToOwned::to_owned);
    db_config.dbname = db_config_props.get("dbname").copied().map(ToOwned::to_owned);
    db_config.host = db_config_props.get("host").copied().map(ToOwned::to_owned);
    db_config.port = db_config_props.get("port").copied().map(str::parse::<u16>).transpose()?;

    let pool = db_config.create_pool(None, tokio_postgres::NoTls)?;

    f(pool).await.map_err(Into::into)
}
