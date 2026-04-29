use colored::Colorize;
use alloy::providers::Provider;
use alloy::primitives::TxHash;
use evmdecode_rpc::{provider::build_provider, fetcher::fetch_tx, tracer::trace_tx};
use evmdecode_simulator::state_diff::parse_state_diff;
use evmdecode_render::pretty::{print_decoded_call, print_state_diffs, print_gas_report};
use evmdecode_decoder::{selector::lookup_4byte, abi_decode::decode_calldata};
use evmdecode_gas::{intrinsic::compute_intrinsic, breakdown::compute_breakdown};

pub async fn run(tx: &str, rpc: &str) -> anyhow::Result<()> {
    let provider = build_provider(rpc)?;
    let raw      = fetch_tx(&provider, tx).await?;

    println!("tx      : {}", raw.hash);
    println!("from    : {}", raw.from);
    println!("to      : {}", raw.to.as_deref().unwrap_or("contract creation"));
    println!("block   : {}", raw.block_number.unwrap_or(0));

    let calldata = hex::decode(&raw.input)?;

    if calldata.len() >= 4 {
        let selector = format!("0x{}", hex::encode(&calldata[..4]));
        if let Some(sig) = lookup_4byte(&selector).await? {
            if let Ok(decoded) = decode_calldata(&calldata, &sig) {
                print_decoded_call(&decoded);
            }
        }
    }

    println!("\n{}", "Fetching state diff via debug_traceTransaction...".dimmed());

    let mut gas_used  = raw.gas; // fallback to gas limit
    let mut log_count = 0usize;

    match trace_tx(&provider, tx).await {
        Ok(trace) => {
            let diffs = parse_state_diff(&trace)?;
            log_count = diffs.len();
            print_state_diffs(&diffs);
        }
        Err(_) => {
            println!("  {} debug_ unavailable, showing receipt + logs", "warn:".yellow());
            let tx_hash: TxHash = tx.parse()?;
            if let Some(receipt) = provider.get_transaction_receipt(tx_hash).await? {
                // use actual gas used, not gas limit
                gas_used  = receipt.gas_used;
                log_count = receipt.inner.logs().len();

                println!("\n{}", "Receipt".bold().cyan());
                println!("  status   : {}", if receipt.status() {
                    "success".green().to_string()
                } else {
                    "reverted".red().to_string()
                });
                println!("  gas used : {}", receipt.gas_used.to_string().yellow());

                println!("\n{}", "Event Logs".bold().cyan());
                for (i, log) in receipt.inner.logs().iter().enumerate() {
                    println!("  [{}] address : {}", i, log.address());
                    for (j, topic) in log.topics().iter().enumerate() {
                        let label = match j {
                            0 => "event sig",
                            1 => "topic[1] ",
                            2 => "topic[2] ",
                            _ => "topic    ",
                        };
                        println!("      {} : {:?}", label, topic);
                    }
                    if !log.data().data.is_empty() {
                        println!("      data     : 0x{}", hex::encode(&log.data().data));
                    }
                    println!();
                }
            }
        }
    }

    // gas breakdown uses actual gas_used from receipt
    let cd     = compute_intrinsic(&calldata);
    let report = compute_breakdown(gas_used, &calldata, log_count);
    print_gas_report(&report, cd.zero_bytes, cd.non_zero_bytes);

    Ok(())
}
