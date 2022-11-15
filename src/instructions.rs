use crate::{
    memory::Memory,
    registers::{Flag, IndexedReg, Registers},
    Byte, Word, SP,
};

#[derive(Debug)]
pub enum Instructions {
    Idle,
    MemToDataBus(bool),
    RegToDataBus(IndexedReg),
    DataBusToReg(IndexedReg),
    AluToDataBus,
    DataBusToMem(bool),
    CompareWithReg(IndexedReg),
    LoadLowerAddr,
    LoadHigherAddr,
    AddToAddrBus(IndexedReg),
    AddToReg(IndexedReg),
    SubFromReg(IndexedReg),
    ORWithReg(IndexedReg),
    ANDWithReg(IndexedReg),
    XORWithReg(IndexedReg),
    LoadTempLowerAddr(bool),
    LoadTempHigherAddr(bool),
    ShiftLeftReg,
    ShiftLeftDataBus,
    ShiftRightReg,
    ShiftRightDataBus,
    RotateLeftReg,
    RotateLeftDataBus,
    RotateRightReg,
    RotateRightDataBus,
    IncReg(IndexedReg),
    DecReg(IndexedReg),
    TransferReg(IndexedReg, IndexedReg),
    PullToReg(IndexedReg),
    PushFromReg(IndexedReg),
    SetFlags(Vec<Flag>),
    ClearFlags(Vec<Flag>),
    DataBusToAlu,
    IncAlu,
    DecAlu,
    IncPC,
    AddToPC,
    MoveAddrToPc,
    LoadStackPointer,
    PushHigherPC,
    PushLowerPC,
    PullLowerPC,
    PullHigherPC,
    PullToStatus,
    SetBitTestFlags,
}

pub struct InstructionExecutor<'a> {
    mem: &'a mut Memory,
    reg: &'a mut Registers,
    addr_bus: &'a mut Word,
    data_bus: &'a mut Byte,
    alu: &'a mut Byte,
}

impl<'a> InstructionExecutor<'a> {
    pub fn new(
        mem: &'a mut Memory,
        reg: &'a mut Registers,
        addr_bus: &'a mut Word,
        data_bus: &'a mut Byte,
        alu: &'a mut Byte,
    ) -> Self {
        Self {
            mem,
            reg,
            addr_bus,
            data_bus,
            alu,
        }
    }

