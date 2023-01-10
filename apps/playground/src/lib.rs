use std::future::Future;

pub type PgClient = tokio_postgres::Client;
pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub use hn_api::HnApiClient;

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
