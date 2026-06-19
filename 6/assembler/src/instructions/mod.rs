pub mod flags;
pub mod opcodes;
pub mod error;

pub use flags::{CompFlags, DestFlags, JumpFlags};

#[derive(Debug)]
pub struct Instruction {
    pub line_number: u32,
    pub operator: OperatorKind,
}

#[derive(Debug)]
pub enum OperatorKind {
    Address {
        address: u16,
    },
    Symbol {
        symbol: String,
    },
    Label {
        label: String,
    },
    Comp {
        comp: CompFlags,
        dest: DestFlags,
        jump: JumpFlags,
    },
}
