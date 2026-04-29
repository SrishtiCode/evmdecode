use alloy::providers::{ProviderBuilder, RootProvider};
use alloy::transports::http::{Client, Http};
use anyhow::Result;

pub type HttpProvider = RootProvider<Http<Client>>;

pub fn build_provider(rpc_url: &str) -> Result<HttpProvider> {
    let provider = ProviderBuilder::new()
        .on_http(rpc_url.parse()?);
    Ok(provider)
}
