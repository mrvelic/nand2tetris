#![allow(unused)]
use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::OnceLock;

use crate::instructions::error::{InstructionError, InstructionErrorKind};
use crate::instructions::{
    CompFlags, DestFlags, Instruction, Instructions, JumpFlags, OperatorKind,
};

pub const SYM: char = '@';
pub const LBL: char = '(';
pub const END_LBL: char = ')';

const GRP_DEST: &str = "dest";
const GRP_COMP: &str = "comp";
const GRP_JUMP: &str = "jump";

fn comp_regex() -> &'static Regex {
    static COMP_REGEX: OnceLock<Regex> = OnceLock::new();
    COMP_REGEX.get_or_init(|| {
        Regex::new("(?:(?P<dest>[A-Z]+)?=)?(?P<comp>[A-Z0-9&!\\+\\-\\|]+);?(?P<jump>[A-Z]{3})?")
            .expect("Regex for comp expression should compile successfully.")
    })
}

pub fn read_instructions<'src>(file_path: &PathBuf) -> Result<Instructions> {
    let mut reader = BufReader::new(File::open(file_path)?);
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut line_number = 0;

    let mut line = String::new();

    while let Some(()) = read_until_relevant_line(&mut reader, &mut line_number, &mut line)? {
        // match the first character of each line
        let instruction = match line.chars().next().unwrap_or_default() {
            // '@' instruction (@SYM or @1234 (address))
            SYM => {
                let symbol = line.trim_start_matches(SYM);
                match symbol.parse::<u16>() {
                    Ok(address) => Instruction {
                        line_number,
                        operator: OperatorKind::Address(address),
                    },
                    Err(_) => Instruction {
                        line_number,
                        operator: OperatorKind::Symbol(symbol.to_string()),
                    },
                }
            }

            // '(LABEL)' instruction
            LBL => Instruction {
                line_number,
                operator: OperatorKind::Label(
                    line.trim_start_matches(LBL)
                        .trim_end_matches(END_LBL)
                        .to_string(),
                ),
            },

            // everything else is a C instruction as we know comments and empty lines are ignored already
            _ => parse_comp_instruction(line_number, &line)?,
        };

        instructions.push(instruction);
    }

    Ok(Instructions(instructions))
}

fn parse_comp_instruction(line_number: u32, line: &str) -> Result<Instruction> {
    let matches = comp_regex()
        .captures(line)
        .ok_or_else(|| InstructionError {
            line_number,
            kind: InstructionErrorKind::InvalidComp {
                comp: line.to_string(),
            },
        })?;

    let instruction = Instruction {
        line_number,
        operator: OperatorKind::Comp(
            matches.name(GRP_DEST).map_or(Ok(DestFlags::None), |m| {
                parse_dest_flags(m.as_str(), line_number)
            })?,
            map_and_parse_flags_or_else(
                &matches,
                line_number,
                GRP_COMP,
                CompFlags::default(),
                |comp| InstructionErrorKind::InvalidComp {
                    comp: comp.to_string(),
                },
            )?,
            map_and_parse_flags_or_else(
                &matches,
                line_number,
                GRP_JUMP,
                JumpFlags::None,
                |jump| InstructionErrorKind::InvalidJump {
                    jump: jump.to_string(),
                },
            )?,
        ),
    };

    Ok(instruction)
}

/// If `matches` contains `match_name`, then attempt to parse the flags of `T`. If the parse fails then error.
/// If the match does not exist, return `default`.
fn map_and_parse_flags_or_else<'src, T: FromStr, E: FnOnce(&'src str) -> InstructionErrorKind>(
    matches: &regex::Captures<'src>,
    line_number: u32,
    match_name: &'src str,
    default: T,
    err: E,
) -> Result<T> {
    Ok(matches.name(match_name).map_or(Ok(default), |m| {
        T::from_str(m.as_str()).map_err(|_| InstructionError {
            line_number,
            kind: err(m.as_str()),
        })
    })?)
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
fn parse_dest_flags<'src>(dest: &'src str, line_number: u32) -> Result<DestFlags> {
    let mut dest_val = DestFlags::None;
    for c in dest.chars() {
        let parsed_flags =
            DestFlags::from_name(c.to_string().as_str()).ok_or_else(|| InstructionError {
                line_number,
                kind: InstructionErrorKind::InvalidDest {
                    dest: dest.to_string(),
                },
            })?;

        dest_val |= parsed_flags;
    }

    Ok(dest_val)
}

/// Consumes the reader line by line, performing a trim on each line, then ignoring any comments (// prefix) or empty lines.
///
/// Returns `Some` on the first non-empty, non-comment line. Returns `None` if there are no more lines.
fn read_until_relevant_line<'src>(
    reader: &'src mut BufReader<File>,
    line_count: &'src mut u32,
    line: &'src mut String,
) -> Result<Option<()>> {
    while reader.read_line(line)? > 0 {
        *line_count += 1;
        let line = line.trim();

        if line.starts_with("//") {
            continue;
        }
        if line.is_empty() {
            continue;
        }

        return Ok(Some(()));
    }

    Ok(None)
}
