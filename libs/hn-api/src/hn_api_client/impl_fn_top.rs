use bytes::Buf;

use crate::error::Error;
use crate::hn_api_client::HnApiClient;

impl HnApiClient {
    pub async fn top(&self) -> Result<Vec<i64>, Error> {
        let mut url = self.base_url.to_owned();
        url.path_segments_mut()
            .expect("could not get path segments")
            .extend(["v0", "topstories.json"]);
        let request = self.http_client.request(reqwest::Method::GET, url).build()?;

        let response = self.http_client.execute(request).await?;

        if !response.status().is_success() {
            return Err(Error::generic(format!("Non 2xx-status: {:?}", response.status())))
        }

        let body_bytes = response.bytes().await?;
        let ids: Vec<i64> = serde_json::from_reader(body_bytes.reader())?;

        Ok(ids)
    }
}
