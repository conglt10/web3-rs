use alloy::{providers::ProviderBuilder, sol, primitives::address};
use alloy::primitives::{
    utils::format_units,
};
use eyre::Result;
use std::env;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "./abi/ERC20.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let rpc_url = env::var("RPC_URL")?.parse()?;
    let erc20_address = env::var("ERC20_ADDRESS")?.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let contract = ERC20::new(erc20_address, provider);

    // Call the contract, retrieve the total supply.
    let total_supply = contract.totalSupply().call().await?._0;

    println!("ERC-20 total supply is {total_supply}");

    let balance = contract.balanceOf(address!("0000000000000000000000000000000000000000")).call().await?._0;
    println!("Balance of zero address: {balance}");

    let decimals = contract.decimals().call().await?._0;
    println!("Decimals of token is {decimals}");

    let ether_balance = format_units(balance, decimals)?;
    println!("Balance of zero address: {ether_balance}");


    Ok(())
}