
use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Push(Segment, u16),
    Pop(Segment, u16),
    Add,
    Subtract,
    Negate,
    Equal,
    GreaterThan,
    LessThan,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

pub fn parser<'src>()
-> impl Parser<'src, &'src str, Vec<Command>, extra::Err<Rich<'src, char, SimpleSpan>>> {
    let index = text::int(10)
        .to_slice()
        .from_str::<u16>()
        .unwrapped()
        .labelled("index");

    let segment = choice((
            just("argument").to(Segment::Argument),
            just("local").to(Segment::Local),
            just("static").to(Segment::Static),
            just("constant").to(Segment::Constant),
            just("this").to(Segment::This),
            just("that").to(Segment::That),
            just("pointer").to(Segment::Pointer),
            just("temp").to(Segment::Temp),
        ))
        .padded()
        .then(index)
        .labelled("segment");

    //let push = just("push").padded().then(segment).to(Push);

    let comment = just("//")
        .then(any().and_is(text::newline().not()).repeated())
        .padded();

    choice((
        just("push").then(segment.clone()).map(|(_, (s, i))| Command::Push(s, i)),
        just("pop").then(segment.clone()).map(|(_, (s, i))| Command::Pop(s, i)),
        just("eq").to(Command::Equal),
        just("gt").to(Command::GreaterThan),
        just("lt").to(Command::LessThan),
        just("add").to(Command::Add),
        just("sub").to(Command::Subtract),
        just("neg").to(Command::Negate),
        just("and").to(Command::And),
        just("or").to(Command::Or),
        just("not").to(Command::Not),
    ))
    .padded_by(comment.repeated())
    .padded()
    .repeated()
    .collect()
    .labelled("command")

}
