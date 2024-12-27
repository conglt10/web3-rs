use alloy::sol;
use eyre::Result;
mod config;
use config::network::get_provider;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Counter,
    "./artifacts/Counter.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    let provider = get_provider().expect("Can not get provider");

    let contract = Counter::deploy(provider).await?;

    println!("Deployed contract at address: {}", contract.address());

    Ok(())
}
