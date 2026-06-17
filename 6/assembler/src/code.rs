use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::instructions::Instruction;

const RAM_START: u16 = 16;

pub fn generate_opcodes(instructions: &Vec<Instruction>) -> Result<Vec<u16>, MissingSymbolError> {
    // first pass: find all labels and non-base symbols
    let symbols = create_symbols(&instructions);

    // now generate the code for each instruction
    let mut opcodes = Vec::<u16>::new();

    for i in instructions {
        match generate_instruction(&symbols, i)? {
            Some(opcode) => opcodes.push(opcode),
            None => {}
        }
    }

    Ok(opcodes)
}

fn generate_instruction(
    symbols: &HashMap<&str, u16>,
    instruction: &Instruction,
) -> Result<Option<u16>, MissingSymbolError> {
    match instruction {
        Instruction::Addr { symbol } => match symbol.parse::<u16>() {
            Ok(address) => Ok(Some(address)),
            Err(_) => match symbols
                .get(symbol.as_str())
                .or_else(|| BASE_SYMBOLS.get(symbol.as_str()))
            {
                Some(&symbol_address) => Ok(Some(symbol_address)),
                None => Err(MissingSymbolError {
                    symbol: symbol.to_string(),
                }),
            },
        },
        Instruction::Comp { dest, comp, jump } => {
            // 111accccccdddjjj
            Ok(Some(
                jump.bits() | (dest.bits() << 3) | (comp.bits() << 6) | (0b111 << 13),
            ))
        }
        _ => Ok(None),
    }
}

fn create_symbols(instructions: &Vec<Instruction>) -> HashMap<&str, u16> {
    let mut symbols = HashMap::<&str, u16>::new();
    let mut ram_pos: u16 = RAM_START;
    let mut pc: u16 = 0; // program counter

    // first pass: find all labels
    for i in instructions {
        // labels point at the next instruction but are not instructions themselves
        // so do not increment the program counter
        if let Instruction::Label { symbol } = i {
            if !symbols.contains_key(symbol.as_str()) {
                symbols.insert(symbol, pc);
            }
        } else {
            pc += 1;
        }
    }

    // second pass: find all variables (address instructions that dont reference a label or base symbol)
    for i in instructions {
        if let Instruction::Addr { symbol } = i
            && symbol.parse::<u16>().is_err()
        {
            if !BASE_SYMBOLS.contains_key(symbol.as_str()) && !symbols.contains_key(symbol.as_str())
            {
                symbols.insert(symbol, ram_pos);
                ram_pos += 1;
            }
        }
    }

    symbols
}

lazy_static! {
    static ref BASE_SYMBOLS: HashMap<&'static str, u16> = {
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
    };
}

#[derive(Debug, Clone)]
pub struct MissingSymbolError {
    symbol: String,
}

impl std::error::Error for MissingSymbolError {}
impl std::fmt::Display for MissingSymbolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot find symbol {} in table.", self.symbol)
    }
}
