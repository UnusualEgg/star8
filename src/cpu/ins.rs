use std::{error::Error, slice::SliceIndex};

pub enum Reg {
    A,
    B,
    C,
    D,
}

fn from_val(val: u8) -> Option<Reg> {
    match val {
        0 => Some(Reg::A),
        1 => Some(Reg::B),
        2 => Some(Reg::C),
        3 => Some(Reg::D),
        _ => None,
    }
}

pub fn single_reg(ins: u8) -> Option<Reg> {
    let reg = ins & 0x0f;
    if !reg.is_power_of_two() {
        None
    } else {
        from_val(reg.trailing_zeros() as u8)
    }
}

pub fn double_reg(ins: u8) -> (Reg, Reg) {
    let regs = ins & 0b0000_1111;
    let reg1 = regs >> 2;
    let reg2 = regs & 0b11;
    (from_val(reg1).unwrap(), from_val(reg2).unwrap())
}

pub enum Condition {
    AEqZero,
    AEqB,
    Zero,
    Overflow,
    Neg,
}
pub fn conditon(ins: u8) -> Option<Condition> {
    match ins & 0x0f {
        0b0000 => Some(Condition::AEqB),
        0b1000 => Some(Condition::AEqB),
        0b0100 => Some(Condition::Zero),
        0b0010 => Some(Condition::Overflow),
        0b0001 => Some(Condition::Neg),
        _ => None,
    }
}
