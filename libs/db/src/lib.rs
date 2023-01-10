pub type DateTime = chrono::DateTime<chrono::FixedOffset>;
pub type DbTran<'a> = tokio_postgres::Transaction<'a>;
pub type DbConn = tokio_postgres::Client;

mod error;
pub use error::DbError;

pub mod items;
pub mod snapshots;
pub mod users;
