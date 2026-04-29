use colored::Colorize;
use tabled::{Table, Tabled};
use evmdecode_core::types::{DecodedCall, GasReport, StateDiff};

#[derive(Tabled)]
struct ParamRow {
    #[tabled(rename = "type")]  kind:  String,
    #[tabled(rename = "value")] value: String,
}

pub fn print_decoded_call(call: &DecodedCall) {
    println!("\n{}", "Decoded Call".bold().cyan());
    println!("  selector : {}", call.selector.yellow());
    if let Some(sig) = &call.function_sig {
        println!("  function : {}", sig.green());
    }
    let rows: Vec<ParamRow> = call.params.iter().map(|p| ParamRow {
        kind:  p.kind.clone(),
        value: p.value.clone(),
    }).collect();
    println!("{}", Table::new(rows));
}

pub fn print_gas_report(report: &GasReport, zero_bytes: usize, non_zero_bytes: usize) {
    let pct = |n: u128| if report.gas_used > 0 {
        n as f64 / report.gas_used as f64 * 100.0
    } else { 0.0 };

    println!("\n{}", "Gas Breakdown".bold().cyan());
    println!("  total      : {}", report.gas_used.to_string().yellow());
    println!("  ├ intrinsic  : {} ({:.1}%)  [21000 base + calldata]",
        report.intrinsic, pct(report.intrinsic));
    println!("  ├ calldata   : {} ({:.1}%)  [{} zero × 4 + {} nonzero × 16]",
        report.calldata_cost, pct(report.calldata_cost), zero_bytes, non_zero_bytes);
    println!("  ├ logs/store : {} ({:.1}%)  [estimated]",
        report.storage_cost, pct(report.storage_cost));
    println!("  └ execution  : {} ({:.1}%)  [opcodes, memory]",
        report.execution_cost, pct(report.execution_cost));
}

#[derive(Tabled)]
struct DiffRow {
    #[tabled(rename = "address")] address: String,
    #[tabled(rename = "slot")]    slot:    String,
    #[tabled(rename = "before")]  before:  String,
    #[tabled(rename = "after")]   after:   String,
}

pub fn print_state_diffs(diffs: &[StateDiff]) {
    println!("\n{}", "State Diff".bold().cyan());
    if diffs.is_empty() {
        println!("  no state changes detected");
        return;
    }
    let mut rows: Vec<DiffRow> = Vec::new();
    for diff in diffs {
        if diff.balance_before != diff.balance_after {
            let before = diff.balance_before.clone().unwrap_or_else(|| "0x0".into());
            let after  = diff.balance_after.clone().unwrap_or_else(|| "0x0".into());
            rows.push(DiffRow {
                address: short_addr(&diff.address),
                slot:    "(balance)".to_string(),
                before:  hex_to_eth(&before),
                after:   hex_to_eth(&after),
            });
        }
        let mut slots: Vec<_> = diff.storage_changes.values().collect();
        slots.sort_by(|a, b| a.slot.cmp(&b.slot));
        for change in slots {
            rows.push(DiffRow {
                address: short_addr(&diff.address),
                slot:    short_slot(&change.slot),
                before:  change.before.clone(),
                after:   change.after.clone(),
            });
        }
    }
    println!("{}", Table::new(rows));
}

fn short_addr(addr: &str) -> String {
    if addr.len() > 10 {
        format!("{}..{}", &addr[..6], &addr[addr.len()-4..])
    } else { addr.to_string() }
}

fn short_slot(slot: &str) -> String {
    if slot.len() > 14 {
        format!("{}..{}", &slot[..8], &slot[slot.len()-4..])
    } else { slot.to_string() }
}

fn hex_to_eth(hex: &str) -> String {
    let hex = hex.trim_start_matches("0x");
    if let Ok(n) = u128::from_str_radix(hex, 16) {
        if n > 1_000_000_000_000_000u128 {
            return format!("{:.6} ETH", n as f64 / 1e18);
        }
        return format!("{} wei", n);
    }
    hex.to_string()
}
