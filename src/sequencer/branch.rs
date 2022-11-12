use crate::instructions::Instructions::{self, *};
use std::vec;

use super::OPCODE_MASK;

const REST_MASK: u8 = 0b00011111;

const BPL: u8 = 0b000;
const BMI: u8 = 0b001;
const BVC: u8 = 0b010;
const BVS: u8 = 0b011;
const BCC: u8 = 0b100;
const BCS: u8 = 0b101;
const BNE: u8 = 0b110;
const BEQ: u8 = 0b111;

pub fn get_seqeunce(instruction: u8) -> Option<Vec<Instructions>> {
    if instruction & REST_MASK != 0b10000 {
        return None;
    }
    let opcode = (instruction & OPCODE_MASK) >> 5;
    let mut sequence = vec![];

    match opcode {
        _ => return None,
    };

    Some(sequence)
}
