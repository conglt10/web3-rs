use alloy::sol;
use alloy::signers::local::PrivateKeySigner;
use eyre::Result;
use std::env;
mod config;
use config::config::get_provider;

sol!(
    #[derive(Debug)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC721,
    "./abi/ERC721.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let erc721_address = env::var("ERC721_ADDRESS")?.parse()?;
    let provider = get_provider().expect("Can not get provider");
    let receiver = PrivateKeySigner::random().address();

    let contract = ERC721::new(erc721_address, provider);
    let result = contract.mint(receiver).send().await?.get_receipt().await;

    println!("transaction hash: {:?}", result?.transaction_hash);

    let balance = contract.balanceOf(receiver).call().await?;
    println!("Balance NFT of {}: {:#?}", receiver, balance._0);

    Ok(())
}
