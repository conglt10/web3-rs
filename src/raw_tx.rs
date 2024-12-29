use alloy::{
    network::TransactionBuilder,
    network::{AnyNetwork, EthereumWallet},
    primitives::utils::parse_ether,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use alloy_serde::WithOtherFields;
use eyre::Result;
use std::env;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let rpc_url = env::var("RPC_URL")?.parse()?;
    let private_key: PrivateKeySigner = env::var("PRIVATE_KEY")?.parse()?;
    let wallet = EthereumWallet::from(private_key.clone());
    let provider = ProviderBuilder::new()
        .network::<AnyNetwork>()
        .wallet(wallet)
        .on_http(rpc_url);
    let receiver = PrivateKeySigner::random().address();

    let nonce = provider
        .get_transaction_count(private_key.address())
        .pending()
        .await?;
    println!("Nonce: {}", nonce);

    let tx = TransactionRequest::default()
        .with_to(receiver)
        .with_nonce(nonce)
        .with_chain_id(provider.get_chain_id().await?)
        .with_value(parse_ether("0.0001")?)
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000);

    let pending_tx = provider.send_transaction(WithOtherFields::new(tx)).await?;

    println!("Pending transaction... {}", pending_tx.tx_hash());

    // Wait for the transaction to be included and get the receipt.
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    Ok(())
}
