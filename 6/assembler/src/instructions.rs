use bitflags::bitflags;

#[derive(Debug)]
pub enum Instruction {
    Addr {
        symbol: String,
    },
    Label {
        symbol: String,
    },
    Comp {
        dest: DestFlags,
        comp: CompFlags,
        jump: JumpFlags,
    },
}

#[derive(Debug)]
pub struct DestFlags(u16);

#[derive(Debug)]
pub struct JumpFlags(u16);

#[derive(Debug)]
pub struct CompFlags(u16);

impl Default for DestFlags {
    fn default() -> Self {
        DestFlags::None
    }
}

impl Default for JumpFlags {
    fn default() -> Self {
        JumpFlags::None
    }
}

impl Default for CompFlags {
    fn default() -> Self {
        Self::empty()
    }
}

bitflags! {
    impl DestFlags: u16 {
        const None = 0;
        const M = 0b001;
        const D = 0b010;
        const A = 0b100;
    }

    impl JumpFlags: u16 {
        const None = 0;
        const JGT = 0b001;
        const JEQ = 0b010;
        const JGE = 0b011;
        const JLT = 0b100;
        const JNE = 0b101;
        const JLE = 0b110;
        const JMP = 0b111;
    }

    /// acccccc
    impl CompFlags: u16 {
        // a = 0
        #[bitflags(flag_name = "0")]
        const _0     = 0b101010;
        #[bitflags(flag_name = "1")]
        const _1     = 0b111111;
        #[bitflags(flag_name = "-1")]
        const Neg1   = 0b111010;
        const D      = 0b001100;
        const A      = 0b110000;
        #[bitflags(flag_name = "!D")]
        const NotD   = 0b001101;
        #[bitflags(flag_name = "!A")]
        const NotA   = 0b110001;
        #[bitflags(flag_name = "-D")]
        const NegD   = 0b001111;
        #[bitflags(flag_name = "-A")]
        const NegA   = 0b110011;
        #[bitflags(flag_name = "D+1")]
        const DPlus1 = 0b011111;
        #[bitflags(flag_name = "A+1")]
        const APlus1 = 0b110111;
        #[bitflags(flag_name = "D-1")]
        const DNeg1  = 0b001110;
        #[bitflags(flag_name = "A-1")]
        const ANeg1  = 0b110010;
        #[bitflags(flag_name = "D+A")]
        const DPlusA = 0b000010;
        #[bitflags(flag_name = "D-A")]
        const DNegA  = 0b010011;
        #[bitflags(flag_name = "A-D")]
        const ANegD  = 0b000111;
        #[bitflags(flag_name = "D&A")]
        const DAndA  = 0b000000;
        #[bitflags(flag_name = "D|A")]
        const DOrA   = 0b010101;

        // a = 1
        const M      = 0b1110000;
        #[bitflags(flag_name = "!M")]
        const NotM   = 0b1110001;
        #[bitflags(flag_name = "-M")]
        const NegM   = 0b1110011;
        #[bitflags(flag_name = "M+1")]
        const MPlus1 = 0b1110111;
        #[bitflags(flag_name = "M-1")]
        const MNeg1  = 0b1110010;
        #[bitflags(flag_name = "D+M")]
        const DPlusM = 0b1000010;
        #[bitflags(flag_name = "D-M")]
        const DNegM  = 0b1010011;
        #[bitflags(flag_name = "M-D")]
        const MNegD  = 0b1000111;
        #[bitflags(flag_name = "D&M")]
        const DAndM  = 0b1000000;
        #[bitflags(flag_name = "D|M")]
        const DOrM   = 0b1010101;
    }
}
