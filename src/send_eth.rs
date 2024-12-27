use alloy::network::TransactionBuilder;
use alloy::primitives::utils::parse_ether;
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy_serde::WithOtherFields;
use eyre::Result;
mod config;
use config::network::{get_provider, get_wallet_address};

#[tokio::main]
async fn main() -> Result<()> {
    let provider = get_provider().expect("Can not get provider");

    let sender = get_wallet_address().expect("Can not get wallet address");

    // random address
    let receiver = PrivateKeySigner::random().address();

    let tx = TransactionRequest::default()
        .with_from(sender)
        .with_to(receiver)
        .with_value(parse_ether("0.0001")?);

    let tx_hash = provider
        .send_transaction(WithOtherFields::new(tx))
        .await?
        .watch()
        .await?;

    println!("transaction hash: {tx_hash}");
    Ok(())
}
