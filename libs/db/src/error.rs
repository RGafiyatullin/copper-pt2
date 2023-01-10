pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Generic")]
    Generic(#[source] AnyError),

    #[error("Db")]
    Db(#[source] tokio_postgres::Error),
}

impl DbError {
    pub fn generic<E>(inner: E) -> Self
    where
        E: Into<AnyError>,
    {
        Self::Generic(inner.into())
    }
}

// default impl<E> From<E> for DbError where E: Into<Box<dyn std::error::Error + Send + Sync +
// 'static>> {     fn from(value: E) -> Self {
//         Self::Generic(value.into())
//     }
// }

impl From<tokio_postgres::Error> for DbError {
    fn from(value: tokio_postgres::Error) -> Self {
        Self::Db(value)
    }
}
