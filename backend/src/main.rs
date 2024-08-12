
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::Vault::run().await
}
