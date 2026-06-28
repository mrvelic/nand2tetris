mod helpers;
mod labels;
pub mod parser;

use anyhow::{Context, Result};
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::Parser;
use std::{
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::instructions::{
    CompFlags as Cmp, DestFlags as Dst, JumpFlags as Jmp, OperatorKind as Op, base_symbols::*,
};

use helpers::*;
use parser::Command;

pub fn translate_file(input: &PathBuf, output: Option<PathBuf>) -> Result<()> {
    let program_name = input
        .file_prefix()
        .with_context(|| "Should be able to read a file name from the path")?
        .to_str().with_context(|| "Should be able to convert the file name to a normal string for use as the program name")?;

    let src = fs::read_to_string(input).with_context(|| "Should read from file")?;
    let (result, errs) = parser::parser().parse(src.trim()).into_output_errors();

    report_errors(&src, errs)?;

    let mut ops = Vec::<Op>::new();

    // base code segments
    ops.extend(setup_segment());
    ops.extend(bool_stack_compare(labels::EQ, labels::EQ_RET, Jmp::JNE));
    ops.extend(bool_stack_compare(labels::LT, labels::LT_RET, Jmp::JGE));
    ops.extend(bool_stack_compare(labels::GT, labels::GT_RET, Jmp::JLE));

    // write translated commands
    ops.push(lbl(labels::CODE));
    if let Some(commands) = result {
        let mut index: u32 = 0;
        for command in commands {
            translate_command(program_name, &mut ops, index, command);
            index += 1;
        }
    }

    // infinite end loop
    ops.extend([
        lbl(labels::END),
        lbl_sym(labels::END),
        ij(Cmp::Zero, Jmp::JMP),
    ]);

    write_output_file(input, output, ops)?;

    Ok(())
}

fn write_output_file(
    input: &PathBuf,
    output: Option<PathBuf>,
    ops: Vec<Op>,
) -> Result<(), anyhow::Error> {
    let output_file_path = output.unwrap_or_else(|| input.with_extension("asm"));
    let output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&output_file_path)
        .with_context(|| format!("Need to open {output_file_path:?} for writing."))?;

    let mut output_buf = BufWriter::new(output_file);
    for o in ops {
        writeln!(output_buf, "{o}")?;
    }

    output_buf.flush()?;
    Ok(())
}

fn translate_command<'name>(
    program_name: &'name str,
    ops: &mut Vec<Op>,
    index: u32,
    command: Command,
) {
    match command {
        Command::Push(segment, value) => {
            if move_to_addr(program_name, ops, segment, value) {
                ops.push(i(Dst::D, Cmp::M));
            }

            ops.extend(push_stack());
        }
        Command::Pop(segment, value) => {
            ops.extend(pop_stack());

            // store the value we popped into R13
            ops.push(sym(R13));
            ops.push(i(Dst::M, Cmp::D));

            // move to the address for the segment
            move_to_addr(program_name, ops, segment, value);

            // store the address of the target segment in D
            ops.push(i(Dst::D, Cmp::A));
            ops.push(sym(R14));
            ops.push(i(Dst::M, Cmp::D));

            // load the popped value back to D
            ops.push(sym(R13));
            ops.push(i(Dst::D, Cmp::M));

            // move to the target address
            ops.push(sym(R14));
            ops.push(i(Dst::A, Cmp::M));

            // store the popped value at the target
            ops.push(i(Dst::M, Cmp::D));
        }
        Command::Add => {
            ops.extend(pop_stack());
            ops.push(i(Dst::M, Cmp::DPlusM));
        }
        Command::Subtract => {
            ops.extend(pop_stack());
            ops.push(i(Dst::M, Cmp::MNegD));
        }
        Command::Equal => ops.extend(do_bool_compare(labels::EQ, index)),
        Command::LessThan => ops.extend(do_bool_compare(labels::LT, index)),
        Command::GreaterThan => ops.extend(do_bool_compare(labels::GT, index)),
        Command::Or => {
            ops.extend(pop_stack());
            ops.push(i(Dst::M, Cmp::DOrM));
        }
        Command::And => {
            ops.extend(pop_stack());
            ops.push(i(Dst::M, Cmp::DAndM));
        }
        Command::Not => {
            ops.extend([sym(SP), i(Dst::A, Cmp::MNeg1), i(Dst::M, Cmp::NotM)]);
        }
        Command::Negate => {
            ops.extend([sym(SP), i(Dst::A, Cmp::MNeg1), i(Dst::M, Cmp::NegM)]);
        }
    }
}

fn report_errors<'src>(
    src: &'src str,
    errs: Vec<chumsky::prelude::Rich<'src, char>>,
) -> Result<(), anyhow::Error> {
    Ok(for e in errs {
        Report::build(ReportKind::Error, ((), e.span().into_range()))
            .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
            .with_message(e.to_string())
            .with_label(
                Label::new(((), e.span().into_range()))
                    .with_message(e.reason().to_string())
                    .with_color(Color::Red),
            )
            .finish()
            .print(Source::from(src))?;
    })
}
