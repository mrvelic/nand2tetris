pub mod parser;

use anyhow::{Context, Result};
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::Parser;
use std::{
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::{
    instructions::{
        CompFlags as Cmp, DestFlags as Dst, JumpFlags as Jmp, OperatorKind as Op, base_symbols,
        base_symbols::*,
    },
    vm_translator::parser::{Command, Segment},
};

pub fn translate_file(input: &PathBuf, output: Option<PathBuf>) -> Result<()> {
    let src = fs::read_to_string(input).with_context(|| "Should read from file")?;
    let (result, errs) = parser::parser().parse(src.trim()).into_output_errors();

    println!("{result:#?}");

    let addrs = by_address();
    println!("{addrs:#?}");

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

    let mut ops = Vec::<Op>::new();

    match result {
        Some(commands) => {
            for command in commands {
                translate_command(&mut ops, command);
            }
        }
        None => {}
    }

    // infinite end loop
    ops.extend([
        Op::Label("END".to_string()),
        Op::Symbol("END".to_string()),
        ij(Cmp::False, Jmp::JMP),
    ]);

    let output_file_path = output.unwrap_or_else(|| input.with_extension("asm"));
    let output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&output_file_path)?;

    let mut output_buf = BufWriter::new(output_file);

    for o in ops {
        writeln!(output_buf, "{o}")?;
    }

    output_buf.flush()?;

    Ok(())
}

fn translate_command(ops: &mut Vec<Op>, command: Command) {
    match command {
        Command::Push(segment, value) => {
            load_addr(ops, segment, value);

            ops.extend(push_stack());
        }
        Command::Pop(segment, value) => {
            ops.extend(pop_stack());

            // store the value we popped into R13
            ops.push(sym(R13));
            ops.push(i(Dst::M, Cmp::D));

            // move to the address for the segment
            load_addr(ops, segment, value);

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

            // store the poped value at the target
            ops.push(i(Dst::M, Cmp::D));
        }
        Command::Add => {
            ops.extend(pop_stack());
            ops.push(i(Dst::M, Cmp::DPlusM));
        }
        Command::Subtract => {
            ops.extend(pop_stack());
            ops.push(i(Dst::M, Cmp::DNegM));
        }
        Command::Equal => {
            ops.extend(pop_stack());
            ops.push(i(Dst::D, Cmp::DNegM));
            ops.push(i(Dst::D, Cmp::NotD));
            ops.push(i(Dst::M, Cmp::True));
            ops.push(i(Dst::M, Cmp::DAndM));
        }
        Command::Or => {
            ops.extend(pop_stack());
            ops.push(i(Dst::M, Cmp::DOrM));
        }
        Command::And => {
            ops.extend(pop_stack());
            ops.push(i(Dst::M, Cmp::DAndM));
        }
        Command::Not => {
            ops.extend([sym(SP), i(Dst::A, Cmp::ANeg1), i(Dst::M, Cmp::NotM)]);
        }
        Command::Negate => {
            ops.extend([sym(SP), i(Dst::A, Cmp::ANeg1), i(Dst::M, Cmp::NegM)]);
        }
        _ => {}
    }
}

fn load_addr(ops: &mut Vec<Op>, segment: Segment, value: u16) {
    let addr = match segment {
        Segment::Argument => Some(ARG),
        Segment::Local => Some(LCL),
        Segment::Static => Some(RAM_START),
        Segment::This => Some(THIS),
        Segment::That => Some(THAT),
        Segment::Temp => {
            ops.push(sym(TEMP + value));
            return;
        }
        _ => None,
    };

    // set index value as D
    ops.extend([val(value), i(Dst::D, Cmp::A)]);

    if let Some(addr) = addr {
        ops.push(sym(addr));
        ops.push(i(Dst::A, Cmp::DPlusM));
    }
}

/// increment SP and store D into M at its address
fn push_stack() -> [Op; 4] {
    [
        sym(SP),
        i(Dst::A | Dst::M, Cmp::MPlus1),
        i(Dst::A, Cmp::ANeg1),
        i(Dst::M, Cmp::D),
    ]
}

/// decrement SP and store M at its address in D
fn pop_stack() -> [Op; 4] {
    [
        sym(SP),
        i(Dst::A | Dst::M, Cmp::MNeg1),
        i(Dst::D, Cmp::M),
        i(Dst::A, Cmp::ANeg1),
    ]
}

fn val(value: u16) -> Op {
    Op::Address(value)
}

fn sym(sym_address: u16) -> Op {
    match base_symbols::by_address().get(&sym_address) {
        Some(symbol) => Op::Symbol(symbol.to_string()),
        None => Op::Address(sym_address),
    }
}

fn i(dest: Dst, comp: Cmp) -> Op {
    Op::Comp(dest, comp, Jmp::None)
}

fn ij(comp: Cmp, jump: Jmp) -> Op {
    Op::Comp(Dst::None, comp, jump)
}
