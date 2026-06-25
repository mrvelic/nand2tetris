#![warn(clippy::pedantic)]

mod assembler;
mod instructions;
mod vm_translator;

use anyhow::Result;
use clap::Parser;
use std::{env, fs, path::PathBuf};

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Assemble hack asm files into hack binary files
    Assemble {
        /// Input file path to assemble.
        input: PathBuf,
        /// Output file path. If omitted, the output path will be the same as the input with a .hack extension.
        output: Option<PathBuf>,
    },
    /// Translate jack VM commands into hack assembly
    Translate {
        /// Input file path to translate.
        input: PathBuf,
        /// Output file path. If omitted, the output path will be the same as the input with a .asm extension.
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Assemble { input, output } => {
            assembler::assemble_file(&input, output)?;
        }
        Commands::Translate { input, output } => {
            vm_translator::translate_file(&input, output)?;
        }
    }

    // let current_dir = env::current_dir().expect("Working directory should be accessible.");

    // let asm_files = find_asm_files(current_dir)
    //     .expect("Working directory should be possible to enumerate.")
    //     .expect("Working directory should contain .asm files to process, or a path to an asm file should be passed as the first argument.");

    // for file_path in asm_files {

    // }

    Ok(())
}

// TODO: Make this a more generic file list finder
#[allow(unused)]
fn find_files(in_dir: PathBuf, extension: &str) -> Result<Option<Vec<PathBuf>>> {
    let args: Vec<String> = env::args().collect();

    let files = match args.len() {
        // with no args, enumerate all *.asm files in working directory
        1 => {
            let dir_entry = fs::read_dir(in_dir)?;

            let files = dir_entry
                .filter_map(std::result::Result::ok)
                .map(|entry| entry.path())
                .filter_map(|path| {
                    if path.extension().is_some_and(|ext| ext == extension) {
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
