pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP client error")]
    HttpClientError(#[source] reqwest::Error),

    #[error("URL parse error")]
    UrlParseError(#[source] url::ParseError),

    #[error("Ser/De error")]
    Serde(#[source] serde_json::Error),

    #[error("Generic")]
    Generic(#[source] AnyError),
}

impl Error {
    pub fn generic<E>(inner: E) -> Self
    where
        E: Into<AnyError>,
    {
        Self::Generic(inner.into())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::HttpClientError(value)
    }
}
impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParseError(value)
    }
}
