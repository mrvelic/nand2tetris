use std::str::FromStr;

use chumsky::prelude::*;

use super::{DEST, END_LBL, JUMP, LBL, SYM};
use crate::instructions::{CompFlags, DestFlags, JumpFlags, OperatorKind};

type Error<'src> = Rich<'src, char, SimpleSpan>;

pub fn parser<'src>() -> impl Parser<'src, &'src str, Vec<OperatorKind>, extra::Err<Error<'src>>> {
    let comment = just("//")
        .then(any().and_is(text::newline().not()).repeated())
        .padded();

    let digits = text::digits(10).to_slice().from_str::<u16>().unwrapped();
    let ident = any()
        .filter(char::is_ascii_graphic)
        .and_is(none_of("@()=;"))
        .repeated()
        .at_least(1)
        .to_slice()
        .labelled("identifier");

    let symbol = just(SYM)
        .ignore_then(ident.map(|s: &str| OperatorKind::Symbol(s.to_string())))
        .labelled("symbol");

    let address = just(SYM)
        .ignore_then(digits.map(OperatorKind::Address))
        .labelled("address");

    let label = ident
        .map(|s: &str| OperatorKind::Label(s.to_string()))
        .delimited_by(just(LBL), just(END_LBL))
        .labelled("label");

    let dest_flags = ident
        .then_ignore(just(DEST))
        .try_map(|dest, e| parse_dest_flags(dest, e))
        .labelled("dest");

    let jump_flags = just(JUMP)
        .ignore_then(ident)
        .try_map(|jump, span| {
            JumpFlags::from_str(jump).map_err(|e| Rich::custom(span, e.to_string()))
        })
        .labelled("jump");

    let comp_flags = ident
        .try_map(|comp, span| {
            CompFlags::from_str(comp).map_err(|e| Rich::custom(span, e.to_string()))
        })
        .labelled("comp");

    let instruction = group((dest_flags.or_not(), comp_flags, jump_flags.or_not()))
        .map(|(dest, comp, jump)| {
            OperatorKind::Comp(
                dest.unwrap_or(DestFlags::None),
                comp,
                jump.unwrap_or(JumpFlags::None),
            )
        })
        .labelled("instruction");

    choice((instruction, address, label, symbol))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
        .collect()
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
fn parse_dest_flags<'src>(dest: &str, span: SimpleSpan) -> Result<DestFlags, Error<'src>> {
    let mut dest_val = DestFlags::None;
    for c in dest.chars() {
        let parsed_flags = DestFlags::from_name(c.to_string().as_str())
            .ok_or_else(|| Rich::custom(span, "Invalid destination flag."))?;

        dest_val |= parsed_flags;
    }

    Ok(dest_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_address() {
        let result = parser().parse("@12345").into_output().unwrap();

        assert_eq!(result[0], OperatorKind::Address(12345));
    }

    #[test]
    fn it_should_parse_label() {
        let result = parser().parse("(Prog.$if_true)").into_output().unwrap();

        assert_eq!(result[0], OperatorKind::Label("Prog.$if_true".to_string()));
    }

    #[test]
    fn it_should_parse_symbol() {
        let result = parser().parse("@Prog.$if_true").into_output().unwrap();

        assert_eq!(result[0], OperatorKind::Symbol("Prog.$if_true".to_string()));
    }

    #[test]
    fn it_should_parse_symbol_without_newline() {
        let result = parser().parse("@Prog.$if_true\n").into_output().unwrap();

        assert_eq!(result[0], OperatorKind::Symbol("Prog.$if_true".to_string()));
    }

    #[test]
    fn it_should_parse_dest_instruction() {
        let result = parser().parse("D=A").into_output().unwrap();

        assert_eq!(
            result[0],
            OperatorKind::Comp(DestFlags::D, CompFlags::A, JumpFlags::None)
        );
    }

    #[test]
    fn it_should_parse_jump_instruction() {
        let result = parser().parse("0;JMP").into_output().unwrap();

        assert_eq!(
            result[0],
            OperatorKind::Comp(DestFlags::None, CompFlags::Zero, JumpFlags::JMP)
        );
    }

    #[test]
    fn it_should_parse_full_instruction() {
        let result = parser().parse("ADM=M+1;JMP").into_output().unwrap();

        assert_eq!(
            result[0],
            OperatorKind::Comp(
                DestFlags::A | DestFlags::D | DestFlags::M,
                CompFlags::MPlus1,
                JumpFlags::JMP
            )
        );
    }

    #[test]
    fn it_should_parse_multiple_lines() {
        let result = parser().parse("@256\nD=A").into_output().unwrap();

        assert_eq!(result[0], OperatorKind::Address(256));
        assert_eq!(
            result[1],
            OperatorKind::Comp(DestFlags::D, CompFlags::A, JumpFlags::None)
        );
    }
}
