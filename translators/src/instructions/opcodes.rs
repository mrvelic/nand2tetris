use anyhow::Result;
use std::collections::HashMap;

use super::error::{InstructionError, InstructionErrorKind};
use super::{CompFlags, DestFlags, JumpFlags, OperatorKind};

pub trait OpcodeGenerator {
    fn get_opcode(&self, symbols: &HashMap<&str, u16>) -> Result<Option<u16>>;
}

impl OpcodeGenerator for OperatorKind {
    fn get_opcode(&self, symbols: &HashMap<&str, u16>) -> Result<Option<u16>> {
        let opcode = match self {
            OperatorKind::Address(address) => Some(*address),
            OperatorKind::Symbol(symbol) => Some(lookup_symbol_address(symbols, symbol)?),
            OperatorKind::Comp(dest, comp, jump) => Some(get_comp_opcode(comp, dest, jump)),
            OperatorKind::Label(_) => None,
        };

        Ok(opcode)
    }
}

fn lookup_symbol_address(symbols: &HashMap<&str, u16>, symbol: &str) -> Result<u16> {
    Ok(*symbols.get(symbol).ok_or_else(|| InstructionError {
        line_number: 0,
        kind: InstructionErrorKind::UndefinedSymbol {
            symbol: symbol.to_owned(),
        },
    })?)
}

fn get_comp_opcode(comp: &CompFlags, dest: &DestFlags, jump: &JumpFlags) -> u16 {
    // 111accccccdddjjj
    *jump as u16 | (dest.bits() << 3) | ((*comp as u16) << 6) | (0b111 << 13)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_comp_opcode_bits() {
        let result = get_comp_opcode(&CompFlags::False, &DestFlags::D, &JumpFlags::JMP);
        assert_eq!(result, 0b1_110_101_010_010_111);
    }
}
