pub mod opcode;

use opcode::lookup;

/// A single decoded instruction in the bytecode stream.
#[derive(Debug, Clone)]
pub struct Instruction {
    /// Byte offset of the opcode inside the bytecode.
    pub offset: usize,
    /// The raw opcode byte.
    pub op: u8,
    /// Human-readable mnemonic (e.g. "PUSH1", "ADD").
    pub mnemonic: &'static str,
    /// The immediate operand bytes that follow the opcode (empty for most ops).
    pub operand: Vec<u8>,
    /// One-line description.
    pub description: &'static str,
    /// Semantic category.
    pub category: opcode::OpCategory,
}

impl Instruction {
    /// Format the operand as a `0x`-prefixed hex string, or empty if none.
    pub fn operand_hex(&self) -> String {
        if self.operand.is_empty() {
            String::new()
        } else {
            format!(
                "0x{}",
                self.operand
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect::<String>()
            )
        }
    }
}

/// Disassemble a slice of raw EVM bytecode.
///
/// PUSH<N> immediates are consumed correctly so the stream never de-syncs.
/// Truncated trailing immediates are included with however many bytes remain.
pub fn disassemble(bytecode: &[u8]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut pc = 0usize;

    while pc < bytecode.len() {
        let op = bytecode[pc];
        let info = lookup(op);
        let operand_end = (pc + 1 + info.operand_size).min(bytecode.len());
        let operand = bytecode[pc + 1..operand_end].to_vec();

        instructions.push(Instruction {
            offset: pc,
            op,
            mnemonic: info.mnemonic,
            operand,
            description: info.description,
            category: info.category,
        });

        pc += 1 + info.operand_size;
    }

    instructions
}

/// Convert a hex string (`0x`-prefixed or bare) to raw bytes.
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.trim().trim_start_matches("0x");
    if hex.len() % 2 != 0 {
        return Err(format!("odd-length hex string ({} chars)", hex.len()));
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| format!("invalid hex at offset {i}: {e}"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_simple_sequence() {
        // PUSH1 0x60, PUSH1 0x40, MSTORE  (classic Solidity preamble)
        let bytes = hex_to_bytes("6060604052").unwrap();
        let instrs = disassemble(&bytes);

        assert_eq!(instrs.len(), 3);

        assert_eq!(instrs[0].mnemonic, "PUSH1");
        assert_eq!(instrs[0].operand, vec![0x60]);
        assert_eq!(instrs[0].offset, 0);

        assert_eq!(instrs[1].mnemonic, "PUSH1");
        assert_eq!(instrs[1].operand, vec![0x40]);
        assert_eq!(instrs[1].offset, 2);

        assert_eq!(instrs[2].mnemonic, "MSTORE");
        assert!(instrs[2].operand.is_empty());
        assert_eq!(instrs[2].offset, 4);
    }

    #[test]
    fn truncated_push_does_not_panic() {
        // PUSH4 but only 2 bytes of immediate data follow
        let bytes = [0x63u8, 0xde, 0xad];
        let instrs = disassemble(&bytes);
        assert_eq!(instrs.len(), 1);
        assert_eq!(instrs[0].mnemonic, "PUSH4");
        assert_eq!(instrs[0].operand, vec![0xde, 0xad]);
    }

    #[test]
    fn push32_consumes_exactly_32_bytes() {
        let mut raw = vec![0x7fu8]; // PUSH32
        raw.extend_from_slice(&[0xabu8; 32]);
        raw.push(0x01); // ADD immediately after
        let instrs = disassemble(&raw);
        assert_eq!(instrs.len(), 2);
        assert_eq!(instrs[0].operand.len(), 32);
        assert_eq!(instrs[1].mnemonic, "ADD");
        assert_eq!(instrs[1].offset, 33);
    }

    #[test]
    fn operand_hex_formatting() {
        let bytes = hex_to_bytes("61dead").unwrap(); // PUSH2 0xdead
        let instrs = disassemble(&bytes);
        assert_eq!(instrs[0].operand_hex(), "0xdead");
    }

    #[test]
    fn empty_bytecode_gives_empty_vec() {
        assert!(disassemble(&[]).is_empty());
    }

    #[test]
    fn unknown_opcode_is_labelled() {
        let instrs = disassemble(&[0x0cu8]); // gaps in the opcode table
        assert_eq!(instrs[0].mnemonic, "UNKNOWN");
    }

    #[test]
    fn hex_to_bytes_strips_0x_prefix() {
        assert_eq!(
            hex_to_bytes("0xdeadbeef").unwrap(),
            vec![0xde, 0xad, 0xbe, 0xef]
        );
        assert_eq!(
            hex_to_bytes("deadbeef").unwrap(),
            vec![0xde, 0xad, 0xbe, 0xef]
        );
    }

    #[test]
    fn hex_to_bytes_rejects_odd_length() {
        assert!(hex_to_bytes("abc").is_err());
    }
}