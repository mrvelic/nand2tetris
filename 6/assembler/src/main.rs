#![warn(clippy::pedantic)]

mod code;
mod instructions;
mod parser;

use anyhow::{Result, anyhow};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use crate::instructions::Instruction;

const INPUT_FILE_EXT: &str = "asm";
const OUTPUT_FILE_EXT: &str = "hack";

fn main() {
    let current_dir = env::current_dir().expect("Working directory should be accessible.");
    let asm_files = find_asm_files(current_dir)
        .expect("Working directory should be possible to enumerate.")
        .expect("Working directory should contain .asm files to process, or a path to an asm file should be passed as the first argument.");

    for file_path in asm_files {
        println!("Assembling file: {}", file_path.display());
        assemble_file(&file_path).unwrap_or_else(|why| eprintln!("{why}"));
    }
}

fn assemble_file(input_file_path: &PathBuf) -> Result<()> {
    let instructions = parser::read_instructions(input_file_path).map_err(|why| {
        anyhow!(
            "Failed to parse instructions from file {}: {}",
            input_file_path.display(),
            why
        )
    })?;

    let output_file_path = input_file_path.with_extension(OUTPUT_FILE_EXT);
    generate_output(&output_file_path, &instructions).map_err(|why| {
        anyhow!(
            "Failed to generate output file {}: {}",
            output_file_path.display(),
            why
        )
    })?;

    println!("Assembled into: {}", output_file_path.display());
    Ok(())
}

fn generate_output(output_file_path: &PathBuf, instructions: &[Instruction]) -> Result<()> {
    let output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file_path)?;
    let mut output_buf = BufWriter::new(output_file);

    let opcodes = code::generate_opcodes(instructions)?;
    for opcode in opcodes {
        writeln!(&mut output_buf, "{opcode:016b}")?;
    }

    output_buf.flush()?;

    Ok(())
}

fn find_asm_files(in_dir: PathBuf) -> Result<Option<Vec<PathBuf>>> {
    let args: Vec<String> = env::args().collect();

    let files = match args.len() {
        // with no args, enumerate all *.asm files in working directory
        1 => {
            let dir_entry = fs::read_dir(in_dir)?;

            let files = dir_entry
                .filter_map(std::result::Result::ok)
                .map(|entry| entry.path())
                .filter_map(|path| {
                    if path.extension().is_some_and(|ext| ext == INPUT_FILE_EXT) {
                        Some(path)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            Some(files)
        }
        // with 1 arg, use that as the target asm file
        2 => Some(vec![in_dir.join(&args[1])]),
        _ => None,
    };

    Ok(files)
}
