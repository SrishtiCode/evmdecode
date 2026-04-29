use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTx {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub input: String,
    pub value: String,
    pub gas: u128,
    pub gas_price: Option<u128>,
    pub block_number: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedCall {
    pub selector: String,
    pub function_sig: Option<String>,
    pub params: Vec<DecodedParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedParam {
    pub name: Option<String>,
    pub kind: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDiff {
    pub address: String,
    pub storage_changes: HashMap<String, StorageChange>,
    pub balance_before: Option<String>,
    pub balance_after: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageChange {
    pub slot: String,
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasReport {
    pub gas_used: u128,
    pub intrinsic: u128,
    pub calldata_cost: u128,
    pub storage_cost: u128,
    pub execution_cost: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MevAlert {
    pub kind: MevKind,
    pub description: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MevKind { UniswapSwap, SandwichVulnerable, Arbitrage }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity { Info, Warning, Critical }
