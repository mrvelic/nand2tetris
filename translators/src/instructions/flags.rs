use bitflags::bitflags;
use strum::{Display, EnumString, FromRepr};

impl Default for DestFlags {
    fn default() -> Self {
        Self::None
    }
}

impl Default for JumpFlags {
    fn default() -> Self {
        Self::None
    }
}

impl Default for CompFlags {
    fn default() -> Self {
        Self::DAndA
    }
}

impl std::fmt::Display for DestFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, _) in self.iter_names() {
            write!(f, "{name}")?;
        }

        Ok(())
    }
}

#[repr(u16)]
#[derive(Display, Debug, Copy, Clone, PartialEq, Eq, EnumString, FromRepr)]
pub enum JumpFlags {
    #[strum(serialize = "")]
    None = 0,
    JGT = 0b001,
    JEQ = 0b010,
    JGE = 0b011,
    JLT = 0b100,
    JNE = 0b101,
    JLE = 0b110,
    JMP = 0b111,
}

#[repr(u16)]
#[derive(Display, Debug, Copy, Clone, PartialEq, Eq, EnumString, FromRepr)]
pub enum CompFlags {
    // a = 0
    #[strum(serialize = "0")]
    False = 0b10_1010,
    #[strum(serialize = "1")]
    True = 0b11_1111,
    #[strum(serialize = "-1")]
    Neg1 = 0b11_1010,
    D = 0b00_1100,
    A = 0b11_0000,
    #[strum(serialize = "!D")]
    NotD = 0b00_1101,
    #[strum(serialize = "!A")]
    NotA = 0b11_0001,
    #[strum(serialize = "-D")]
    NegD = 0b00_1111,
    #[strum(serialize = "-A")]
    NegA = 0b11_0011,
    #[strum(serialize = "D+1")]
    DPlus1 = 0b01_1111,
    #[strum(serialize = "A+1")]
    APlus1 = 0b11_0111,
    #[strum(serialize = "D-1")]
    DNeg1 = 0b00_1110,
    #[strum(serialize = "A-1")]
    ANeg1 = 0b11_0010,
    #[strum(serialize = "D+A")]
    DPlusA = 0b00_0010,
    #[strum(serialize = "D-A")]
    DNegA = 0b01_0011,
    #[strum(serialize = "A-D")]
    ANegD = 0b00_0111,
    #[strum(serialize = "D&A")]
    DAndA = 0b00_0000,
    #[strum(serialize = "D|A")]
    DOrA = 0b01_0101,

    // a = 1
    M = 0b1_110000,
    #[strum(serialize = "!M")]
    NotM = 0b1_110001,
    #[strum(serialize = "-M")]
    NegM = 0b1_110011,
    #[strum(serialize = "M+1")]
    MPlus1 = 0b1_110111,
    #[strum(serialize = "M-1")]
    MNeg1 = 0b1_110010,
    #[strum(serialize = "D+M")]
    DPlusM = 0b1_000010,
    #[strum(serialize = "D-M")]
    DNegM = 0b1_010011,
    #[strum(serialize = "M-D")]
    MNegD = 0b1_000111,
    #[strum(serialize = "D&M")]
    DAndM = 0b1_000000,
    #[strum(serialize = "D|M")]
    DOrM = 0b1_010101,
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct DestFlags: u16 {
        const None = 0;
        const A = 0b100;
        const D = 0b010;
        const M = 0b001;
    }
}
