use std::vec;

use crate::{instructions::Instructions, memory::Memory, registers::Registers};

mod branch;
mod group_one;
mod group_three;
mod group_two;
mod other;

const GROUP_ONE: u8 = 0b01;
const GROUP_TWO: u8 = 0b10;
const GROUP_THREE: u8 = 0b00;

const OPCODE_MASK: u8 = 0b11100000;
const ADDR_MODE_MASK: u8 = 0b00011100;
const OPCODE_GROUP_MASK: u8 = 0b00000011;

pub fn get_seqeunce(instruction: u8, reg: &Registers, mem: &Memory) -> Vec<Instructions> {
    if let Some(res) = get_group_sequence(instruction) {
        return res;
    }

    if let Some(res) = get_branch_sequence(instruction, reg, mem) {
        return res;
    }

    if let Some(res) = get_other_sequence(instruction) {
        return res;
    }

    vec![]
}

fn get_group_sequence(instruction: u8) -> Option<Vec<Instructions>> {
    let opcode_group: u8 = instruction & OPCODE_GROUP_MASK;
    match opcode_group {
        GROUP_ONE => group_one::get_seqeunce(instruction),
        GROUP_TWO => group_two::get_seqeunce(instruction),
        GROUP_THREE => group_three::get_seqeunce(instruction),
        _ => None,
    }
}

fn get_branch_sequence(
    instruction: u8,
    reg: &Registers,
    mem: &Memory,
) -> Option<Vec<Instructions>> {
    branch::get_seqeunce(instruction, reg, mem)
}

fn get_other_sequence(instruction: u8) -> Option<Vec<Instructions>> {
    other::get_seqeunce(instruction)
}
