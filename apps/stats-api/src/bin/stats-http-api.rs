type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), AnyError> {
    std::future::pending().await
}
