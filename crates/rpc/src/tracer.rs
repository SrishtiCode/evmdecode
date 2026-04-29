use alloy::primitives::TxHash;
use alloy::providers::Provider;
use alloy::transports::http::{Client, Http};
use alloy::providers::RootProvider;
use anyhow::{anyhow, Result};
use serde_json::Value;

pub async fn trace_tx(
    provider: &RootProvider<Http<Client>>,
    hash: &str,
) -> Result<Value> {
    let tx_hash: TxHash = hash.parse()?;

    // prestateTracer returns before/after state for every touched account
    let params = serde_json::json!([
        format!("{:?}", tx_hash),
        { "tracer": "prestateTracer", "tracerConfig": { "diffMode": true } }
    ]);

    let result: Value = provider
        .client()
        .request("debug_traceTransaction", params)
        .await
        .map_err(|e| anyhow!("debug_traceTransaction failed: {e}\nNote: not all RPC nodes support debug_ namespace. Use Alchemy/Infura."))?;

    Ok(result)
}
