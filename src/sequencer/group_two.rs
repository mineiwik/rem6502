use crate::{
    instructions::{Instructions::{self, *}, Direction, DataSource, AddrSource},
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
        ZP => sequence.push(LoadZPAddr),
        ZP_X => {
            sequence.push(LoadZPAddr);
            let mut reg = IndexedReg::X;
            if opcode == STX || opcode == LDX {
                reg = IndexedReg::Y;
            }
            sequence.push(AddToAddrBus(reg));
        }
        A => {
            sequence.push(LoadAddr(AddrSource::PC));
            sequence.push(Idle);
        }
        A_X => {
            sequence.push(LoadAddr(AddrSource::PC));
            sequence.push(Idle);
            let mut reg = IndexedReg::X;
            if opcode == LDX {
                reg = IndexedReg::Y;
            }
            sequence.push(AddToAddrBus(reg));
        }
        ACC => sequence.push(Idle),
        IM => {}
        _ => return None,
    }

    match (opcode, addr_mode) {
        (DEC, ACC) | (STX, ACC) | (LDX, ACC) => return None,

        (ASL, ACC) => sequence.push(Shift(Direction::Left, DataSource::Reg)),
        (ASL, _) => {
            sequence.push(MemToDataBus(AddrSource::AddrBus));
            sequence.push(Shift(Direction::Left, DataSource::DataBus));
            sequence.push(Idle);
            sequence.push(DataBusToMem(AddrSource::AddrBus));
        }

        (ROL, ACC) => sequence.push(Rotate(Direction::Left, DataSource::Reg)),
        (ROL, _) => {
            sequence.push(MemToDataBus(AddrSource::AddrBus));
            sequence.push(Rotate(Direction::Left, DataSource::DataBus));
            sequence.push(Idle);
            sequence.push(DataBusToMem(AddrSource::AddrBus));
        }

        (LSR, ACC) => sequence.push(Shift(Direction::Right, DataSource::Reg)),
        (LSR, _) => {
            sequence.push(MemToDataBus(AddrSource::AddrBus));
            sequence.push(Shift(Direction::Right, DataSource::DataBus));
            sequence.push(Idle);
            sequence.push(DataBusToMem(AddrSource::AddrBus));
        }

        (ROR, ACC) => sequence.push(Rotate(Direction::Right, DataSource::Reg)),
        (ROR, _) => {
            sequence.push(MemToDataBus(AddrSource::AddrBus));
            sequence.push(Rotate(Direction::Right, DataSource::DataBus));
            sequence.push(Idle);
            sequence.push(DataBusToMem(AddrSource::AddrBus));
        }

        (STX, _) => {
            sequence.push(RegToDataBus(IndexedReg::X));
            sequence.push(DataBusToMem(AddrSource::AddrBus));
        }

        (LDX, IM) => {
            sequence.push(MemToDataBus(AddrSource::PC));
            sequence.push(DataBusToReg(IndexedReg::X));
        }
        (LDX, _) => {
            sequence.push(MemToDataBus(AddrSource::AddrBus));
            sequence.push(DataBusToReg(IndexedReg::X));
        }

        (DEC, _) => {
            sequence.push(MemToDataBus(AddrSource::AddrBus));
            sequence.push(DecDataBus);
            sequence.push(Idle);
            sequence.push(DataBusToMem(AddrSource::AddrBus));
        }

        (INC, _) => {
            sequence.push(MemToDataBus(AddrSource::AddrBus));
            sequence.push(IncDataBus);
            sequence.push(Idle);
            sequence.push(DataBusToMem(AddrSource::AddrBus));
        }
        
        _ => return None,
    }

    Some(sequence)
}
