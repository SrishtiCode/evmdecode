use alloy::primitives::{address, Address, Bytes, B256};

pub const V2_SWAP_EXACT_TOKENS_FOR_TOKENS: [u8; 4] = [0x38, 0xed, 0x17, 0x39];
pub const V2_SWAP_TOKENS_FOR_EXACT_TOKENS: [u8; 4] = [0x88, 0x03, 0xdb, 0xee];
pub const V2_SWAP_EXACT_ETH_FOR_TOKENS: [u8; 4]    = [0x7f, 0xf3, 0x6a, 0xb5];
pub const V2_SWAP_TOKENS_FOR_EXACT_ETH: [u8; 4]    = [0x4a, 0x25, 0xd9, 0x4a];
pub const V2_SWAP_EXACT_TOKENS_FOR_ETH: [u8; 4]    = [0x18, 0xcb, 0xaf, 0xe5];
pub const V3_EXACT_INPUT_SINGLE: [u8; 4]  = [0x41, 0x4b, 0xf3, 0x89];
pub const V3_EXACT_INPUT: [u8; 4]         = [0xc0, 0x4b, 0x8d, 0x59];
pub const V3_EXACT_OUTPUT_SINGLE: [u8; 4] = [0xdb, 0x3e, 0x21, 0x98];
pub const V3_EXACT_OUTPUT: [u8; 4]        = [0xf2, 0x8c, 0x01, 0x66];
pub const V3_MULTICALL: [u8; 4]           = [0xac, 0x96, 0x50, 0xd8];
pub const V3_MULTICALL_DEADLINE: [u8; 4]  = [0x5a, 0xe4, 0x01, 0xdc];

pub const UNISWAP_V2_ROUTER: Address      = address!("7a250d5630b4cf539739df2c5dacb4c659f2488d");
pub const UNISWAP_V3_ROUTER: Address      = address!("e592427a0aece92de3edee1f18e0157c05861564");
pub const UNISWAP_V3_ROUTER2: Address     = address!("68b3465833fb72a70ecdf485e0e4c7bd8665fc45");
pub const UNISWAP_UNIVERSAL_ROUTER: Address  = address!("ef1c6e67703c7bd7107eed8303fbe6ec2554bf6b");
pub const UNISWAP_UNIVERSAL_ROUTER2: Address = address!("3fc91a3afd70395cd496c647d5a6cc9d4b2b7fad");
pub const ONEINCH_V5: Address = address!("1111111254eeb25477b68fb85ed929f73a960582");
pub const ONEINCH_V6: Address = address!("111111125421ca6dc452d289314280a0f8842a65");
pub const PARASWAP: Address   = address!("def171fe48cf0115b1d80b88dc8eab59176fee57");

