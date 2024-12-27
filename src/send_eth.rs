use alloy::rpc::types::TransactionRequest;
use alloy::primitives::utils::parse_ether;
use alloy::providers::Provider;
use alloy::network::TransactionBuilder;
use alloy_serde::WithOtherFields;
use alloy::signers::local::PrivateKeySigner;
use eyre::Result;
mod config;
use config::config::{get_provider, get_wallet_address};


#[tokio::main]
async fn main() -> Result<()> {
    let provider = get_provider().expect("Can not get provider");

    let sender = get_wallet_address().unwrap();

    // random address
    let receiver = PrivateKeySigner::random().address();

    let tx = TransactionRequest::default().with_from(sender).with_to(receiver).with_value(parse_ether("0.0001")?);

    let tx_hash = provider.send_transaction(WithOtherFields::new(tx)).await?.watch().await?;

    println!("transaction hash: {tx_hash}");
    Ok(())
}

