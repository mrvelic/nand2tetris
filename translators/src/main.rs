#![warn(clippy::pedantic)]

mod assembler;

use anyhow::{Context, Result};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use assembler::{code, parser};

const INPUT_FILE_EXT: &str = "asm";
const OUTPUT_FILE_EXT: &str = "hack";

fn main() -> Result<()> {
    let current_dir = env::current_dir().expect("Working directory should be accessible.");
    let asm_files = find_asm_files(current_dir)
        .expect("Working directory should be possible to enumerate.")
        .expect("Working directory should contain .asm files to process, or a path to an asm file should be passed as the first argument.");

    for file_path in asm_files {
        println!("Assembling file: {}", file_path.display());
        assemble_file(&file_path)?;
    }

    Ok(())
}

fn assemble_file(input_file_path: &PathBuf) -> Result<()> {
    let instructions = parser::read_instructions(input_file_path).with_context(|| {
        format!(
            "Failed to parse instructions from file: {}",
            input_file_path.display()
        )
    })?;

    let opcodes = code::generate_opcodes(&instructions)?;

    let output_file_path = input_file_path.with_extension(OUTPUT_FILE_EXT);
    let output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&output_file_path)?;

    let mut output_buf = BufWriter::new(output_file);

    write_opcodes(&opcodes, &mut output_buf).with_context(|| {
        format!(
            "Failed to generate output file: {}",
            &output_file_path.display()
        )
    })?;

    println!("Assembled into: {}", output_file_path.display());
    Ok(())
}

fn write_opcodes<W: Write>(opcodes: &[u16], output_buf: &mut BufWriter<W>) -> Result<()> {
    for opcode in opcodes {
        writeln!(output_buf, "{opcode:016b}")?;
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
