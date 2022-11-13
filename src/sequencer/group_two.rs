use crate::{
    instructions::Instructions::{self, *},
    registers::IndexedReg,
};
use std::vec;

use super::{ADDR_MODE_MASK, OPCODE_MASK};

const ASL: u8 = 0b000;
const ROL: u8 = 0b001;
const LSR: u8 = 0b010;
const ROR: u8 = 0b011;
const STX: u8 = 0b100;
const LDX: u8 = 0b101;
const DEC: u8 = 0b110;
const INC: u8 = 0b111;

const IM: u8 = 0b000;
const ZP: u8 = 0b001;
const ACC: u8 = 0b010;
const A: u8 = 0b011;
const ZP_X: u8 = 0b101;
const A_X: u8 = 0b111;

pub fn get_seqeunce(instruction: u8) -> Option<Vec<Instructions>> {
    let opcode = (instruction & OPCODE_MASK) >> 5;
    let addr_mode = (instruction & ADDR_MODE_MASK) >> 2;
    let mut sequence = vec![];

    match addr_mode {
        ZP => sequence.push(LoadLowerAddr),
        ZP_X => {
            sequence.push(LoadLowerAddr);
            let mut reg = IndexedReg::X;
            if opcode == STX || opcode == LDX {
                reg = IndexedReg::Y;
            }
            sequence.push(AddToAddrBus(reg));
        }
        A => {
            sequence.push(LoadLowerAddr);
            sequence.push(LoadHigherAddr);
        }
        A_X => {
            sequence.push(LoadLowerAddr);
            sequence.push(LoadHigherAddr);
            let mut reg = IndexedReg::X;
            if opcode == LDX {
                reg = IndexedReg::Y;
            }
            sequence.push(AddToAddrBus(reg));
        }
        ACC | IM => sequence.push(Idle),
        _ => return None,
    }

    match (opcode, addr_mode) {
        (DEC, ACC) | (STX, ACC) | (LDX, ACC) => return None,
        (ASL, ACC) => sequence.push(ShiftLeftOneBit(false)),
        (ASL, _) => sequence.push(ShiftLeftOneBit(true)),
        (ROL, ACC) => sequence.push(RotateLeftOneBit(false)),
        (ROL, _) => sequence.push(RotateLeftOneBit(true)),
        (LSR, ACC) => sequence.push(ShiftRightOneBit(false)),
        (LSR, _) => sequence.push(ShiftRightOneBit(true)),
        (ROR, ACC) => sequence.push(RotateRightOneBit(false)),
        (ROR, _) => sequence.push(RotateRightOneBit(true)),
        (STX, _) => sequence.push(StoreToAddr(IndexedReg::X)),
        (LDX, IM) => sequence.push(LoadImmediate(IndexedReg::X)),
        (LDX, _) => sequence.push(LoadFromAddr(IndexedReg::X)),
        (DEC, _) => {
            sequence.push(LoadToAlu(true));
            sequence.push(DecAlu);
            sequence.push(StoreAlu);
        }
        (INC, _) => {
            sequence.push(LoadToAlu(true));
            sequence.push(IncAlu);
            sequence.push(StoreAlu);
        }
        _ => return None,
    }

    Some(sequence)
}
