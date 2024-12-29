use alloy::{
    primitives::{address, b256, utils::format_units},
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let latest_block = provider.get_block_number().await?;

    // Get all logs from the latest block emitted by the UNI token address.
    let weth_token_address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    let transfer_event_signature =
        b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");
    let filter = Filter::new()
        .event_signature(transfer_event_signature)
        .address(weth_token_address)
        .from_block(latest_block);

    // Get all logs from the latest block that match the filter.
    let logs = provider.get_logs(&filter).await?;

    for log in logs {
        println!("Tx hash: {:?}", log.transaction_hash.unwrap());
        let topic1 = hex::encode(log.data().topics()[1]);
        let topic2 = hex::encode(log.data().topics()[2]);

        let src = format!("0x{}", &topic1[26..]);
        let dst = format!("0x{}", &topic2[26..]);
        let amount_hex = log.data().data.to_string();
        let amount = u64::from_str_radix(amount_hex.trim_start_matches("0x"), 16)
            .expect("Invalid hex string");

        println!(
            "Transfer from {src} to {dst} with amount {} ether or {amount} wei:  \n",
            format_units(amount, 18)?
        );
    }

    Ok(())
}
