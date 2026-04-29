/// EIP-2028 calldata costs
const ZERO_BYTE_COST:     u128 = 4;
const NON_ZERO_BYTE_COST: u128 = 16;
const BASE_GAS:           u128 = 21_000;

pub struct CalldataCost {
    pub zero_bytes:     usize,
    pub non_zero_bytes: usize,
    pub calldata_gas:   u128,
    pub intrinsic_gas:  u128,  // base + calldata
}

pub fn compute_intrinsic(calldata: &[u8]) -> CalldataCost {
    let mut zero     = 0usize;
    let mut non_zero = 0usize;

    for &byte in calldata {
        if byte == 0 { zero += 1; } else { non_zero += 1; }
    }

    let calldata_gas  = (zero as u128 * ZERO_BYTE_COST)
                      + (non_zero as u128 * NON_ZERO_BYTE_COST);
    let intrinsic_gas = BASE_GAS + calldata_gas;

    CalldataCost { zero_bytes: zero, non_zero_bytes: non_zero, calldata_gas, intrinsic_gas }
}
