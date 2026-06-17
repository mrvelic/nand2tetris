mod code;
mod instructions;
mod parser;

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::instructions::Instruction;

const INPUT_FILE_EXT: &str = "asm";
const OUTPUT_FILE_EXT: &str = "hack";

fn main() {
    let asm_files: Vec<PathBuf> = match env::current_dir() {
        Err(why) => panic!("Working directory inaccessible: {}", why),
        Ok(path) => match find_asm_files(path) {
            None => panic!("Failed to find *.asm files in working directory"),
            Some(files) => {
                files
            }
        },
    };

    for file_path in asm_files {
        assemble_file(&file_path);
    }
}

fn assemble_file(input_file_path: &PathBuf) {
    println!("Assembling file: {}", input_file_path.display());

    match parser::read_instructions(input_file_path) {
        Ok(result) => {
            let output_file_path = input_file_path.with_extension(OUTPUT_FILE_EXT);
            match generate_output(&output_file_path, &result) {
                Err(why) => eprintln!(
                    "Failed to generate output file {}: {}",
                    output_file_path.display(),
                    why
                ),
                _ => (),
            }
        }
        Err(why) => eprintln!(
            "Failed to parse instructions from file {}: {}",
            input_file_path.display(),
            why
        ),
    }
}

fn generate_output(
    output_file_path: &PathBuf,
    instructions: &Vec<Instruction>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file_path)?;
    let mut output_buf = std::io::BufWriter::new(output_file);

    dbg!(&instructions);
    let opcodes = code::generate_opcodes(instructions)?;
    for opcode in opcodes {
        writeln!(&mut output_buf, "{:016b}", opcode)?;
    }

    output_buf.flush()?;

    println!("Assembled into: {}", output_file_path.display());
    Ok(())
}

fn find_asm_files(in_dir: PathBuf) -> Option<Vec<PathBuf>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // with no args, enumerate all *.asm files in working directory
        1 => match fs::read_dir(in_dir) {
            Err(why) => {
                eprintln!("Failed to enumerate directory: {}", why);
                None
            }
            Ok(dir_entry) => {
                let files = dir_entry
                    .filter_map(|res| res.ok())
                    .map(|entry| entry.path())
                    .filter_map(|path| {
                        if path.extension().map_or(false, |ext| ext == INPUT_FILE_EXT) {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                Some(files)
            }
        },
        // with 1 arg, use that as the target asm file
        2 => Some(vec![in_dir.join(&args[1])]),
        _ => None,
    }
}
