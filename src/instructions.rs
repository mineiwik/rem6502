use crate::{
    constants::{Byte, Word, SP},
    memory::Memory,
    registers::{Flag, IndexedReg, Registers},
};

#[derive(Debug)]
pub enum Instructions {
    LoadImmediate(IndexedReg),
    LoadLowerAddr,
    LoadHigherAddr,
    LoadFromAddr(IndexedReg),
    StoreToAddr(IndexedReg),
    AddToAddrBus(IndexedReg),
    AddImmediate(IndexedReg),
    AddFromAddr(IndexedReg),
    SubImmediate(IndexedReg),
    SubFromAddr(IndexedReg),
    ORImmediate(IndexedReg),
    ORFromAddr(IndexedReg),
    ANDImmediate(IndexedReg),
    ANDFromAddr(IndexedReg),
    XORImmediate(IndexedReg),
    XORFromAddr(IndexedReg),
    CmpImmediate(IndexedReg),
    CmpFromAddr(IndexedReg),
    LoadTempLowerAddr,
    LoadTempHigherAddr,
    ShiftLeftOneBit(bool),
    ShiftRightOneBit(bool),
    RotateLeftOneBit(bool),
    RotateRightOneBit(bool),
    NoOp,
    IncReg(IndexedReg),
    DecReg(IndexedReg),
    TransferReg(IndexedReg, IndexedReg),
    PullToReg(IndexedReg),
    PushFromReg(IndexedReg),
    SetFlags(Vec<Flag>),
    ClearFlags(Vec<Flag>),
}

pub struct InstructionExecutor<'a> {
    mem: &'a mut Memory,
    reg: &'a mut Registers,
    addr_bus: &'a mut Word,
    alu_out: &'a mut Byte,
}

impl<'a> InstructionExecutor<'a> {
    pub fn new(
        mem: &'a mut Memory,
        reg: &'a mut Registers,
        addr_bus: &'a mut Word,
        alu_out: &'a mut Byte,
    ) -> Self {
        Self {
            mem,
            reg,
            addr_bus,
            alu_out,
        }
    }

    pub fn execute_instruction(&mut self, instruction: &Instructions) {
        match instruction {
            Instructions::LoadImmediate(ind_reg) => self.load_byte_to_reg(ind_reg, false),
            Instructions::LoadLowerAddr => self.load_lower_byte_to_addr_bus(),
            Instructions::LoadHigherAddr => self.load_higher_byte_to_addr_bus(false, false),
            Instructions::LoadFromAddr(ind_reg) => self.load_byte_to_reg(ind_reg, true),
            Instructions::StoreToAddr(ind_reg) => self.store_byte_from_reg(ind_reg, true),
            Instructions::AddToAddrBus(ind_reg) => self.add_to_addr_bus(ind_reg),
            Instructions::AddImmediate(ind_reg) => self.add_byte_to_reg(ind_reg, false),
            Instructions::AddFromAddr(ind_reg) => self.add_byte_to_reg(ind_reg, true),
            Instructions::SubImmediate(ind_reg) => self.sub_byte_from_reg(ind_reg, false),
            Instructions::SubFromAddr(ind_reg) => self.sub_byte_from_reg(ind_reg, true),
            Instructions::ORImmediate(ind_reg) => self.or_byte_with_reg(ind_reg, false),
            Instructions::ORFromAddr(ind_reg) => self.or_byte_with_reg(ind_reg, true),
            Instructions::ANDImmediate(ind_reg) => self.and_byte_with_reg(ind_reg, false),
            Instructions::ANDFromAddr(ind_reg) => self.and_byte_with_reg(ind_reg, true),
            Instructions::XORImmediate(ind_reg) => self.xor_byte_with_reg(ind_reg, false),
            Instructions::XORFromAddr(ind_reg) => self.xor_byte_with_reg(ind_reg, true),
            Instructions::CmpImmediate(ind_reg) => self.cmp_byte_with_reg(ind_reg, false),
            Instructions::CmpFromAddr(ind_reg) => self.cmp_byte_with_reg(ind_reg, true),
            Instructions::LoadTempLowerAddr => self.load_lower_byte_to_alu(),
            Instructions::LoadTempHigherAddr => self.load_higher_byte_to_addr_bus(true, true),
            Instructions::ShiftLeftOneBit(use_mem) => self.shift_left_one_bit(*use_mem),
            Instructions::ShiftRightOneBit(use_mem) => self.shift_right_one_bit(*use_mem),
            Instructions::RotateLeftOneBit(use_mem) => self.rotate_left_one_bit(*use_mem),
            Instructions::RotateRightOneBit(use_mem) => self.rotate_right_one_bit(*use_mem),
            Instructions::NoOp => {}
            Instructions::IncReg(ind_reg) => self.inc_reg(ind_reg),
            Instructions::DecReg(ind_reg) => self.dec_reg(ind_reg),
            Instructions::TransferReg(from, to) => self.transfer_reg(from, to),
            Instructions::PullToReg(ind_reg) => self.pull_to_reg(ind_reg),
            Instructions::PushFromReg(ind_reg) => self.push_from_reg(ind_reg),
            Instructions::SetFlags(flags) => self.set_flags(flags),
            Instructions::ClearFlags(flags) => self.clear_flags(flags),
        }
    }

