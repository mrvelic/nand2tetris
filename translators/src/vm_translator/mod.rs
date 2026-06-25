pub mod parser;

use anyhow::{Context, Result};
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::Parser;
use std::{fs, path::PathBuf};

pub fn translate_file(input: &PathBuf, _output: Option<PathBuf>) -> Result<()> {
    let src = fs::read_to_string(input).with_context(|| "Should read from file")?;
    let (result, errs) = parser::parser().parse(src.trim()).into_output_errors();

    println!("{result:#?}");

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

    Ok(())
}
