use anyhow::Result;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::instructions::{Instruction, OperatorKind, opcodes::OpcodeGenerator};

const RAM_START: u16 = 16;

fn base_symbols() -> &'static HashMap<&'static str, u16> {
    static BASE_SYMBOLS: OnceLock<HashMap<&'static str, u16>> = OnceLock::new();
    BASE_SYMBOLS.get_or_init(|| {
        HashMap::from([
            ("SCREEN", 16384),
            ("KBD", 24576),
            ("SP", 0),
            ("LCL", 1),
            ("ARG", 2),
            ("THIS", 3),
            ("THAT", 4),
            ("R0", 0),
            ("R1", 1),
            ("R2", 2),
            ("R3", 3),
            ("R4", 4),
            ("R5", 5),
            ("R6", 6),
            ("R7", 7),
            ("R8", 8),
            ("R9", 9),
            ("R10", 10),
            ("R11", 11),
            ("R12", 12),
            ("R13", 13),
            ("R14", 14),
            ("R15", 15),
        ])
    })
}

pub fn generate_opcodes(instructions: &[Instruction]) -> Result<Vec<u16>> {
    // first pass: find all labels and non-base symbols
    let mut symbols = extract_symbols(instructions);
    symbols.extend(base_symbols());

    // now generate the code for each instruction
    let mut opcodes = Vec::<u16>::new();

    for i in instructions {
        if let Some(opcode) = i.operator.get_opcode(&symbols)? {
            opcodes.push(opcode);
        }
    }

    Ok(opcodes)
}

fn extract_symbols(instructions: &[Instruction]) -> HashMap<&str, u16> {
    let mut symbols = HashMap::<&str, u16>::new();
    let mut ram_pos: u16 = RAM_START;
    let mut pc: u16 = 0; // program counter

    // first pass: find all labels
    for i in instructions {
        // labels point at the next instruction but are not instructions themselves
        // so do not increment the program counter
        if let OperatorKind::Label(symbol) = &i.operator {
            if !symbols.contains_key(symbol.as_str()) {
                symbols.insert(symbol, pc);
            }
        } else {
            pc += 1;
        }
    }

    // second pass: find all variables (address instructions that dont reference a label or base symbol)
    for i in instructions {
        if let OperatorKind::Symbol(symbol) = &i.operator
            && !base_symbols().contains_key(symbol.as_str())
            && !symbols.contains_key(symbol.as_str())
        {
            symbols.insert(symbol, ram_pos);
            ram_pos += 1;
        }
    }

    symbols
}