    fn get_reg(&self, ind_reg: &IndexedReg) -> Byte {
        match ind_reg {
            IndexedReg::A => self.reg.get_a(),
            IndexedReg::X => self.reg.get_x(),
            IndexedReg::Y => self.reg.get_y(),
            IndexedReg::S => self.reg.get_s(),
        }
    }

    fn get_mut_reg(&mut self, ind_reg: &IndexedReg) -> &mut Byte {
        match ind_reg {
            IndexedReg::A => self.reg.get_mut_a(),
            IndexedReg::X => self.reg.get_mut_x(),
            IndexedReg::Y => self.reg.get_mut_y(),
            IndexedReg::S => self.reg.get_mut_s(),
        }
    }

    fn get_addr(&self, use_addr_bus: bool) -> Word {
        if use_addr_bus {
            *self.addr_bus
        } else {
            *self.reg.get_pc()
        }
    }

    fn get_val(&self, use_mem: bool) -> Byte {
        if use_mem {
            self.mem.read_byte(*self.addr_bus)
        } else {
            self.get_reg(&IndexedReg::A)
        }
    }

    fn shift_left_one_bit(&mut self, use_mem: bool) {
        let val = self.get_val(use_mem);
        let reg = self.reg.get_mut_a();
        *reg = val << 1;
        self.reg.get_mut_p().c = val & 0x80 != 0x0;
    }

    fn shift_right_one_bit(&mut self, use_mem: bool) {
        let val = self.get_val(use_mem);
        let reg = self.reg.get_mut_a();
        *reg = val >> 1;
        self.reg.get_mut_p().c = val & 0x01 != 0x0;
    }

    fn rotate_left_one_bit(&mut self, use_mem: bool) {
        let val = self.get_val(use_mem);
        let reg = self.reg.get_mut_a();
        *reg = val.rotate_left(1);
        self.reg.get_mut_p().c = val & 0x80 != 0x0;
    }

    fn rotate_right_one_bit(&mut self, use_mem: bool) {
        let val = self.get_val(use_mem);
        let reg = self.reg.get_mut_a();
        *reg = val.rotate_right(1);
        self.reg.get_mut_p().c = val & 0x01 != 0x0;
    }

    fn load_byte_to_reg(&mut self, ind_reg: &IndexedReg, use_addr_bus: bool) {
        let val = self.mem.read_byte(self.get_addr(use_addr_bus));
        let reg = self.get_mut_reg(ind_reg);
        *reg = val;
    }

    fn store_byte_from_reg(&mut self, ind_reg: &IndexedReg, use_addr_bus: bool) {
        self.mem
            .write_byte(self.get_addr(use_addr_bus), self.get_reg(ind_reg))
    }

    fn add_byte_to_reg(&mut self, ind_reg: &IndexedReg, use_addr_bus: bool) {
        let val = self.mem.read_byte(self.get_addr(use_addr_bus));
        let reg = self.get_mut_reg(ind_reg);
        let val = reg.wrapping_add(val);
        *reg = val;
        self.reg.set_flags(val);
    }

    fn sub_byte_from_reg(&mut self, ind_reg: &IndexedReg, use_addr_bus: bool) {
        let val = self.mem.read_byte(self.get_addr(use_addr_bus));
        let reg = self.get_mut_reg(ind_reg);
        let val = reg.wrapping_sub(val);
        *reg = val;
        self.reg.set_flags(val);
    }

    fn or_byte_with_reg(&mut self, ind_reg: &IndexedReg, use_addr_bus: bool) {
        let val = self.mem.read_byte(self.get_addr(use_addr_bus));
        let reg = self.get_mut_reg(ind_reg);
        let val = *reg | val;
        *reg = val;
        self.reg.set_flags(val);
    }

    fn and_byte_with_reg(&mut self, ind_reg: &IndexedReg, use_addr_bus: bool) {
        let val = self.mem.read_byte(self.get_addr(use_addr_bus));
        let reg = self.get_mut_reg(ind_reg);
        let val = *reg & val;
        *reg = val;
        self.reg.set_flags(val);
    }

