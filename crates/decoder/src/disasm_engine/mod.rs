pub mod opcode;

use opcode::lookup;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub offset:      usize,
    pub op:          u8,
    pub mnemonic:    &'static str,
    pub operand:     Vec<u8>,
    pub description: &'static str,
    pub category:    opcode::OpCategory,
}

impl Instruction {
    pub fn operand_hex(&self) -> String {
        if self.operand.is_empty() {
            String::new()
        } else {
            format!("0x{}", self.operand.iter().map(|b| format!("{b:02x}")).collect::<String>())
        }
    }

    pub fn is_jump(&self)     -> bool { matches!(self.op, 0x56 | 0x57) }
    pub fn is_jumpdest(&self) -> bool { self.op == 0x5b }
    pub fn is_push(&self)     -> bool { matches!(self.op, 0x60..=0x7f) }
    pub fn is_storage(&self)  -> bool { matches!(self.op, 0x54 | 0x55) } // SLOAD SSTORE
}

pub fn disassemble(bytecode: &[u8]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut pc = 0usize;

    while pc < bytecode.len() {
        let op   = bytecode[pc];
        let info = lookup(op);
        let end  = (pc + 1 + info.operand_size).min(bytecode.len());
        let operand = bytecode[pc + 1..end].to_vec();

        instructions.push(Instruction {
            offset: pc,
            op,
            mnemonic:    info.mnemonic,
            operand,
            description: info.description,
            category:    info.category,
        });

        pc += 1 + info.operand_size;
    }

    instructions
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.trim().trim_start_matches("0x");
    if !hex.len().is_multiple_of(2) {
        return Err(format!("odd-length hex string ({} chars)", hex.len()));
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i+2], 16)
            .map_err(|e| format!("invalid hex at {i}: {e}")))
        .collect()
}
