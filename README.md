# evmdecode

> EVM transaction decoder, simulator, disassembler, and MEV analyzer — built in Rust.

A CLI tool that takes any Ethereum transaction hash or contract address and gives you a full breakdown — decoded ABI, gas analysis, bytecode disassembly, and MEV opportunity detection. Built with `alloy-rs`, `tokio`, and a clean multi-crate workspace architecture.

---

## Features

| Command | What it does |
|---------|-------------|
| `decode` | Resolve function selector via 4byte.directory + ABI decode all calldata params |
| `simulate` | Fetch receipt, show event logs with topics, compute gas breakdown |
| `disasm` | Full EVM bytecode disassembly with opcode categories, jump table, and function selectors |
| `mev` | Detect Uniswap V2/V3 swaps, sandwich attacks, and arbitrage with confidence scores |

---

## Demo

### Decode a USDC transfer
```bash
$ evmdecode decode --tx 0x412f5f3c... --rpc $RPC_URL

tx      : 0x412f5f3c2f50993e7736150699c747ad5f682e20a610a0d1d5fa9d2fae466a4c
from    : 0x946Aa581287709B59dB1e635DAF3c35408C20DEf
to      : 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48   ← USDC contract
selector: 0xa9059cbb
signature: transfer(address,uint256)

Decoded Call
  selector : a9059cbb
  function : transfer(address,uint256)
+---------+-----------------------------------------------------+
| type    | value                                               |
+---------+-----------------------------------------------------+
| address | Address(0x4e5ae324d39935169cf35721b1fb31ed65d69974) |
| uint256 | Uint(187208650578, 256)                             |  ← 187,208 USDC
+---------+-----------------------------------------------------+
```

### Simulate + gas breakdown
```bash
$ evmdecode simulate --tx 0x412f5f3c... --rpc $RPC_URL

Receipt
  status   : success
  gas used : 40372

Gas Breakdown
  total      : 40372
  ├ intrinsic  : 21620 (53.6%)  [21000 base + calldata]
  ├ calldata   : 620   (1.5%)   [39 zero × 4 + 29 nonzero × 16]
  ├ logs/store : 2000  (5.0%)   [estimated]
  └ execution  : 16752 (41.5%)  [opcodes, memory]
```

### Disassemble a contract
```bash
$ evmdecode disasm --address 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 \
    --rpc $RPC_URL --decode-metadata --functions

Bytecode: 2186 bytes

Metadata
  total instructions : 844
  jump destinations  : 69
  storage ops        : 2 SLOAD / 3 SSTORE

Opcode Categories
  Push    : 211    Control  : 137
  Dup     : 121    Swap     : 72
  Storage : 5      System   : 14

Function Selectors (PUSH4)
  0x3659cfe6 → upgradeTo(address)
  0x4f1ef286 → upgradeToAndCall(address,bytes)
  0x5c60da1b → implementation()
  0x8f283970 → changeAdmin(address)
  0xf851a440 → admin()
```

### MEV analysis
```bash
$ evmdecode mev --tx 0x42f750... --rpc $RPC_URL

MEV Analysis

  Swap [98%]
  confidence : ███████████████████░
  detail     : Uniswap V2 swap across 1 pool(s)
```

---

## Installation

