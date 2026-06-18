use anyhow::Result;
use std::collections::HashMap;

use super::error::{InstructionError, InstructionErrorKind};
use super::{Instruction, OperatorKind};

pub trait OpcodeGenerator {
    fn get_opcode(&self, symbols: &HashMap<&str, u16>) -> Result<Option<u16>>;
}

impl OpcodeGenerator for Instruction {
    fn get_opcode(&self, symbols: &HashMap<&str, u16>) -> Result<Option<u16>> {
        let opcode = match &self.operator {
            OperatorKind::Address { address } => Some(*address),
            OperatorKind::Symbol { symbol } => Some(*symbols.get(symbol.as_str()).ok_or_else(
                || InstructionError {
                    line_number: self.line_number,
                    kind: InstructionErrorKind::UndefinedSymbol {
                        symbol: symbol.clone(),
                    },
                },
            )?),
            OperatorKind::Comp { dest, comp, jump } => {
                // 111accccccdddjjj
                Some(jump.bits() | (dest.bits() << 3) | (comp.bits() << 6) | (0b111 << 13))
            }
            OperatorKind::Label { label: _ } => None,
        };

        Ok(opcode)
    }
}
