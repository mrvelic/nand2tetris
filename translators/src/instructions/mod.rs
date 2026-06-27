pub mod base_symbols;
pub mod error;
pub mod flags;
pub mod opcodes;

pub use flags::{CompFlags, DestFlags, JumpFlags};

pub trait AssemblyString {
    fn to_asm(&self) -> String;
}

pub struct Instructions(pub Vec<Instruction>);

#[derive(Debug)]
pub struct Instruction {
    #[allow(unused)]
    pub line_number: u32,
    pub operator: OperatorKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OperatorKind {
    Address(u16),
    Symbol(String),
    Label(String),
    Comp(DestFlags, CompFlags, JumpFlags),
}

impl AssemblyString for OperatorKind {
    fn to_asm(&self) -> String {
        match self {
            Self::Address(address) => format!("@{address}"),
            Self::Symbol(symbol) => format!("@{symbol}"),
            Self::Label(label) => format!("({label})"),
            Self::Comp(dest, comp, jump) => {
                let mut comp_str = String::new();

                if !dest.is_empty() {
                    comp_str.push_str(format!("{dest}=").as_str());
                }

                comp_str.push_str(format!("{comp}").as_str());

                if *jump != JumpFlags::None {
                    comp_str.push_str(format!(";{jump}").as_str());
                }

                comp_str
            }
        }
    }
}

impl std::fmt::Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            writeln!(f, "{i}")?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = &self.operator;
        write!(f, "{op}")
    }
}

impl std::fmt::Display for OperatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let asm = self.to_asm();
        write!(f, "{asm}")
    }
}