    fn xor_byte_with_reg(&mut self, ind_reg: &IndexedReg, use_addr_bus: bool) {
        let val = self.mem.read_byte(self.get_addr(use_addr_bus));
        let reg = self.get_mut_reg(ind_reg);
        let val = *reg ^ val;
        *reg = val;
        self.reg.set_flags(val);
    }

    fn cmp_byte_with_reg(&mut self, ind_reg: &IndexedReg, use_addr_bus: bool) {
        let lhs = self.get_reg(ind_reg);
        let rhs = self.mem.read_byte(self.get_addr(use_addr_bus));
        self.reg.get_mut_p().c = lhs >= rhs;
        self.reg.get_mut_p().z = lhs == rhs;
        self.reg.get_mut_p().n = lhs < rhs;
    }

    fn load_lower_byte_to_alu(&mut self) {
        let res = self.mem.read_byte(*self.addr_bus);
        *self.addr_bus = self.addr_bus.wrapping_add(1);
        *self.alu_out = res;
    }

    fn load_lower_byte_to_addr_bus(&mut self) {
        let res = self.mem.read_byte(*self.reg.get_pc());
        *self.addr_bus = u16::from_le_bytes([res, 0x0]);
    }

    fn load_higher_byte_to_addr_bus(&mut self, use_alu: bool, use_addr_bus: bool) {
        let res = self.mem.read_byte(self.get_addr(use_addr_bus));
        let addr_bytes = self.addr_bus.to_le_bytes();
        let l_byte = if use_alu {
            *self.alu_out
        } else {
            addr_bytes[0]
        };
        *self.addr_bus = u16::from_le_bytes([l_byte, res]);
    }

    fn add_to_addr_bus(&mut self, ind_reg: &IndexedReg) {
        *self.addr_bus = self.addr_bus.wrapping_add(u16::from(self.get_reg(ind_reg)));
    }

    fn inc_reg(&mut self, ind_reg: &IndexedReg) {
        let reg = self.get_mut_reg(ind_reg);
        *reg = reg.wrapping_add(1);
    }

    fn dec_reg(&mut self, ind_reg: &IndexedReg) {
        let reg = self.get_mut_reg(ind_reg);
        *reg = reg.wrapping_sub(1);
    }

    fn transfer_reg(&mut self, from: &IndexedReg, to: &IndexedReg) {
        let val = self.get_reg(from);
        let reg = self.get_mut_reg(to);
        *reg = val;
    }

    fn push(&mut self, val: Byte) {
        let addr = self.reg.get_s();
        self.mem.write_byte(u16::from_le_bytes([addr, SP]), val);
        *self.reg.get_mut_s() -= 0b1;
    }

    fn pull(&mut self) -> Byte {
        *self.reg.get_mut_s() += 0b1;
        let addr = self.reg.get_s();
        self.mem.read_byte(u16::from_le_bytes([addr, SP]))
    }

    fn push_from_reg(&mut self, ind_reg: &IndexedReg) {
        let val = self.get_reg(ind_reg);
        self.push(val);
    }

    fn pull_to_reg(&mut self, ind_reg: &IndexedReg) {
        let val = self.pull();
        let reg = self.get_mut_reg(ind_reg);
        *reg = val;
    }

    fn get_mut_flag(&mut self, flag: &Flag) -> &mut bool {
        match flag {
            Flag::C => &mut self.reg.get_mut_p().c,
            Flag::Z => &mut self.reg.get_mut_p().z,
            Flag::I => &mut self.reg.get_mut_p().i,
            Flag::D => &mut self.reg.get_mut_p().d,
            Flag::B => &mut self.reg.get_mut_p().b,
            Flag::V => &mut self.reg.get_mut_p().v,
            Flag::N => &mut self.reg.get_mut_p().n,
        }
    }

    fn get_flag(&mut self, flag: &Flag) -> &bool {
        match flag {
            Flag::C => &self.reg.get_mut_p().c,
            Flag::Z => &self.reg.get_mut_p().z,
            Flag::I => &self.reg.get_mut_p().i,
            Flag::D => &self.reg.get_mut_p().d,
            Flag::B => &self.reg.get_mut_p().b,
            Flag::V => &self.reg.get_mut_p().v,
            Flag::N => &self.reg.get_mut_p().n,
        }
    }

    fn set_flags(&mut self, flags: &Vec<Flag>) {
        for flag in flags {
            let flag = self.get_mut_flag(flag);
            *flag = true;
        }
    }

    fn clear_flags(&mut self, flags: &Vec<Flag>) {
        for flag in flags {
            let flag = self.get_mut_flag(flag);
            *flag = false;
        }
    }
}
