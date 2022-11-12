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
            sequence.push(AddToAddrBus(IndexedReg::X));
        }
        A => {
            sequence.push(LoadLowerAddr);
            sequence.push(LoadHigherAddr);
        }
        A_X => {
            sequence.push(LoadLowerAddr);
            sequence.push(LoadHigherAddr);
            sequence.push(AddToAddrBus(IndexedReg::X));
        }
        ACC | IM => {}
        _ => return None,
    }

    match (opcode, addr_mode) {
        (DEC, ACC) | (STX, ACC) | (LDX, ACC) => return None,
        (STX, ZP_X) => {
            // ZP_Y
        }
        (LDX, ZP_X) => {
            // ZP_Y
        }
        (LDX, A_X) => {
            // A_Y
        }
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
            sequence.push(LoadToAlu);
            sequence.push(DecAlu);
            sequence.push(StoreAlu);
        }
        (INC, _) => {
            sequence.push(LoadToAlu);
            sequence.push(IncAlu);
            sequence.push(StoreAlu);
        }
        _ => return None,
    }

    Some(sequence)
}
