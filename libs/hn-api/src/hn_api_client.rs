mod impl_fn_item;
mod impl_fn_new;
mod impl_fn_top;

#[derive(Debug)]
pub struct HnApiClient {
    http_client: reqwest::Client,
    base_url: url::Url,
}
