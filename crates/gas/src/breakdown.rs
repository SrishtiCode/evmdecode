use evmdecode_core::types::GasReport;
use super::intrinsic::compute_intrinsic;

pub fn compute_breakdown(
    gas_used:  u128,
    calldata:  &[u8],
    log_count: usize,
) -> GasReport {
    let cd = compute_intrinsic(calldata);

    // EIP-2028: each log costs 375 base + 375 per topic + 8 per data byte
    // for a standard Transfer log (3 topics, 32 bytes data):
    // 375 + (3 * 375) + (32 * 8) = 375 + 1125 + 256 = 1756
    // we estimate generously at 2000 per log
    let log_gas     = log_count as u128 * 2_000;

    // storage: remaining after intrinsic and logs
    // real breakdown needs debug_traceTransaction; we estimate here
    let execution_estimate = gas_used
        .saturating_sub(cd.intrinsic_gas)
        .saturating_sub(log_gas);

    GasReport {
        gas_used,
        intrinsic:    cd.intrinsic_gas,
        calldata_cost: cd.calldata_gas,
        storage_cost:  log_gas,
        execution_cost: execution_estimate,
    }
}
