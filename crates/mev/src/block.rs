use alloy::primitives::{Address, TxHash, B256};
use serde::Deserialize;
use crate::decode::{RawLog, ERC20_TRANSFER_TOPIC};

#[derive(Debug, Deserialize)]
struct RpcResponse<T> { result: T }

#[derive(Debug, Deserialize, Clone)]
pub struct BlockTx {
    pub hash: TxHash,
    pub from: Address,
    pub to:   Option<Address>,
    #[serde(rename = "input")]
    pub input: alloy::primitives::Bytes,
    #[serde(rename = "transactionIndex")]
    pub index: Option<alloy::primitives::ruint::aliases::U64>,
}

#[derive(Debug, Deserialize)]
struct Block { pub transactions: Vec<BlockTx> }

#[derive(Debug, Deserialize)]
struct ReceiptLog {
    pub address: Address,
    pub topics:  Vec<B256>,
    pub data:    alloy::primitives::Bytes,
}

#[derive(Debug, Deserialize)]
struct Receipt { pub logs: Vec<ReceiptLog> }

pub struct BlockContext {
    pub all_txs:      Vec<BlockTx>,
    pub target_index: usize,
}

pub async fn fetch_block_context(
    rpc_url: &str,
    block_number: u64,
    tx_hash: TxHash,
) -> eyre::Result<Option<BlockContext>> {
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "jsonrpc": "2.0", "id": 1,
        "method": "eth_getBlockByNumber",
        "params": [format!("0x{:x}", block_number), true]
    });
    let resp: RpcResponse<Option<Block>> =
        client.post(rpc_url).json(&body).send().await?.json().await?;

    let block = match resp.result { Some(b) => b, None => return Ok(None) };
    let target_index = block.transactions.iter().position(|tx| tx.hash == tx_hash);
    match target_index {
        Some(idx) => Ok(Some(BlockContext { all_txs: block.transactions, target_index: idx })),
        None => Ok(None),
    }
}

pub async fn fetch_transfer_logs(rpc_url: &str, tx_hash: TxHash) -> eyre::Result<Vec<RawLog>> {
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "jsonrpc": "2.0", "id": 1,
        "method": "eth_getTransactionReceipt",
        "params": [format!("{:?}", tx_hash)]
    });
    let resp: RpcResponse<Option<Receipt>> =
        client.post(rpc_url).json(&body).send().await?.json().await?;

    let receipt = match resp.result { Some(r) => r, None => return Ok(vec![]) };
    Ok(receipt.logs.into_iter()
        .filter(|log| log.topics.first() == Some(&ERC20_TRANSFER_TOPIC))
        .map(|log| RawLog { address: log.address, topics: log.topics, data: log.data })
        .collect())
}

pub fn token_set_from_logs(logs: &[RawLog]) -> std::collections::HashSet<Address> {
    logs.iter().map(|l| l.address).collect()
}
