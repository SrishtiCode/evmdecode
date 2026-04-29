use alloy::primitives::{TxHash, Address};
use alloy::providers::{Provider, RootProvider};
use alloy::transports::http::{Client, Http};
use anyhow::{anyhow, Result};
use evmdecode_core::types::RawTx;
use tokio::time::{sleep, Duration};

pub async fn fetch_tx(provider: &RootProvider<Http<Client>>, hash: &str) -> Result<RawTx> {
    let tx_hash: TxHash = hash.parse()?;

    for attempt in 0..3u32 {
        match tokio::time::timeout(
            Duration::from_secs(10),
            provider.get_transaction_by_hash(tx_hash),
        ).await {
            Ok(Ok(Some(tx))) => {
                return Ok(RawTx {
                    hash: hash.to_string(),
                    from: tx.from.to_string(),
                    to: tx.to.map(|a| a.to_string()),
                    input: hex::encode(&tx.input),
                    value: tx.value.to_string(),
                    gas: tx.gas,
                    gas_price: tx.gas_price,
                    block_number: tx.block_number,
                });
            }
            Ok(Ok(None)) => return Err(anyhow!("tx not found: {}", hash)),
            Ok(Err(e)) => {
                if attempt == 2 { return Err(e.into()); }
                sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            Err(_) => return Err(anyhow!("RPC timeout after 10s")),
        }
    }
    unreachable!()
}

pub async fn fetch_bytecode(
    provider: &RootProvider<Http<Client>>,
    address: &str,
) -> Result<Vec<u8>> {
    let addr: Address = address.parse()
        .map_err(|_| anyhow!("invalid address: {}", address))?;

    let code = provider.get_code_at(addr).await?;

    if code.is_empty() {
        anyhow::bail!("no bytecode at {} — is this an EOA?", address);
    }

    Ok(code.to_vec())
}
