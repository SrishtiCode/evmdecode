use colored::Colorize;
use evmdecode_rpc::{provider::build_provider, fetcher::{fetch_tx, fetch_bytecode}};
use evmdecode_decoder::disasm_engine::{disassemble, hex_to_bytes, opcode::OpCategory};
use evmdecode_decoder::selector::lookup_4byte;

pub async fn run(
    tx:              Option<String>,
    address:         Option<String>,
    hex_input:       Option<String>,
    rpc:             Option<String>,
    filter:          Option<String>,
    jumptable:       bool,
    functions:       bool,
    storage_slots:   bool,
    decode_metadata: bool,
) -> anyhow::Result<()> {

    // ── resolve bytecode ────────────────────────────────────────────────
    let bytecode: Vec<u8> = if let Some(hex) = hex_input {
        hex_to_bytes(&hex).map_err(|e| anyhow::anyhow!(e))?

    } else if let Some(addr) = &address {
        let rpc_url = rpc.as_deref().ok_or_else(|| anyhow::anyhow!("--rpc required with --address"))?;
        let provider = build_provider(rpc_url)?;
        fetch_bytecode(&provider, addr).await?

    } else if let Some(tx_hash) = &tx {
        let rpc_url = rpc.as_deref().ok_or_else(|| anyhow::anyhow!("--rpc required with --tx"))?;
        let provider = build_provider(rpc_url)?;
        let raw = fetch_tx(&provider, tx_hash).await?;
        // use the 'to' address to fetch contract bytecode
        let contract = raw.to.ok_or_else(|| anyhow::anyhow!("tx has no 'to' — contract deployment tx"))?;
        fetch_bytecode(&provider, &contract).await?

    } else {
        anyhow::bail!("provide one of --tx, --address, or --hex");
    };

    println!("{} {} bytes", "Bytecode:".bold().cyan(), bytecode.len());

    let instructions = disassemble(&bytecode);

    // ── collect metadata ────────────────────────────────────────────────
    let jump_dests: Vec<usize> = instructions.iter()
        .filter(|i| i.is_jumpdest())
        .map(|i| i.offset)
        .collect();

    // PUSH4 values that look like function selectors (4 bytes)
    let mut selectors: Vec<String> = instructions.iter()
        .filter(|i| i.op == 0x63 && i.operand.len() == 4)
        .map(|i| hex::encode(&i.operand))
        .collect();
    selectors.dedup();

    let storage_ops: Vec<&evmdecode_decoder::disasm_engine::Instruction> = instructions.iter()
        .filter(|i| i.is_storage())
        .collect();

    // ── metadata block ──────────────────────────────────────────────────
    if decode_metadata {
        println!("\n{}", "Metadata".bold().cyan());
        println!("  total instructions : {}", instructions.len());
        println!("  jump destinations  : {}", jump_dests.len());
        println!("  storage ops        : {} SLOAD / {} SSTORE",
            storage_ops.iter().filter(|i| i.op == 0x54).count(),
            storage_ops.iter().filter(|i| i.op == 0x55).count(),
        );

        // opcode category breakdown
        let mut counts = std::collections::HashMap::new();
        for instr in &instructions {
            *counts.entry(format!("{:?}", instr.category)).or_insert(0u32) += 1;
        }
        let mut counts: Vec<_> = counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        println!("\n{}", "Opcode Categories".bold().cyan());
        for (cat, n) in &counts {
            println!("  {:12} : {}", cat, n);
        }
    }

    // ── function selector table ─────────────────────────────────────────
    if functions || !selectors.is_empty() {
        println!("\n{}", "Function Selectors (PUSH4)".bold().cyan());
        if selectors.is_empty() {
            println!("  none found");
        } else {
            for sel in &selectors {
                let full_sel = format!("0x{}", sel);
                match lookup_4byte(&full_sel).await {
                    Ok(Some(sig)) => println!("  0x{} → {}", sel.yellow(), sig.green()),
                    _             => println!("  0x{} → {}", sel.yellow(), "unknown".dimmed()),
                }
            }
        }
    }

    // ── jump table ──────────────────────────────────────────────────────
    if jumptable {
        println!("\n{}", "Jump Destinations".bold().cyan());
        if jump_dests.is_empty() {
            println!("  none");
        } else {
            for dest in &jump_dests {
                println!("  offset 0x{:04x} ({})", dest, dest);
            }
        }
    }

    // ── storage slot analysis ───────────────────────────────────────────
    if storage_slots {
        println!("\n{}", "Storage Operations".bold().cyan());
        for instr in &storage_ops {
            let op_name = if instr.op == 0x54 { "SLOAD ".green() } else { "SSTORE".red() };
            println!("  offset 0x{:04x}  {}", instr.offset, op_name);
        }
    }

    // ── disassembly listing ─────────────────────────────────────────────
    let filter_str = filter.as_deref().unwrap_or("").to_uppercase();

    println!("\n{}", "Disassembly".bold().cyan());
    println!("{:>6}  {:>4}  {:<14} {:<20} {}",
        "OFFSET".dimmed(), "HEX".dimmed(), "MNEMONIC".dimmed(),
        "OPERAND".dimmed(), "DESCRIPTION".dimmed());
    println!("{}", "─".repeat(72).dimmed());

    for instr in &instructions {
        // apply --filter if set
        if !filter_str.is_empty() && !instr.mnemonic.contains(&filter_str) {
            continue;
        }

        let mnemonic_colored = match instr.category {
            OpCategory::Control   => instr.mnemonic.yellow().to_string(),
            OpCategory::Storage   => instr.mnemonic.red().to_string(),
            OpCategory::System    => instr.mnemonic.magenta().to_string(),
            OpCategory::Push      => instr.mnemonic.blue().to_string(),
            OpCategory::Arithmetic | OpCategory::Comparison | OpCategory::Bitwise
                                  => instr.mnemonic.white().to_string(),
            OpCategory::Invalid   => instr.mnemonic.bright_red().to_string(),
            _                     => instr.mnemonic.normal().to_string(),
        };

        let jumpdest_marker = if instr.is_jumpdest() { " ◀".cyan().to_string() } else { String::new() };

        println!("  {:04x}  {:02x}    {:<22} {:<20} {}{}",
            instr.offset,
            instr.op,
            mnemonic_colored,
            instr.operand_hex(),
            instr.description.dimmed(),
            jumpdest_marker,
        );
    }

    println!("\n{} {} instructions", "Total:".bold(), instructions.len());
    Ok(())
}
