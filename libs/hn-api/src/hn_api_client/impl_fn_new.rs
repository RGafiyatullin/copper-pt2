use std::str::FromStr;

use reqwest::Client;

use crate::error::Error;
use crate::hn_api_client::HnApiClient;

impl HnApiClient {
    pub fn new(base_url: &str) -> Result<Self, Error> {
        let http_client = Client::builder().build()?;
        let base_url = reqwest::Url::from_str(base_url)?;

        let client = Self { http_client, base_url };

        Ok(client)
    }
}
