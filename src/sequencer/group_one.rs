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
            sequence.push(LoadTempLowerAddr(true));
            sequence.push(LoadTempHigherAddr(true));
        }
        ZP_Y_IND => {
            sequence.push(LoadLowerAddr);
            sequence.push(LoadTempLowerAddr(true));
            sequence.push(LoadTempHigherAddr(true));
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
        IM => {}
        _ => return None,
    }

    match (opcode, addr_mode) {
        (LDA, IM) => {
            sequence.push(MemToDataBus(false));
            sequence.push(DataBusToReg(IndexedReg::A));
        }
        (LDA, _) => {
            sequence.push(MemToDataBus(true));
            sequence.push(DataBusToReg(IndexedReg::A));
        }

        (STA, _) => {
            sequence.push(RegToDataBus(IndexedReg::A));
            sequence.push(DataBusToMem(true));
        }

        (ADC, IM) => {
            sequence.push(MemToDataBus(false));
            sequence.push(AddToReg(IndexedReg::A));
        }
        (ADC, _) => {
            sequence.push(MemToDataBus(true));
            sequence.push(AddToReg(IndexedReg::A));
        }

        (SBC, IM) => {
            sequence.push(MemToDataBus(false));
            sequence.push(SubFromReg(IndexedReg::A));
        }
        (SBC, _) => {
            sequence.push(MemToDataBus(true));
            sequence.push(SubFromReg(IndexedReg::A));
        }

        (ORA, IM) => {
            sequence.push(MemToDataBus(false));
            sequence.push(ORWithReg(IndexedReg::A));
        }
        (ORA, _) => {
            sequence.push(MemToDataBus(true));
            sequence.push(ORWithReg(IndexedReg::A));
        }

        (AND, IM) => {
            sequence.push(MemToDataBus(false));
            sequence.push(ANDWithReg(IndexedReg::A));
        }
        (AND, _) => {
            sequence.push(MemToDataBus(true));
            sequence.push(ANDWithReg(IndexedReg::A));
        }

        (EOR, IM) => {
            sequence.push(MemToDataBus(false));
            sequence.push(XORWithReg(IndexedReg::A));
        }
        (EOR, _) => {
            sequence.push(MemToDataBus(true));
            sequence.push(XORWithReg(IndexedReg::A));
        }

        (CMP, IM) => {
            sequence.push(MemToDataBus(false));
            sequence.push(CompareWithReg(IndexedReg::A));
        }
        (CMP, _) => {
            sequence.push(MemToDataBus(true));
            sequence.push(CompareWithReg(IndexedReg::A));
        }
        _ => return None,
    }

    Some(sequence)
}
