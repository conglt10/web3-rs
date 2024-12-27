pub mod config {
    use std::env;
    use alloy::{
        network::{AnyNetwork, EthereumWallet}, primitives::Address, providers::{fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller}, ProviderBuilder, RootProvider}, signers::local::PrivateKeySigner
    };
    use std::error::Error;
    use reqwest::Client;
    use reqwest::Url;

    type Provider = FillProvider<JoinFill<JoinFill<alloy::providers::Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>>, WalletFiller<EthereumWallet>>, RootProvider<alloy::transports::http::Http<Client>, AnyNetwork>, alloy::transports::http::Http<Client>, AnyNetwork>;

    pub fn get_provider() -> Result<Provider, Box<dyn Error>> {
        let rpc_url = get_rpc_url()?;
        let private_key: PrivateKeySigner = get_private_key()?;
        let wallet = EthereumWallet::from(private_key);
        let provider  = ProviderBuilder::new()
            .with_recommended_fillers()
            .network::<AnyNetwork>()
            .wallet(wallet)
            .on_http(rpc_url);
        Ok(provider)
    }

    pub fn get_wallet_address() -> Result<Address, Box<dyn Error>> {
        let private_key: PrivateKeySigner = get_private_key()?;
        Ok(private_key.address())
    }

    fn get_private_key() -> Result<PrivateKeySigner,  Box<dyn Error>> {
        dotenvy::dotenv()?;

        let private_key: PrivateKeySigner = env::var("PRIVATE_KEY")?.parse()?;
        Ok(private_key)
    }

    fn get_rpc_url() -> Result<Url,  Box<dyn Error>> {
        dotenvy::dotenv()?;

        let rpc_url = env::var("RPC_URL")?.parse()?;
        Ok(rpc_url)
    }
}
