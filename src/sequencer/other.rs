use crate::{
    instructions::Instructions::{self, *},
    registers::{Flag, IndexedReg},
};
use std::vec;

const BRK: u8 = 0x00;
const JSR_ABS: u8 = 0x20;
const RTI: u8 = 0x40;
const RTS: u8 = 0x60;
const PHP: u8 = 0x08;
const PLP: u8 = 0x28;
const PHA: u8 = 0x48;
const PLA: u8 = 0x68;
const DEY: u8 = 0x88;
const TAY: u8 = 0xA8;
const INY: u8 = 0xC8;
const INX: u8 = 0xE8;
const CLC: u8 = 0x18;
const SEC: u8 = 0x38;
const CLI: u8 = 0x58;
const SEI: u8 = 0x78;
const TYA: u8 = 0x98;
const CLV: u8 = 0xB8;
const CLD: u8 = 0xD8;
const SED: u8 = 0xF8;
const TXA: u8 = 0x8A;
const TXS: u8 = 0x9A;
const TAX: u8 = 0xAA;
const TSX: u8 = 0xBA;
const DEX: u8 = 0xCA;
const NOP: u8 = 0xEA;

pub fn get_seqeunce(instruction: u8) -> Option<Vec<Instructions>> {
    let mut sequence = vec![];

    sequence.push(Idle);

    match instruction {
        INX => sequence.push(IncReg(IndexedReg::X)),
        INY => sequence.push(IncReg(IndexedReg::Y)),
        DEX => sequence.push(DecReg(IndexedReg::X)),
        DEY => sequence.push(DecReg(IndexedReg::Y)),
        NOP => sequence.push(Idle),
        TAX => sequence.push(TransferReg(IndexedReg::A, IndexedReg::X)),
        TAY => sequence.push(TransferReg(IndexedReg::A, IndexedReg::Y)),
        TSX => sequence.push(TransferReg(IndexedReg::S, IndexedReg::X)),
        TXA => sequence.push(TransferReg(IndexedReg::X, IndexedReg::A)),
        TXS => sequence.push(TransferReg(IndexedReg::X, IndexedReg::S)),
        TYA => sequence.push(TransferReg(IndexedReg::Y, IndexedReg::A)),
        PLA => sequence.push(PullToReg(IndexedReg::A)),
        PHA => sequence.push(PushFromReg(IndexedReg::A)),
        PLP => sequence.push(PullToReg(IndexedReg::S)),
        PHP => sequence.push(PushFromReg(IndexedReg::S)),
        CLC => sequence.push(ClearFlags(vec![Flag::C])),
        CLD => sequence.push(ClearFlags(vec![Flag::D])),
        CLI => sequence.push(ClearFlags(vec![Flag::I])),
        CLV => sequence.push(ClearFlags(vec![Flag::V])),
        SEC => sequence.push(SetFlags(vec![Flag::C])),
        SED => sequence.push(SetFlags(vec![Flag::D])),
        SEI => sequence.push(SetFlags(vec![Flag::I])),
        _ => return None,
    };

    Some(sequence)
}
