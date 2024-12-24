use alloy::providers::{Provider, ProviderBuilder};
use eyre::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let rpc_url = env::var("RPC_URL")?.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Get latest block number.
    let latest_block = provider.get_block_number().await?;

    // Print the block number.
    println!("Latest block number: {latest_block}");

    Ok(())
}
