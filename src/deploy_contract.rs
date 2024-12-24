use alloy::{
    providers::ProviderBuilder,
    network::{AnyNetwork, EthereumWallet},
    signers::local::PrivateKeySigner,
    sol
};
use eyre::Result;
use std::env;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Counter,
    "./artifacts/Counter.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let rpc_url = env::var("RPC_URL")?.parse()?;
    let private_key: PrivateKeySigner = env::var("PRIVATE_KEY")?.parse()?;
    let deployer = EthereumWallet::from(private_key);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .network::<AnyNetwork>()
        .wallet(deployer)
        .on_http(rpc_url);

    let contract = Counter::deploy(&provider).await?;

    println!("Deployed contract at address: {}", contract.address());

    Ok(())
}