use crate::{
    instructions::{Instructions::{self, *}, AddrSource},
    memory::Memory,
    registers::Registers,
    Byte, Word,
};
use std::vec;

const REST_MASK: u8 = 0b00011111;
const BRANCH_MASK: u8 = 0b11000000;
const CMP_MASK: u8 = 0b00100000;

const NEGATIVE: u8 = 0b00;
const OVERFLOW: u8 = 0b01;
const CARRY: u8 = 0b10;
const ZERO: u8 = 0b11;

pub fn get_seqeunce(instruction: u8, reg: &Registers, mem: &Memory) -> Option<Vec<Instructions>> {
    if instruction & REST_MASK != 0b10000 {
        return None;
    }

    let branch = (instruction & BRANCH_MASK) >> 6;
    let comparator = (instruction & CMP_MASK) >> 5;
    let mut sequence = vec![];
    let status = reg.get_p();

    sequence.push(MemToDataBus(AddrSource::PC));
    sequence.push(Idle);

    let flag = match branch {
        NEGATIVE => status.n,
        OVERFLOW => status.v,
        CARRY => status.c,
        ZERO => status.z,
        _ => return None,
    };

    if flag as u8 != comparator {
        return Some(sequence);
    }
    sequence.push(AddToPC);
    if is_crossing_pb(reg.get_pc() + 0b1, mem.read_byte(reg.get_pc())) {
        sequence.push(Idle)
    }

    Some(sequence)
}

fn is_crossing_pb(pc: Word, rel: Byte) -> bool {
    let mut operand = rel as u16;
    if rel >> 7 == 1 {
        operand |= 0xFF00;
    }
    pc.wrapping_add(operand) >> 8 != pc >> 8
}
