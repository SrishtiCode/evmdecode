evm-xray/
├── Cargo.toml                    ← workspace root
├── Cargo.lock
├── README.md
├── .env.example                  ← RPC_URL=https://...
├── .github/
│   └── workflows/
│       └── ci.yml               ← cargo test + clippy
│
├── crates/
│   ├── cli/                     ← binary entrypoint
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs          ← clap setup, dispatch to commands
│   │       └── commands/
│   │           ├── mod.rs
│   │           ├── decode.rs    ← `evm-xray decode --tx`
│   │           ├── simulate.rs  ← `evm-xray simulate --tx`
│   │           └── mev.rs       ← `evm-xray mev --tx`
│   │
│   ├── core/                    ← shared types, no business logic
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── types.rs         ← RawTx, DecodedCall, StateDiff, GasReport
│   │       └── error.rs         ← unified Error enum
│   │
│   ├── rpc/                     ← all network I/O
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── provider.rs      ← ProviderBuilder wrapper + retry
│   │       ├── fetcher.rs       ← eth_getTransaction, eth_getTransactionReceipt
│   │       └── tracer.rs        ← debug_traceTransaction, eth_call
│   │
│   ├── decoder/                 ← ABI decode logic
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── selector.rs      ← 4byte.directory HTTP lookup
│   │       ├── abi_decode.rs    ← DynSolType param decoding
│   │       └── custom_abi.rs    ← JsonAbi file loading
│   │
│   ├── simulator/               ← state diff + eth_call replay
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── state_diff.rs    ← parse prestate tracer output
│   │       └── replayer.rs      ← eth_call at historical block
│   │
│   ├── gas/                     ← gas breakdown math
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── intrinsic.rs     ← 21000 + calldata cost
│   │       └── breakdown.rs     ← storage + execution split
│   │
│   ├── mev/                     ← pattern detectors
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── detector.rs      ← MevDetector trait
│   │       ├── uniswap.rs       ← swap selector matching
│   │       ├── sandwich.rs      ← surrounding tx analysis
│   │       └── arb.rs           ← token flow loop detection
│   │
│   └── render/                  ← output formatting
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── pretty.rs        ← colored terminal tables
│           └── json.rs          ← serde_json output
│
└── tests/
    ├── fixtures/
    │   ├── uniswap_swap.json    ← saved tx for offline tests
    │   └── erc20_transfer.json
    └── integration_test.rs



cargo run -p cli -- decode <tx_hash>    

