use alloy::signers::local::PrivateKeySigner;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let wallet = PrivateKeySigner::random();
    println!("Address: {:?}", wallet.address());
    println!(
        "Private key: {:?}",
        format!("0x{}", hex::encode(wallet.credential().to_bytes()))
    );

    Ok(())
}
