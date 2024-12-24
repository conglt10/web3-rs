use alloy::{
    hex,
    network::{AnyNetwork, EthereumWallet, TransactionBuilder, ReceiptResponse},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};
use eyre::Result;
use std::env;
use alloy_serde::WithOtherFields;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract Counter {
        uint256 public number;

        function setNumber(uint256 newNumber) public {
            number = newNumber;
        }

        function increment() public {
            number++;
        }
    }
}

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

    // Deploy the `Counter` contract from bytecode at runtime.
    let bytecode = hex::decode(
        // solc v0.8.26; solc Counter.sol --via-ir --optimize --bin
        "6080806040523460135760df908160198239f35b600080fdfe6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033"
    )?;
    let tx = TransactionRequest::default().with_deploy_code(bytecode);

    // Deploy the contract.
    let receipt = provider.send_transaction(WithOtherFields::new(tx)).await?.get_receipt().await?;

    let contract_address = receipt.contract_address().expect("Failed to get contract address");
    let contract = Counter::new(contract_address, &provider);
    println!("Deployed contract at address: {}", contract.address());

    Ok(())
}