    pub fn execute_instruction(&mut self, instruction: &Instructions) {
        match instruction {
            Instructions::MemToDataBus(use_addr_bus) => self.mem_to_data_bus(*use_addr_bus),
            Instructions::RegToDataBus(ind_reg) => self.reg_to_data_bus(ind_reg),
            Instructions::DataBusToReg(ind_reg) => self.data_bus_to_reg(ind_reg),
            Instructions::AluToDataBus => self.alu_to_data_bus(),
            Instructions::DataBusToMem(use_addr_bus) => self.data_bus_to_mem(*use_addr_bus),
            Instructions::CompareWithReg(ind_reg) => self.compare_with_reg(ind_reg),
            Instructions::LoadLowerAddr => self.load_lower_byte_to_addr_bus(),
            Instructions::LoadHigherAddr => self.load_higher_byte_to_addr_bus(false, false),
            Instructions::AddToAddrBus(ind_reg) => self.add_to_addr_bus(ind_reg),
            Instructions::AddToReg(ind_reg) => self.add_to_reg(ind_reg),
            Instructions::SubFromReg(ind_reg) => self.sub_from_reg(ind_reg),
            Instructions::ORWithReg(ind_reg) => self.or_with_reg(ind_reg),
            Instructions::ANDWithReg(ind_reg) => self.and_with_reg(ind_reg),
            Instructions::XORWithReg(ind_reg) => self.xor_with_reg(ind_reg),
            Instructions::LoadTempLowerAddr(use_addr_bus) => {
                self.load_lower_byte_to_alu(*use_addr_bus)
            }
            Instructions::LoadTempHigherAddr(use_addr_bus) => {
                self.load_higher_byte_to_addr_bus(true, *use_addr_bus)
            }
            Instructions::ShiftLeftReg => self.shift_left_reg(),
            Instructions::ShiftLeftDataBus => self.shift_left_data_bus(),
            Instructions::ShiftRightReg => self.shift_right_reg(),
            Instructions::ShiftRightDataBus => self.shift_right_data_bus(),
            Instructions::RotateLeftReg => self.rotate_left_reg(),
            Instructions::RotateLeftDataBus => self.rotate_left_data_bus(),
            Instructions::RotateRightReg => self.rotate_right_reg(),
            Instructions::RotateRightDataBus => self.rotate_right_data_bus(),
            Instructions::Idle => {}
            Instructions::IncReg(ind_reg) => self.inc_reg(ind_reg),
            Instructions::DecReg(ind_reg) => self.dec_reg(ind_reg),
            Instructions::TransferReg(from, to) => self.transfer_reg(from, to),
            Instructions::PullToReg(ind_reg) => self.pull_to_reg(ind_reg),
            Instructions::PushFromReg(ind_reg) => self.push_from_reg(ind_reg),
            Instructions::SetFlags(flags) => self.set_flags(flags),
            Instructions::ClearFlags(flags) => self.clear_flags(flags),
            Instructions::DataBusToAlu => self.data_bus_to_alu(),
            Instructions::IncAlu => self.inc_alu(),
            Instructions::DecAlu => self.dec_alu(),
            Instructions::IncPC => self.inc_pc(),
            Instructions::AddToPC => self.add_to_pc(),
            Instructions::MoveAddrToPc => self.load_addr_to_pc(),
            Instructions::LoadStackPointer => self.load_stack_pointer(),
            Instructions::PushHigherPC => self.push_higher_pc(),
            Instructions::PushLowerPC => self.push_lower_pc(),
            Instructions::PullHigherPC => self.pull_higher_pc(),
            Instructions::PullLowerPC => self.pull_lower_pc(),
            Instructions::PullToStatus => self.pull_to_status(),
            Instructions::SetBitTestFlags => self.set_bit_test_flags(),
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

    fn get_addr(&self, use_addr_bus: bool) -> Word {
        if use_addr_bus {
            *self.addr_bus
        } else {
            self.reg.get_pc()
        }
    }

    fn mem_to_data_bus(&mut self, use_addr_bus: bool) {
        *self.data_bus = self.mem.read_byte(self.get_addr(use_addr_bus));
    }

    fn reg_to_data_bus(&mut self, ind_reg: &IndexedReg) {
        *self.data_bus = self.get_reg(ind_reg);
    }

    fn data_bus_to_reg(&mut self, ind_reg: &IndexedReg) {
        *self.get_mut_reg(ind_reg) = *self.data_bus;
    }

    fn alu_to_data_bus(&mut self) {
        *self.data_bus = *self.alu;
    }

    fn data_bus_to_mem(&mut self, use_addr_bus: bool) {
        self.mem
            .write_byte(self.get_addr(use_addr_bus), *self.data_bus);
    }

    fn transfer_reg(&mut self, from: &IndexedReg, to: &IndexedReg) {
        let val = self.get_reg(from);
        let reg = self.get_mut_reg(to);
        *reg = val;
    }

    fn shift_left_data_bus(&mut self) {
        *self.alu = *self.data_bus << 1;
        self.reg.get_mut_p().c = *self.data_bus & 0x80 != 0x0;
    }

    fn shift_left_reg(&mut self) {
        let val = self.get_reg(&IndexedReg::A);
        *self.get_mut_reg(&IndexedReg::A) = val << 1;
        self.reg.get_mut_p().c = val & 0x80 != 0x0;
    }

    fn shift_right_data_bus(&mut self) {
        *self.alu = *self.data_bus >> 1;
        self.reg.get_mut_p().c = *self.data_bus & 0x01 != 0x0;
    }

    fn shift_right_reg(&mut self) {
        let val = self.get_reg(&IndexedReg::A);
        *self.get_mut_reg(&IndexedReg::A) = val >> 1;
        self.reg.get_mut_p().c = val & 0x01 != 0x0;
    }

    fn rotate_left_data_bus(&mut self) {
        *self.alu = self.data_bus.rotate_left(1);
        self.reg.get_mut_p().c = *self.data_bus & 0x80 != 0x0;
    }

    fn rotate_left_reg(&mut self) {
        let val = self.get_reg(&IndexedReg::A);
        *self.get_mut_reg(&IndexedReg::A) = val.rotate_left(1);
        self.reg.get_mut_p().c = val & 0x80 != 0x0;
    }

    fn rotate_right_data_bus(&mut self) {
        *self.alu = self.data_bus.rotate_right(1);
        self.reg.get_mut_p().c = *self.data_bus & 0x01 != 0x0;
    }

    fn rotate_right_reg(&mut self) {
        let val = self.get_reg(&IndexedReg::A);
        *self.get_mut_reg(&IndexedReg::A) = val.rotate_right(1);
        self.reg.get_mut_p().c = val & 0x01 != 0x0;
    }

    fn add_to_reg(&mut self, ind_reg: &IndexedReg) {
        let val = self.get_reg(ind_reg).wrapping_add(*self.data_bus);
        *self.get_mut_reg(ind_reg) = val;
        self.reg.set_flags(val);
    }

    fn sub_from_reg(&mut self, ind_reg: &IndexedReg) {
        let val = self.get_reg(ind_reg).wrapping_sub(*self.data_bus);
        *self.get_mut_reg(ind_reg) = val;
        self.reg.set_flags(val);
    }

    fn or_with_reg(&mut self, ind_reg: &IndexedReg) {
        let val = self.get_reg(ind_reg) | *self.data_bus;
        *self.get_mut_reg(ind_reg) = val;
        self.reg.set_flags(val);
    }

    fn and_with_reg(&mut self, ind_reg: &IndexedReg) {
        let val = self.get_reg(ind_reg) & *self.data_bus;
        *self.get_mut_reg(ind_reg) = val;
        self.reg.set_flags(val);
    }

    fn xor_with_reg(&mut self, ind_reg: &IndexedReg) {
        let val = self.get_reg(ind_reg) ^ *self.data_bus;
        *self.get_mut_reg(ind_reg) = val;
        self.reg.set_flags(val);
    }

    fn compare_with_reg(&mut self, ind_reg: &IndexedReg) {
        let lhs = self.get_reg(ind_reg);
        let rhs = *self.data_bus;
        self.reg.get_mut_p().c = lhs >= rhs;
        self.reg.get_mut_p().z = lhs == rhs;
        self.reg.get_mut_p().n = lhs < rhs;
    }

    fn load_lower_byte_to_alu(&mut self, use_addr_bus: bool) {
        let res = self.mem.read_byte(self.get_addr(use_addr_bus));
        *self.addr_bus = self.addr_bus.wrapping_add(1);
        *self.alu = res;
        if !use_addr_bus {
            self.reg.inc_pc();
        }
    }

    fn load_lower_byte_to_addr_bus(&mut self) {
        let res = self.mem.read_byte(self.reg.get_pc());
        *self.addr_bus = u16::from_le_bytes([res, 0x0]);
        self.reg.inc_pc();
    }

    fn load_higher_byte_to_addr_bus(&mut self, use_alu: bool, use_addr_bus: bool) {
        let res = self.mem.read_byte(self.get_addr(use_addr_bus));
        let addr_bytes = self.addr_bus.to_le_bytes();
        let l_byte = if use_alu { *self.alu } else { addr_bytes[0] };
        *self.addr_bus = u16::from_le_bytes([l_byte, res]);
        if !use_addr_bus {
            self.reg.inc_pc();
        }
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

    fn data_bus_to_alu(&mut self) {
        *self.alu = *self.data_bus;
        self.inc_pc();
    }

    fn inc_alu(&mut self) {
        // TODO rename
        *self.alu = self.data_bus.wrapping_add(1);
    }

    fn dec_alu(&mut self) {
        // TODO rename
        *self.alu = self.data_bus.wrapping_sub(1);
    }

    fn inc_pc(&mut self) {
        self.reg.inc_pc();
    }

    fn add_to_pc(&mut self) {
        *self.reg.get_mut_pc() = self.reg.get_pc().wrapping_add(*self.alu as u16);
    }

    fn load_addr_to_pc(&mut self) {
        *self.reg.get_mut_pc() = *self.addr_bus;
    }

    fn load_stack_pointer(&mut self) {
        *self.addr_bus = u16::from_le_bytes([self.reg.get_s(), SP]);
    }

    fn push_higher_pc(&mut self) {
        let val = self.reg.get_pc().to_le_bytes()[1];
        self.push(val);
    }

    fn push_lower_pc(&mut self) {
        let val = self.reg.get_pc().to_le_bytes()[0];
        self.push(val);
    }

    fn pull_higher_pc(&mut self) {
        let pc = self.reg.get_pc().to_le_bytes();
        let val = self.pull();
        *self.reg.get_mut_pc() = u16::from_le_bytes([pc[0], val]);
    }

    fn pull_lower_pc(&mut self) {
        let pc = self.reg.get_pc().to_le_bytes();
        let val = self.pull();
        *self.reg.get_mut_pc() = u16::from_le_bytes([val, pc[1]]);
    }

    fn pull_to_status(&mut self) {
        let p = self.pull();
        self.reg.set_p(p);
    }

    fn set_bit_test_flags(&mut self) {
        let val = *self.data_bus & self.get_reg(&IndexedReg::A);
        self.reg.get_mut_p().z = val == 0;
        self.reg.get_mut_p().n = (val & 0x80) >> 7 == 1;
        self.reg.get_mut_p().v = (val & 0x40) >> 6 == 1;
    }
}
