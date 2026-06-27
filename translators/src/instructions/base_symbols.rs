use std::{collections::HashMap, sync::OnceLock};

pub const SCREEN: u16 = 16384;
pub const KBD: u16 = 24576;
pub const SP: u16 = 0;
pub const LCL: u16 = 1;
pub const ARG: u16 = 2;
pub const THIS: u16 = 3;
pub const THAT: u16 = 4;
pub const TEMP: u16 = 5;
pub const R0: u16 = 0;
pub const R1: u16 = 1;
pub const R2: u16 = 2;
pub const R3: u16 = 3;
pub const R4: u16 = 4;
pub const R5: u16 = 5;
pub const R6: u16 = 6;
pub const R7: u16 = 7;
pub const R8: u16 = 8;
pub const R9: u16 = 9;
pub const R10: u16 = 10;
pub const R11: u16 = 11;
pub const R12: u16 = 12;
pub const R13: u16 = 13;
pub const R14: u16 = 14;
pub const R15: u16 = 15;
pub const RAM_START: u16 = 16;

pub fn by_name() -> &'static HashMap<&'static str, u16> {
    static BASE_SYMBOL_NAMES: OnceLock<HashMap<&'static str, u16>> = OnceLock::new();
    BASE_SYMBOL_NAMES.get_or_init(|| {
        HashMap::from([
            ("SCREEN", SCREEN),
            ("KBD", KBD),
            ("SP", SP),
            ("LCL", LCL),
            ("ARG", ARG),
            ("THIS", THIS),
            ("THAT", THAT),
            ("TEMP", TEMP),
            ("R0", R0),
            ("R1", R1),
            ("R2", R2),
            ("R3", R3),
            ("R4", R4),
            ("R5", R5),
            ("R6", R6),
            ("R7", R7),
            ("R8", R8),
            ("R9", R9),
            ("R10", R10),
            ("R11", R11),
            ("R12", R12),
            ("R13", R13),
            ("R14", R14),
            ("R15", R15),
        ])
    })
}

pub fn by_address() -> &'static HashMap<u16, &'static str> {
    static BASE_SYMBOL_ADDRS: OnceLock<HashMap<u16, &'static str>> = OnceLock::new();
    BASE_SYMBOL_ADDRS.get_or_init(|| {
        by_name()
            .iter()
            .filter(|(k, _)| {
                **k != "R0"
                    && **k != "R1"
                    && **k != "R2"
                    && **k != "R3"
                    && **k != "R4"
                    && **k != "R5"
            })
            .map(|(k, v)| (*v, *k))
            .collect()
    })
}
