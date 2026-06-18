use thiserror::Error;

#[derive(Debug, Error)]
#[error("[L: {line_number:?}] {kind:?}.")]
pub struct InstructionError {
    pub line_number: u32,
    pub kind: InstructionErrorKind, 
}

#[derive(Debug, Error)]
pub enum InstructionErrorKind {
    #[error("Undefined symbol: {symbol:?}.")]
    UndefinedSymbol { symbol: String },
    #[error("Invalid comp instruction: {comp:?}.")]
    InvalidComp { comp: String },
    #[error("Invalid jump instruction: {jump:?}.")]
    InvalidJump { jump: String },
    #[error("Invalid dest instruction: {dest:?}.")]
    InvalidDest { dest: String },
}
