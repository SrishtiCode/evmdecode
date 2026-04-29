use std::collections::HashSet;
use alloy::primitives::{Address, TxHash};
use crate::block::{fetch_block_context, fetch_transfer_logs, token_set_from_logs, BlockTx};
use crate::decode::{classify_selector, is_known_router, RawLog, SwapLog, SwapVersion};

#[derive(Debug, Clone)]
pub struct MevAlert {
    pub kind:       MevKind,
    pub confidence: f32,
    pub detail:     String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MevKind { Swap, SandwichAttack, Arbitrage }

impl std::fmt::Display for MevKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MevKind::Swap          => write!(f, "Swap"),
            MevKind::SandwichAttack => write!(f, "Sandwich attack"),
            MevKind::Arbitrage     => write!(f, "Arbitrage"),
        }
    }
}

pub async fn detect(
    swap_logs:    Vec<SwapLog>,
    raw_logs:     Vec<RawLog>,
    selector:     Option<[u8; 4]>,
    to:           Option<Address>,
    block_number: u64,
    tx_hash:      TxHash,
    rpc_url:      &str,
) -> Vec<MevAlert> {
    let mut alerts = Vec::new();

    if let Some(alert) = detect_swap(&swap_logs, selector.as_ref(), to.as_ref()) {
        alerts.push(alert);
    }
    if let Some(alert) = detect_arbitrage(&swap_logs, &raw_logs) {
        alerts.push(alert);
    }

    let is_swap = selector.as_ref().is_some_and(|s| classify_selector(s).is_some())
        || !swap_logs.is_empty();

    if is_swap {
        if let Ok(Some(alert)) = detect_sandwich(tx_hash, block_number, rpc_url).await {
            alerts.push(alert);
        }
    }

    alerts
}

fn detect_swap(
    swap_logs: &[SwapLog],
    selector:  Option<&[u8; 4]>,
    to:        Option<&Address>,
) -> Option<MevAlert> {
    let sel_match = selector.and_then(classify_selector);
    let router_match = to.map(is_known_router).unwrap_or(false);
    let has_swap_log = !swap_logs.is_empty();

    if !has_swap_log && sel_match.is_none() { return None; }

    let confidence = match (sel_match.is_some(), router_match, has_swap_log) {
        (_, _, true) if swap_logs.len() >= 2 => 0.92,
        (true, true, true)  => 0.98,
        (true, true, false) => 0.85,
        (true, false, true) => 0.80,
        (false, true, true) => 0.85,
        (false, false, true) => 0.80,
        _ => 0.50,
    };

    let version = sel_match
        .or_else(|| swap_logs.first().map(|l| l.version))
        .map(|v| match v { SwapVersion::V2 => "Uniswap V2", SwapVersion::V3 => "Uniswap V3" })
        .unwrap_or("Unknown DEX");

    let pool_count = swap_logs.len();
    let detail = if pool_count > 0 {
        format!("{version} swap across {pool_count} pool(s)")
    } else {
        format!("{version} swap (selector match, no Swap logs in receipt)")
    };

    Some(MevAlert { kind: MevKind::Swap, confidence, detail })
}

fn detect_arbitrage(swap_logs: &[SwapLog], _raw_logs: &[RawLog]) -> Option<MevAlert> {
    if swap_logs.len() < 2 { return None; }
    let pools: HashSet<Address> = swap_logs.iter().map(|s| s.pool).collect();
    if pools.len() < 2 { return None; }

    let cycle_detected = {
        let sends = swap_logs.iter().any(|s| s.amount0 < 0);
        let recvs = swap_logs.iter().any(|s| s.amount0 > 0);
        sends && recvs
    };
    if !cycle_detected { return None; }

    let pool_count = pools.len();
    let confidence = match pool_count { 2 => 0.72, 3 => 0.85, _ => 0.90 };

    Some(MevAlert {
        kind:   MevKind::Arbitrage,
        confidence,
        detail: format!("Cyclic route across {pool_count} pools — likely atomic arbitrage"),
    })
}

async fn detect_sandwich(
    tx_hash:      TxHash,
    block_number: u64,
    rpc_url:      &str,
) -> eyre::Result<Option<MevAlert>> {
    let ctx = match fetch_block_context(rpc_url, block_number, tx_hash).await? {
        Some(c) => c, None => return Ok(None),
    };

    let target_idx = ctx.target_index;
    let txs = &ctx.all_txs;
    let window = 5usize;

    let before_txs: Vec<&BlockTx> = txs[target_idx.saturating_sub(window)..target_idx].iter().collect();
    let after_txs:  Vec<&BlockTx> = txs[(target_idx+1)..(target_idx+1+window).min(txs.len())].iter().collect();

    let target_logs   = fetch_transfer_logs(rpc_url, tx_hash).await?;
    let target_tokens = token_set_from_logs(&target_logs);
    if target_tokens.is_empty() { return Ok(None); }

    for before_tx in &before_txs {
        for after_tx in &after_txs {
            if before_tx.from != after_tx.from { continue; }
            if let Some(victim) = txs.get(target_idx) {
                if before_tx.from == victim.from { continue; }
            }

            let before_logs = fetch_transfer_logs(rpc_url, before_tx.hash).await?;
            let after_logs  = fetch_transfer_logs(rpc_url, after_tx.hash).await?;
            let before_tokens = token_set_from_logs(&before_logs);
            let after_tokens  = token_set_from_logs(&after_logs);

            let overlap: HashSet<_> = target_tokens.intersection(&before_tokens)
                .filter(|t| after_tokens.contains(*t)).collect();

            if !overlap.is_empty() {
                let attacker    = before_tx.from;
                let gap_before  = target_idx - txs.iter().position(|t| t.hash == before_tx.hash).unwrap_or(0);
                let gap_after   = txs.iter().position(|t| t.hash == after_tx.hash).unwrap_or(0) - target_idx;
                let confidence  = match (gap_before, gap_after) {
                    (1, 1) => 0.92, (1, _) | (_, 1) => 0.78, _ => 0.60,
                };
                return Ok(Some(MevAlert {
                    kind: MevKind::SandwichAttack,
                    confidence,
                    detail: format!("Attacker {attacker} front-ran at -{gap_before}, back-ran at +{gap_after}"),
                }));
            }
        }
    }
    Ok(None)
}
