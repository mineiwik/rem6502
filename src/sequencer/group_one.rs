use crate::{
    instructions::Instructions::{self, *},
    registers::IndexedReg,
};
use std::vec;

use super::{ADDR_MODE_MASK, OPCODE_MASK};

const ORA: u8 = 0b000;
const AND: u8 = 0b001;
const EOR: u8 = 0b010;
const ADC: u8 = 0b011;
const STA: u8 = 0b100;
const LDA: u8 = 0b101;
const CMP: u8 = 0b110;
const SBC: u8 = 0b111;

const ZP_X_IND: u8 = 0b000;
const ZP: u8 = 0b001;
const IM: u8 = 0b010;
const A: u8 = 0b011;
const ZP_Y_IND: u8 = 0b100;
const ZP_X: u8 = 0b101;
const A_Y: u8 = 0b110;
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
        ZP_X_IND => {
            sequence.push(LoadLowerAddr);
            sequence.push(AddToAddrBus(IndexedReg::X));
            sequence.push(LoadTempLowerAddr);
            sequence.push(LoadTempHigherAddr);
        }
        ZP_Y_IND => {
            sequence.push(LoadLowerAddr);
            sequence.push(LoadTempLowerAddr);
            sequence.push(LoadTempHigherAddr);
            sequence.push(AddToAddrBus(IndexedReg::Y));
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
        A_Y => {
            sequence.push(LoadLowerAddr);
            sequence.push(LoadHigherAddr);
            sequence.push(AddToAddrBus(IndexedReg::Y));
        }
        IM => sequence.push(Idle),
        _ => return None,
    }

    match (opcode, addr_mode) {
        (LDA, IM) => sequence.push(LoadImmediate(IndexedReg::A)),
        (LDA, _) => sequence.push(LoadFromAddr(IndexedReg::A)),

        (STA, _) => sequence.push(StoreToAddr(IndexedReg::A)),

        (ADC, IM) => sequence.push(AddImmediate(IndexedReg::A)),
        (ADC, _) => sequence.push(AddFromAddr(IndexedReg::A)),

        (SBC, IM) => sequence.push(SubImmediate(IndexedReg::A)),
        (SBC, _) => sequence.push(SubFromAddr(IndexedReg::A)),

        (ORA, IM) => sequence.push(ORImmediate(IndexedReg::A)),
        (ORA, _) => sequence.push(ORFromAddr(IndexedReg::A)),

        (AND, IM) => sequence.push(ANDImmediate(IndexedReg::A)),
        (AND, _) => sequence.push(ANDFromAddr(IndexedReg::A)),

        (EOR, IM) => sequence.push(XORImmediate(IndexedReg::A)),
        (EOR, _) => sequence.push(XORFromAddr(IndexedReg::A)),

        (CMP, IM) => sequence.push(CmpImmediate(IndexedReg::A)),
        (CMP, _) => sequence.push(CmpFromAddr(IndexedReg::A)),
        _ => return None,
    }

    Some(sequence)
}
