use alloy::signers::local::{coins_bip39::English, MnemonicBuilder};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let phrase = "test test test test test test test test test test test junk"; // anvil localhost

    let wallet = MnemonicBuilder::<English>::default()
        .phrase(phrase)
        .index(0)? // m/44'/60'/0'/0/0
        .build()?;

    println!("Wallet: {}", wallet.address());
    println!(
        "Private key: {:?}",
        format!("0x{}", hex::encode(wallet.credential().to_bytes()))
    );

    let wallet = MnemonicBuilder::<English>::default()
        .word_count(24)
        .derivation_path("m/44'/60'/0'/0/0")?
        .build_random()?;

    println!("Random wallet: {}", wallet.address());
    println!(
        "Private key: {:?}",
        format!("0x{}", hex::encode(wallet.credential().to_bytes()))
    );
    Ok(())
}
