use crate::{Byte, Word};

#[derive(Default)]
pub struct Status {
    pub n: bool,
    pub v: bool,
    pub b: bool,
    pub d: bool,
    pub i: bool,
    pub z: bool,
    pub c: bool,
}

pub struct Registers {
    pub a: Byte,
    pub x: Byte,
    pub y: Byte,
    pub ir: Byte,
    pc: Word,
    s: Byte,
    pub p: Status,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x0,
            x: 0x0,
            y: 0x0,
            ir: 0x0,
            pc: 0x0,
            s: 0xFF,
            p: Default::default(),
        }
    }

    pub fn set_p(&mut self, p: Byte) {
        self.p.c = (p & 0x1) >> 0 == 1;
        self.p.z = (p & 0x2) >> 1 == 1;
        self.p.i = (p & 0x4) >> 2 == 1;
        self.p.d = (p & 0x8) >> 3 == 1;
        self.p.b = (p & 0x10) >> 4 == 1;
        self.p.v = (p & 0x40) >> 6 == 1;
        self.p.n = (p & 0x80) >> 7 == 1;
    }

    pub fn get_p_byte(&self) -> Byte {
        let mut res = 0x0;
        res |= (self.p.c as u8) << 0;
        res |= (self.p.z as u8) << 1;
        res |= (self.p.i as u8) << 2;
        res |= (self.p.d as u8) << 3;
        res |= (self.p.b as u8) << 4;
        res |= (self.p.v as u8) << 6;
        res |= (self.p.n as u8) << 7;
        res
    }

    pub fn get_p(&self) -> &Status {
        &self.p
    }

    pub fn get_a(&self) -> Byte {
        self.a
    }

    pub fn get_x(&self) -> Byte {
        self.x
    }

    pub fn get_s(&self) -> Byte {
        self.s
    }

    pub fn get_y(&self) -> Byte {
        self.y
    }

    pub fn get_mut_pc(&mut self) -> &mut Word {
        &mut self.pc
    }

    pub fn get_mut_p(&mut self) -> &mut Status {
        &mut self.p
    }

    pub fn get_mut_a(&mut self) -> &mut Byte {
        &mut self.a
    }

    pub fn get_mut_x(&mut self) -> &mut Byte {
        &mut self.x
    }

    pub fn get_mut_y(&mut self) -> &mut Byte {
        &mut self.y
    }

    pub fn get_mut_s(&mut self) -> &mut Byte {
        &mut self.s
    }

    pub fn set_flags(&mut self, val: Byte) {
        self.p.z = val == 0x0;
        self.p.n = val >> 7 != 0x0;
    }

    pub fn inc_pc(&mut self) {
        self.pc = self.pc.wrapping_add(0b1);
    }

    pub fn jmp_pc(&mut self, new_pc: Word) {
        self.pc = new_pc;
    }

    pub fn get_pc(&self) -> Word {
        self.pc
    }
}

#[derive(Debug)]
pub enum IndexedReg {
    A,
    X,
    Y,
    S,
}

#[derive(Debug)]
pub enum Flag {
    C,
    Z,
    I,
    D,
    B,
    V,
    N,
}
