use std::io::{self, BufRead};
use std::path::{PathBuf};
use std::fs::File;

use crate::instructions::{Instruction, CompFlags, DestFlags, JumpFlags};

const SYM:     char = '@';
const LBL:     char = '(';
const END_LBL: char = ')';
const DEST:    char = '=';
const JMP:     char = ';';

pub fn read_instructions(file_path: &PathBuf) -> io::Result<Vec<Instruction>> {  
    let mut reader = io::BufReader::new(File::open(file_path)?).lines();      
    let mut instructions: Vec<Instruction> = Vec::new();
    
    while let Some(line) = read_until_relevant_line(&mut reader)? {
        // match the first character of each line
        let instruction = match &line.chars().nth(0).unwrap_or_default() {
            // '@' instruction (@SYM or @1234 (address))
            &SYM => Instruction::Addr { symbol: line.trim_start_matches(SYM).to_string() },

            // '(LABEL)' instruction
            &LBL => Instruction::Label { symbol: line.trim_start_matches(LBL).trim_end_matches(END_LBL).to_string() },

            // everything else is a C instruction as we know comments and empty lines are ignored already
            _ => {
                // split by `;` first to determine if a jump condition is set
                let (pre_jump, jump) = match line.split_once(JMP) {
                    Some((pre_jump, jump)) => (Some(pre_jump), Some(jump)),
                    None => (Some(line.as_str()), None),
                };

                // split by `=` to determine if a dest is specified
                let (dest, comp, jump) = match line.split_once(DEST) {
                    Some((dest, comp)) => (Some(dest), Some(comp), jump),
                    None => (None, pre_jump, jump),
                };

                Instruction::Comp {
                    dest: dest.and_then(parse_dest_flags).unwrap_or_default(),
                    jump: jump.and_then(JumpFlags::from_name).unwrap_or_default(),
                    comp: comp.and_then(CompFlags::from_name).unwrap_or_default(),
                }
            }
        };

        instructions.push(instruction);
    }

    Ok(instructions)
}

/// Parses dest instruction as flags.
/// Splits the dest string into individual chars and looks them up in the `DestFlags` enum, then OR's them together to build the flags.
/// 
/// # Examples
/// ```
/// 'A'   -> ['A']           -> DestFlags::A                               -> 0b100
/// 'DM'  -> ['D', 'M']      -> DestFlags::D | DestFlags::M                -> 0b011
/// 'ADM' -> ['A', 'D', 'M'] -> DestFlags::A | DestFlags::D | DestFlags::M -> 0b111
/// ```
fn parse_dest_flags(dest: &str) -> Option<DestFlags> {
    let mut dest_val = DestFlags::None;
    for c in dest.chars() {
        dest_val = dest_val | DestFlags::from_name(c.to_string().as_str()).unwrap_or(DestFlags::None)
    }
    
    Some(dest_val)
}

/// Consumes the reader line by line, performing a trim on each line, then ignoring any comments (// prefix) or empty lines.
/// 
/// Returns the first non-empty, non-comment line. Returns `None` if there are no more lines.
fn read_until_relevant_line(reader: &mut io::Lines<io::BufReader<File>>) -> io::Result<Option<String>> {
    for line in reader {
        let line = line?.trim().to_string();

        if line.starts_with("//") { continue; }
        if line.is_empty() { continue; }

        return Ok(Some(line));
    }

    Ok(None)
}
