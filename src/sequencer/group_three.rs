use crate::{
    instructions::{Instructions::{self, *}, AddrSource},
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
        ZP => sequence.push(LoadZPAddr),
        ZP_X => {
            sequence.push(LoadZPAddr);
            sequence.push(AddToAddrBus(IndexedReg::X));
        }
        A => {
            sequence.push(LoadAddr(AddrSource::PC));
            sequence.push(Idle);
        }
        A_X => {
            sequence.push(LoadAddr(AddrSource::PC));
            sequence.push(Idle);
            sequence.push(AddToAddrBus(IndexedReg::X));
        }
        IM => {}
        _ => return None,
    }

    match (opcode, addr_mode) {
        (STY, _) => {
            sequence.push(RegToDataBus(IndexedReg::Y));
            sequence.push(DataBusToMem(AddrSource::AddrBus));
        }
        (LDY, IM) => {
            sequence.push(MemToDataBus(AddrSource::PC));
            sequence.push(DataBusToReg(IndexedReg::Y));
        }
        (LDY, _) => {
            sequence.push(MemToDataBus(AddrSource::AddrBus));
            sequence.push(DataBusToReg(IndexedReg::Y));
        }

        (CPX, IM) => {
            sequence.push(MemToDataBus(AddrSource::PC));
            sequence.push(CompareWithReg(IndexedReg::X));
        }
        (CPX, _) => {
            sequence.push(MemToDataBus(AddrSource::AddrBus));
            sequence.push(CompareWithReg(IndexedReg::X));
        }

        (CPY, IM) => {
            sequence.push(MemToDataBus(AddrSource::PC));
            sequence.push(CompareWithReg(IndexedReg::Y));
        }
        (CPY, _) => {
            sequence.push(MemToDataBus(AddrSource::AddrBus));
            sequence.push(CompareWithReg(IndexedReg::Y));
        }

        (JMP, A) => sequence.push(MoveAddrToPc),

        (JMP_ABS, A) => {
            sequence.push(LoadAddr(AddrSource::AddrBus));
            sequence.push(Idle);
            sequence.push(MoveAddrToPc);
        }

        (BIT, A) | (BIT, ZP) => {
            sequence.push(MemToDataBus(AddrSource::AddrBus));
            sequence.push(SetBitTestFlags);
        }
        _ => return None,
    }

    Some(sequence)
}
