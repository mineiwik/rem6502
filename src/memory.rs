use crate::{constants::*, Word};

pub struct Memory {
    data: Vec<Byte>
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: vec![0; MEMORY_LENGTH]
        }
    }

    pub fn read_zp_byte(&self, addr: Byte) -> Byte {
        self.read_byte(u16::from_le_bytes([addr, ZP]))
    }

    pub fn read_byte(&self, addr: Word) -> Byte {
        *self.data.get(addr as usize).unwrap()
    }

    pub fn write_zp_byte(&mut self, addr: Byte, val: Byte) {
        self.write_byte(u16::from_le_bytes([addr, ZP]), val);
    }

    pub fn write_byte(&mut self, addr: Word, val: Byte) {
        *self.data.get_mut(addr as usize).unwrap() = val;
    }
}