pub const V2_SWAP_TOPIC: B256 = B256::new([
    0xd7,0x8a,0xd9,0x5f,0xa4,0x6c,0x99,0x4b,0x65,0x51,0xd0,0xda,0x85,0xfc,0x27,0x5f,
    0xe6,0x13,0xce,0x37,0x65,0x7f,0xb8,0xd5,0xe3,0xd1,0x30,0x84,0x01,0x59,0xd8,0x22,
]);
pub const V3_SWAP_TOPIC: B256 = B256::new([
    0xc4,0x20,0x79,0xf9,0x4a,0x63,0x50,0xd7,0xe6,0x23,0x5f,0x29,0x17,0x49,0x24,0xf9,
    0x28,0xcc,0x2a,0xc8,0x18,0xeb,0x64,0xfe,0xd8,0x00,0x4e,0x11,0x5f,0xbc,0xca,0x67,
]);
pub const ERC20_TRANSFER_TOPIC: B256 = B256::new([
    0xdd,0xf2,0x52,0xad,0x1b,0xe2,0xc8,0x9b,0x69,0xc2,0xb0,0x68,0xfc,0x37,0x8d,0xaa,
    0x95,0x2b,0xa7,0xf1,0x63,0xc4,0xa1,0x16,0x28,0xf5,0x5a,0x4d,0xf5,0x23,0xb3,0xef,
]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapVersion { V2, V3 }

pub fn classify_selector(sel: &[u8; 4]) -> Option<SwapVersion> {
    match sel {
        s if *s == V2_SWAP_EXACT_TOKENS_FOR_TOKENS => Some(SwapVersion::V2),
        s if *s == V2_SWAP_TOKENS_FOR_EXACT_TOKENS => Some(SwapVersion::V2),
        s if *s == V2_SWAP_EXACT_ETH_FOR_TOKENS    => Some(SwapVersion::V2),
        s if *s == V2_SWAP_TOKENS_FOR_EXACT_ETH    => Some(SwapVersion::V2),
        s if *s == V2_SWAP_EXACT_TOKENS_FOR_ETH    => Some(SwapVersion::V2),
        s if *s == V3_EXACT_INPUT_SINGLE            => Some(SwapVersion::V3),
        s if *s == V3_EXACT_INPUT                   => Some(SwapVersion::V3),
        s if *s == V3_EXACT_OUTPUT_SINGLE           => Some(SwapVersion::V3),
        s if *s == V3_EXACT_OUTPUT                  => Some(SwapVersion::V3),
        s if *s == V3_MULTICALL                     => Some(SwapVersion::V3),
        s if *s == V3_MULTICALL_DEADLINE            => Some(SwapVersion::V3),
        _ => None,
    }
}

pub fn is_known_router(addr: &Address) -> bool {
    matches!(addr,
        &UNISWAP_V2_ROUTER | &UNISWAP_V3_ROUTER | &UNISWAP_V3_ROUTER2
        | &UNISWAP_UNIVERSAL_ROUTER | &UNISWAP_UNIVERSAL_ROUTER2
        | &ONEINCH_V5 | &ONEINCH_V6 | &PARASWAP
    )
}

#[derive(Debug, Clone)]
pub struct RawLog {
    pub address: Address,
    pub topics:  Vec<B256>,
    pub data:    Bytes,
}

#[derive(Debug, Clone)]
pub struct SwapLog {
    pub pool:    Address,
    pub version: SwapVersion,
    pub amount0: i128,
    pub amount1: i128,
}

pub fn parse_swap_logs(logs: &[RawLog]) -> Vec<SwapLog> {
    let mut out = Vec::new();
    for log in logs {
        let Some(topic0) = log.topics.first() else { continue };
        if *topic0 == V2_SWAP_TOPIC && log.data.len() >= 128 {
            let a0in  = read_u128(&log.data[0..32]);
            let a1in  = read_u128(&log.data[32..64]);
            let a0out = read_u128(&log.data[64..96]);
            let a1out = read_u128(&log.data[96..128]);
            out.push(SwapLog { pool: log.address, version: SwapVersion::V2,
                amount0: a0in as i128 - a0out as i128,
                amount1: a1in as i128 - a1out as i128 });
        } else if *topic0 == V3_SWAP_TOPIC && log.data.len() >= 128 {
            out.push(SwapLog { pool: log.address, version: SwapVersion::V3,
                amount0: read_i128(&log.data[0..32]),
                amount1: read_i128(&log.data[32..64]) });
        }
    }
    out
}

fn read_u128(b: &[u8]) -> u128 {
    let s = &b[b.len().saturating_sub(16)..];
    let mut arr = [0u8; 16];
    arr[16 - s.len()..].copy_from_slice(s);
    u128::from_be_bytes(arr)
}

fn read_i128(b: &[u8]) -> i128 {
    let neg = b[0] & 0x80 != 0;
    let s = &b[b.len().saturating_sub(16)..];
    let mut arr = [if neg { 0xff } else { 0x00 }; 16];
    arr[16 - s.len()..].copy_from_slice(s);
    i128::from_be_bytes(arr)
}
