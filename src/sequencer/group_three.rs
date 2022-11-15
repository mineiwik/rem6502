use crate::{
    instructions::Instructions::{self, *},
    registers::IndexedReg,
};
use std::vec;

use super::{ADDR_MODE_MASK, OPCODE_MASK};

const BIT: u8 = 0b001;
const JMP: u8 = 0b010;
const JMP_ABS: u8 = 0b011;
const STY: u8 = 0b100;
const LDY: u8 = 0b101;
const CPY: u8 = 0b110;
const CPX: u8 = 0b111;

const IM: u8 = 0b000;
const ZP: u8 = 0b001;
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
        IM => {}
        _ => return None,
    }

    match (opcode, addr_mode) {
        (STY, _) => {
            sequence.push(RegToDataBus(IndexedReg::Y));
            sequence.push(DataBusToMem(true));
        }
        (LDY, IM) => {
            sequence.push(MemToDataBus(false));
            sequence.push(DataBusToReg(IndexedReg::Y));
        }
        (LDY, _) => {
            sequence.push(MemToDataBus(true));
            sequence.push(DataBusToReg(IndexedReg::Y));
        }

        (CPX, IM) => {
            sequence.push(MemToDataBus(false));
            sequence.push(CompareWithReg(IndexedReg::X));
        }
        (CPX, _) => {
            sequence.push(MemToDataBus(true));
            sequence.push(CompareWithReg(IndexedReg::X));
        }

        (CPY, IM) => {
            sequence.push(MemToDataBus(false));
            sequence.push(CompareWithReg(IndexedReg::Y));
        }
        (CPY, _) => {
            sequence.push(MemToDataBus(true));
            sequence.push(CompareWithReg(IndexedReg::Y));
        }

        (JMP, A) => sequence.push(MoveAddrToPc),

        (JMP_ABS, A) => {
            sequence.push(LoadTempLowerAddr(true));
            sequence.push(LoadTempHigherAddr(true));
            sequence.push(MoveAddrToPc);
        }

        (BIT, A) | (BIT, ZP) => {
            sequence.push(MemToDataBus(true));
            sequence.push(SetBitTestFlags);
        }
        _ => return None,
    }

    Some(sequence)
}
