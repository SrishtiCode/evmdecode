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

    // decode calldata
    if calldata.len() >= 4 {
        let selector = format!("0x{}", hex::encode(&calldata[..4]));
        if let Some(sig) = lookup_4byte(&selector).await? {
            if let Ok(decoded) = decode_calldata(&calldata, &sig) {
                print_decoded_call(&decoded);
            }
        }
    }

    // state diff
    println!("\n{}", "Fetching state diff via debug_traceTransaction...".dimmed());
    let log_count = match trace_tx(&provider, tx).await {
        Ok(trace) => {
            let diffs = parse_state_diff(&trace)?;
            let count = diffs.len();
            print_state_diffs(&diffs);
            count
        }
        Err(_) => {
            println!("  {} debug_ unavailable, showing receipt + logs", "warn:".yellow());
            let tx_hash: TxHash = tx.parse()?;
            let mut count = 0;
            if let Some(receipt) = provider.get_transaction_receipt(tx_hash).await? {
                println!("\n{}", "Receipt".bold().cyan());
                println!("  status   : {}", if receipt.status() {
                    "success".green().to_string()
                } else {
                    "reverted".red().to_string()
                });
                println!("  gas used : {}", receipt.gas_used.to_string().yellow());

                count = receipt.inner.logs().len();
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
            count
        }
    };

    // gas breakdown — works regardless of debug_ availability
    let cd      = compute_intrinsic(&calldata);
    let report  = compute_breakdown(raw.gas, &calldata, log_count);
    print_gas_report(&report, cd.zero_bytes, cd.non_zero_bytes);

    Ok(())
}
