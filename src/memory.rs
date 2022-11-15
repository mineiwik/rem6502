use crate::{Byte, Word, MEMORY_LENGTH};

pub struct Memory {
    data: Vec<Byte>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: vec![0; MEMORY_LENGTH],
        }
    }

    pub fn read_byte(&self, addr: Word) -> Byte {
        *self.data.get(addr as usize).unwrap()
    }

    pub fn write_byte(&mut self, addr: Word, val: Byte) {
        *self.data.get_mut(addr as usize).unwrap() = val;
    }
}
