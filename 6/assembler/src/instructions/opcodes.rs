use anyhow::Result;
use std::collections::HashMap;

use super::error::{InstructionError, InstructionErrorKind};
use super::{Instruction, OperatorKind, DestFlags, JumpFlags, CompFlags};

pub trait OpcodeGenerator {
    fn get_opcode(&self, symbols: &HashMap<&str, u16>) -> Result<Option<u16>>;
}

impl OpcodeGenerator for Instruction {
    fn get_opcode(&self, symbols: &HashMap<&str, u16>) -> Result<Option<u16>> {
        let opcode = match &self.operator {
            OperatorKind::Address { address } => Some(*address),
            OperatorKind::Symbol { symbol } => Some(lookup_symbol_address(symbols, symbol, self.line_number)?),
            OperatorKind::Comp { comp, dest, jump } => Some(get_comp_opcode(comp, dest, jump)),
            OperatorKind::Label { label: _ } => None,
        };

        Ok(opcode)
    }
}

fn lookup_symbol_address(symbols: &HashMap<&str, u16>, symbol: &str, line_number: u32) -> Result<u16> {
    Ok(*symbols.get(symbol).ok_or_else(
        || InstructionError {
            line_number,
            kind: InstructionErrorKind::UndefinedSymbol {
                symbol: symbol.to_owned(),
            },
        },
    )?)
}

fn get_comp_opcode(comp: &CompFlags, dest: &DestFlags, jump: &JumpFlags) -> u16 {
    // 111accccccdddjjj
    jump.bits() | (dest.bits() << 3) | (comp.bits() << 6) | (0b111 << 13)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_comp_opcode_returns_expected_bits() {
        let result = get_comp_opcode(&CompFlags::_0, &DestFlags::D, &JumpFlags::JMP);
        assert_eq!(result, 0b1_110_101_010_010_111);
    }
}
