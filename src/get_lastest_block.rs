use alloy::providers::Provider;
use eyre::Result;

mod config;
use config::config::get_provider;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = get_provider().expect("Can not get provider");

    // Get latest block number.
    let latest_block = provider.get_block_number().await?;

    // Print the block number.
    println!("Latest block number: {latest_block}");

    Ok(())
}
