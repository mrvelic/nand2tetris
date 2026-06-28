use super::{labels, parser::Segment};
use crate::instructions::{
    CompFlags as Cmp, DestFlags as Dst, JumpFlags as Jmp, OperatorKind as Op, base_symbols,
    base_symbols::*,
};

pub fn i(dest: Dst, comp: Cmp) -> Op {
    Op::Comp(dest, comp, Jmp::None)
}

pub fn ij(comp: Cmp, jump: Jmp) -> Op {
    Op::Comp(Dst::None, comp, jump)
}

pub fn val(value: u16) -> Op {
    Op::Address(value)
}

pub fn lbl(label: &'static str) -> Op {
    Op::Label(label.to_string())
}

pub fn lbl_sym(label: &'static str) -> Op {
    Op::Symbol(label.to_string())
}

pub fn sym(sym_address: u16) -> Op {
    match base_symbols::by_address().get(&sym_address) {
        Some(symbol) => Op::Symbol(symbol.to_string()),
        None => Op::Address(sym_address),
    }
}

/// increment SP and store D into M at its address
pub fn push_stack() -> [Op; 4] {
    [
        sym(SP),
        i(Dst::A | Dst::M, Cmp::MPlus1),
        i(Dst::A, Cmp::ANeg1),
        i(Dst::M, Cmp::D),
    ]
}

/// decrement SP and store M at its address in D
pub fn pop_stack() -> [Op; 4] {
    [
        sym(SP),
        i(Dst::A | Dst::M, Cmp::MNeg1),
        i(Dst::D, Cmp::M),
        i(Dst::A, Cmp::ANeg1),
    ]
}

/// Move to a memory segment location or constant
/// Returns `true` if the address indicates a memory location
pub fn move_to_addr<'name>(
    program_name: &'name str,
    ops: &mut Vec<Op>,
    segment: Segment,
    value: u16,
) -> bool {
    let addr = match segment {
        Segment::Argument => Some(ARG),
        Segment::Local => Some(LCL),
        Segment::This => Some(THIS),
        Segment::That => Some(THAT),
        Segment::Pointer => {
            ops.push(sym(if let 0 = value { THIS } else { THAT }));
            return true;
        }
        Segment::Static => {
            ops.push(Op::Symbol(format!("{program_name}.{value}")));
            return true;
        }
        Segment::Temp => {
            ops.push(sym(TEMP + value));
            return true;
        }
        _ => None,
    };

    // set index (or constant) value as D
    ops.extend([val(value), i(Dst::D, Cmp::A)]);

    // load the segment address, then move the address to the pointer location + value (D)
    if let Some(addr) = addr {
        ops.push(sym(addr));
        ops.push(i(Dst::A, Cmp::DPlusM));
        return true;
    }

    return false;
}

pub fn setup_segment() -> [Op; 7] {
    [
        // init stack pointer as 256
        lbl(labels::SETUP),
        sym(256),
        i(Dst::D, Cmp::A),
        sym(SP),
        i(Dst::M, Cmp::D),
        // jump to code
        lbl_sym(labels::CODE),
        ij(Cmp::Zero, Jmp::JMP),
    ]
}

pub fn do_bool_compare(label: &'static str, index: u32) -> [Op; 5] {
    [
        // store the return address in reg D
        Op::Symbol(format!("{label}_RET_{index}")),
        i(Dst::D, Cmp::A),
        // jump to compare code block
        lbl_sym(label),
        ij(Cmp::Zero, Jmp::JMP),
        // return address label
        Op::Label(format!("{label}_RET_{index}")),
    ]
}

/// Generate a set of instructions for boolean comparing the top two stack values.
/// Stores the return address from register `D` into `R15`.
/// Then will pop the two values and push a `Cmp::Neg1` (true) value when `when_false_jump_critera` is _NOT_ met. Otherwise will push `Cmp::Zero` (false).
/// Then will jump back to the address stored in `R15`.
pub fn bool_stack_compare(
    label: &'static str,
    return_label: &'static str,
    when_false_jump_critera: Jmp,
) -> [Op; 18] {
    [
        lbl(label),
        // return address storage
        sym(R15),
        i(Dst::M, Cmp::D),
        // stack pop
        sym(SP),
        i(Dst::A | Dst::M, Cmp::MNeg1),
        i(Dst::D, Cmp::M),
        i(Dst::A, Cmp::ANeg1),
        // find diff from popped value to current stack value
        i(Dst::D, Cmp::MNegD),
        // default zero (false) result and jump to return if meets criteria
        // (happy path should avoid jump)
        i(Dst::M, Cmp::Zero),
        lbl_sym(return_label),
        ij(Cmp::D, when_false_jump_critera),
        // set result to -1 (true) if not jumping
        sym(SP),
        i(Dst::A, Cmp::MNeg1),
        i(Dst::M, Cmp::Neg1),
        lbl(return_label),
        // back to return address
        sym(R15),
        i(Dst::A, Cmp::M),
        ij(Cmp::Zero, Jmp::JMP),
    ]
}
