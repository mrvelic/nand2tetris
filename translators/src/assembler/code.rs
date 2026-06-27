use anyhow::Result;
use std::collections::HashMap;

use crate::instructions::{
    Instruction, OperatorKind,
    base_symbols::{self, RAM_START},
    opcodes::OpcodeGenerator,
};

pub fn generate_opcodes(instructions: &[Instruction]) -> Result<Vec<u16>> {
    // first pass: find all labels and non-base symbols
    let mut symbols = extract_symbols(instructions);
    symbols.extend(base_symbols::by_name());

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
            && !base_symbols::by_name().contains_key(symbol.as_str())
            && !symbols.contains_key(symbol.as_str())
        {
            symbols.insert(symbol, ram_pos);
            ram_pos += 1;
        }
    }

    symbols
}