### Prerequisites
- Rust 1.75+ (`rustup update stable`)
- An Ethereum RPC endpoint ([Alchemy](https://www.alchemy.com), [Infura](https://infura.io), or local node)

### Build from source
```bash
git clone https://github.com/YOUR_USERNAME/evmdecode
cd evmdecode
cargo build --release
```

The binary will be at `target/release/evmdecode`.

---

## Usage

```bash
export RPC_URL="https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY"

# Decode calldata from a tx hash
evmdecode decode --tx <TX_HASH> --rpc $RPC_URL

# Simulate + event logs + gas breakdown
evmdecode simulate --tx <TX_HASH> --rpc $RPC_URL

# Disassemble a contract by address
evmdecode disasm --address <CONTRACT_ADDRESS> --rpc $RPC_URL \
  --decode-metadata \   # show opcode category breakdown
  --functions \         # resolve PUSH4 selectors via 4byte.directory
  --jumptable \         # list all JUMPDEST offsets
  --storage-slots       # list all SLOAD/SSTORE positions

# Disassemble from a tx hash (fetches the contract at tx.to)
evmdecode disasm --tx <TX_HASH> --rpc $RPC_URL --functions

# Disassemble raw hex bytecode (no RPC needed)
evmdecode disasm --hex 0x6080604052... --decode-metadata

# Filter disassembly to specific opcodes
evmdecode disasm --address <ADDR> --rpc $RPC_URL --filter JUMP

# MEV analysis
evmdecode mev --tx <TX_HASH> --rpc $RPC_URL
```

---

## Architecture

Eight-crate Rust workspace — each crate has one responsibility:

```
evmdecode/
├── crates/
│   ├── core/        # shared types: RawTx, DecodedCall, StateDiff, GasReport, MevAlert
│   ├── rpc/         # async RPC provider, tx fetcher, bytecode fetcher, trace caller
│   ├── decoder/     # 4byte selector lookup, ABI decode via alloy::dyn_abi, disassembler
│   ├── simulator/   # state diff parsing (debug_traceTransaction), eth_call replay
│   ├── gas/         # intrinsic gas (EIP-2028), calldata cost, execution breakdown
│   ├── mev/         # swap/sandwich/arb detectors, Uniswap V2/V3 log parsing
│   ├── render/      # colored terminal output, ASCII tables via tabled
│   └── cli/         # clap subcommands: decode, simulate, disasm, mev
```

Dependency flow (no circular deps):

```
cli → [rpc, decoder, simulator, gas, mev, render] → core
```

---

## MEV Detection

Three detectors run in sequence on every `mev` call:

**Swap detector** — matches calldata selector against known Uniswap V2/V3 function signatures and checks if `tx.to` is a known router address. Confirmed by Swap event logs in the receipt. Confidence 50–98%.

**Arbitrage detector** — looks for ≥2 unique pools in the same tx's Swap logs with a cyclic token flow (amount sent to one pool, received from another). Confidence 72–90% depending on pool count.

**Sandwich detector** — fetches all txs in the same block, searches a ±5 tx window around the target for two txs from the same address that touch the same token. No `debug_` namespace required — uses `eth_getBlockByNumber` + `eth_getTransactionReceipt`. Confidence 60–92% based on adjacency.

Supported protocols:

| Protocol | Detection method |
|----------|-----------------|
| Uniswap V2 | Selector + Swap event topic |
| Uniswap V3 | Selector + Swap event topic |
| Uniswap Universal Router | Address match |
| 1inch V5/V6 | Address match |
| Paraswap | Address match |

---

## RPC Requirements

| Command | Required RPC methods |
|---------|---------------------|
| `decode` | `eth_getTransactionByHash` |
| `simulate` | `eth_getTransactionByHash`, `eth_getTransactionReceipt` |
| `simulate` (full state diff) | `debug_traceTransaction` — needs Alchemy Growth or archive node |
| `disasm` | `eth_getCode` |
| `mev` | `eth_getTransactionByHash`, `eth_getTransactionReceipt`, `eth_getBlockByNumber` |

Free-tier Alchemy keys work for all commands except full state diff.

---

## Tech Stack

| Crate | Purpose |
|-------|---------|
| [`alloy`](https://alloy.rs) | Ethereum provider, ABI encoding, primitive types |
| [`tokio`](https://tokio.rs) | Async runtime |
| [`clap`](https://docs.rs/clap) | CLI argument parsing |
| [`reqwest`](https://docs.rs/reqwest) | HTTP client for RPC + 4byte.directory |
| [`tabled`](https://docs.rs/tabled) | ASCII table rendering |
| [`colored`](https://docs.rs/colored) | Terminal color output |
| [`serde`](https://serde.rs) | JSON serialization |

---

## What I Learned Building This

- **alloy-rs vs ethers-rs** — alloy 0.3 uses concrete types (`RootProvider<Http<Client>>`) rather than trait objects for providers; `impl Provider` doesn't work as a return type due to type parameter ambiguity
- **EVM gas model** — intrinsic gas is 21,000 base + 4 per zero calldata byte + 16 per non-zero byte (EIP-2028); storage writes (SSTORE) dominate execution cost
- **4byte.directory** — selector collisions are common; the oldest registration (`ordering=id ASC`) is canonical; a hardcoded table of the top ~15 selectors prevents most false positives
- **Proxy patterns** — the USDC contract is an EIP-1967 transparent proxy; the implementation address lives at storage slot `keccak256("eip1967.proxy.implementation") - 1`
- **MEV mechanics** — sandwich detection doesn't require archive node access; `eth_getBlockByNumber` + transfer log comparison is sufficient to identify same-attacker front/back-run pairs

---

## License

MIT
