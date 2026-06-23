use bitflags::bitflags;

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
        const _0     = 0b10_1010;
        #[bitflags(flag_name = "1")]
        const _1     = 0b11_1111;
        #[bitflags(flag_name = "-1")]
        const Neg1   = 0b11_1010;
        const D      = 0b00_1100;
        const A      = 0b11_0000;
        #[bitflags(flag_name = "!D")]
        const NotD   = 0b00_1101;
        #[bitflags(flag_name = "!A")]
        const NotA   = 0b11_0001;
        #[bitflags(flag_name = "-D")]
        const NegD   = 0b00_1111;
        #[bitflags(flag_name = "-A")]
        const NegA   = 0b11_0011;
        #[bitflags(flag_name = "D+1")]
        const DPlus1 = 0b01_1111;
        #[bitflags(flag_name = "A+1")]
        const APlus1 = 0b11_0111;
        #[bitflags(flag_name = "D-1")]
        const DNeg1  = 0b00_1110;
        #[bitflags(flag_name = "A-1")]
        const ANeg1  = 0b11_0010;
        #[bitflags(flag_name = "D+A")]
        const DPlusA = 0b00_0010;
        #[bitflags(flag_name = "D-A")]
        const DNegA  = 0b01_0011;
        #[bitflags(flag_name = "A-D")]
        const ANegD  = 0b00_0111;
        #[bitflags(flag_name = "D&A")]
        const DAndA  = 0b00_0000;
        #[bitflags(flag_name = "D|A")]
        const DOrA   = 0b01_0101;

        // a = 1
        const M      = 0b1_110000;
        #[bitflags(flag_name = "!M")]
        const NotM   = 0b1_110001;
        #[bitflags(flag_name = "-M")]
        const NegM   = 0b1_110011;
        #[bitflags(flag_name = "M+1")]
        const MPlus1 = 0b1_110111;
        #[bitflags(flag_name = "M-1")]
        const MNeg1  = 0b1_110010;
        #[bitflags(flag_name = "D+M")]
        const DPlusM = 0b1_000010;
        #[bitflags(flag_name = "D-M")]
        const DNegM  = 0b1_010011;
        #[bitflags(flag_name = "M-D")]
        const MNegD  = 0b1_000111;
        #[bitflags(flag_name = "D&M")]
        const DAndM  = 0b1_000000;
        #[bitflags(flag_name = "D|M")]
        const DOrM   = 0b1_010101;
    }
}