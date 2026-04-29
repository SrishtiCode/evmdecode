use colored::Colorize;
use alloy::primitives::{TxHash, Address};
use alloy::providers::Provider;
use evmdecode_rpc::{provider::build_provider, fetcher::fetch_tx};
use evmdecode_mev::{detect, MevKind};
use evmdecode_mev::decode::{parse_swap_logs, RawLog};

pub async fn run(tx: &str, rpc: &str) -> anyhow::Result<()> {
    let provider  = build_provider(rpc)?;
    let raw       = fetch_tx(&provider, tx).await?;

    println!("tx    : {}", raw.hash);
    println!("from  : {}", raw.from);
    println!("to    : {}", raw.to.as_deref().unwrap_or("none"));
    println!("block : {}", raw.block_number.unwrap_or(0));

    let block_number = raw.block_number.unwrap_or(0);
    let tx_hash: TxHash = tx.parse()?;
    let to: Option<Address> = raw.to.as_deref().and_then(|s| s.parse().ok());

    // parse selector
    let calldata = hex::decode(&raw.input)?;
    let selector: Option<[u8; 4]> = if calldata.len() >= 4 {
        let mut s = [0u8; 4];
        s.copy_from_slice(&calldata[..4]);
        Some(s)
    } else { None };

    // fetch receipt logs
    println!("\n{}", "Fetching receipt logs...".dimmed());
    let receipt = provider.get_transaction_receipt(tx_hash).await?;
    let (raw_logs, swap_logs) = match receipt {
        None => {
            println!("  receipt not found");
            (vec![], vec![])
        }
        Some(r) => {
            println!("  {} logs found", r.inner.logs().len());
            let raw: Vec<RawLog> = r.inner.logs().iter().map(|log| RawLog {
                address: log.address(),
                topics:  log.topics().to_vec(),
                data:    log.data().data.clone(),
            }).collect();
            let swaps = parse_swap_logs(&raw);
            (raw, swaps)
        }
    };

    println!("  {} swap events parsed", swap_logs.len());

    // run all detectors
    println!("\n{}", "Running MEV detectors...".dimmed());
    let alerts = detect(
        swap_logs, raw_logs, selector, to,
        block_number, tx_hash, rpc,
    ).await;

    // print results
    println!("\n{}", "MEV Analysis".bold().cyan());
    if alerts.is_empty() {
        println!("  {} no MEV patterns detected", "✓".green());
        println!("  this looks like a plain transfer or non-swap interaction");
        return Ok(());
    }

    for alert in &alerts {
        let kind_str = match alert.kind {
            MevKind::Swap           => alert.kind.to_string().blue().to_string(),
            MevKind::SandwichAttack => alert.kind.to_string().red().bold().to_string(),
            MevKind::Arbitrage      => alert.kind.to_string().yellow().to_string(),
        };

        let confidence_pct = (alert.confidence * 100.0) as u32;
        let bar_len = (alert.confidence * 20.0) as usize;
        let bar = format!("{}{}", "█".repeat(bar_len), "░".repeat(20 - bar_len));

        println!("\n  {} [{}%]", kind_str, confidence_pct);
        println!("  confidence : {}", bar);
        println!("  detail     : {}", alert.detail);
    }

    Ok(())
}
