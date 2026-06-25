pub mod code;
pub mod parser;
pub mod parser_new;

use anyhow::{Context, Result, anyhow};
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::Parser;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use crate::instructions::{Instruction, Instructions};

pub const SYM: char = '@';
pub const DEST: char = '=';
pub const JUMP: char = ';';
pub const LBL: char = '(';
pub const END_LBL: char = ')';

//const INPUT_FILE_EXT: &str = "asm";
const OUTPUT_FILE_EXT: &str = "hack";

pub fn assemble_file(input: &PathBuf, output: Option<PathBuf>) -> Result<()> {
    println!("Assembling file: {}", input.display());

    let src = fs::read_to_string(input).with_context(|| "Should read from file")?;
    let (result, errs) = parser_new::parser().parse(src.trim()).into_output_errors();

    for e in errs {
        Report::build(ReportKind::Error, ((), e.span().into_range()))
            .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
            .with_message(e.to_string())
            .with_label(
                Label::new(((), e.span().into_range()))
                    .with_message(e.reason().to_string())
                    .with_color(Color::Red),
            )
            .finish()
            .print(Source::from(&src))?;
    }

    let operators = result.ok_or_else(|| anyhow!("Failed to parse instructions from file"))?;
    let instructions = Instructions(
        operators
            .iter()
            .map(|o| Instruction {
                line_number: 0,
                operator: o.clone(),
            })
            .collect::<Vec<_>>(),
    );

    // let instructions = parser::read_instructions(input).with_context(|| {
    //     format!(
    //         "Failed to parse instructions from file: {}",
    //         input.display()
    //     )
    // })?;

    print!("{instructions}");

    let opcodes = code::generate_opcodes(&instructions.0)?;

    let output_file_path = output.unwrap_or_else(|| input.with_extension(OUTPUT_FILE_EXT));
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
