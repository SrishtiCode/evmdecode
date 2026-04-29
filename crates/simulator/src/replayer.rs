use alloy::providers::RootProvider;
use alloy::transports::http::{Client, Http};
use alloy::providers::Provider;
use anyhow::Result;
use evmdecode_core::types::RawTx;
use serde_json::Value;

pub async fn replay_tx(
    provider: &RootProvider<Http<Client>>,
    raw: &RawTx,
) -> Result<Value> {
    let block_tag = match raw.block_number {
        Some(n) => format!("0x{:x}", n),
        None    => "latest".to_string(),
    };

    let call_obj = serde_json::json!({
        "from":  raw.from,
        "to":    raw.to,
        "gas":   format!("0x{:x}", raw.gas),
        "value": format!("0x{:x}", raw.value.parse::<u128>().unwrap_or(0)),
        "data":  format!("0x{}", raw.input),
    });

    let result: Value = provider
        .client()
        .request("eth_call", serde_json::json!([call_obj, block_tag]))
        .await
        .map_err(|e| anyhow::anyhow!("eth_call failed: {e}"))?;

    Ok(result)
}
