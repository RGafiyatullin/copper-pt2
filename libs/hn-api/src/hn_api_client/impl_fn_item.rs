use std::collections::HashMap;

use bytes::Buf;

use crate::error::Error;
use crate::hn_api_client::HnApiClient;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemInfo {
    #[serde(flatten)]
    pub common: ItemInfoCommon,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemInfoCommon {
    pub by: String,
    pub time: i64,
    // pub score: i32,
}

// pub type ItemInfo = serde_json::Value;

impl HnApiClient {
    pub async fn item(&self, item_id: i64) -> Result<ItemInfo, Error> {
        log::trace!("item({})", item_id);

        let mut url = self.base_url.to_owned();
        url.path_segments_mut()
            .expect("could not get path segments")
            .extend(["v0", "item"])
            .extend([format!("{}.json", item_id)]);
        let request = self.http_client.request(reqwest::Method::GET, url).build()?;

        let response = self.http_client.execute(request).await?;

        if !response.status().is_success() {
            return Err(Error::generic(format!("Non 2xx-status: {:?}", response.status())))
        }

        let body_bytes = response.bytes().await?;

        let item_info: ItemInfo = serde_json::from_reader(body_bytes.reader())?;

        // let body_bytes = body_bytes.into_iter().collect::<Vec<_>>();
        // log::trace!("item({}) body: {:?}", item_id, body_bytes);
        // let item_info = serde_json::from_reader(std::io::Cursor::new(body_bytes))?;

        Ok(item_info)
    }
}
