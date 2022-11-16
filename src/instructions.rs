use crate::{
    memory::Memory,
    registers::{Flag, IndexedReg, Registers},
    Byte, Word, SP,
};

#[derive(Debug)]
pub enum Instructions {
    Idle,
    MemToDataBus(AddrSource),
    RegToDataBus(IndexedReg),
    DataBusToReg(IndexedReg),
    DataBusToMem(AddrSource),
    CompareWithReg(IndexedReg),
    LoadZPAddr,
    LoadAddr(AddrSource),
    AddToAddrBus(IndexedReg),
    AddToReg(IndexedReg),
    SubFromReg(IndexedReg),
    ORWithReg(IndexedReg),
    ANDWithReg(IndexedReg),
    XORWithReg(IndexedReg),
    IncReg(IndexedReg),
    DecReg(IndexedReg),
    TransferReg(IndexedReg, IndexedReg),
    PullToReg(IndexedReg),
    PushFromReg(IndexedReg),
    SetFlags(Flag),
    ClearFlags(Flag),
    IncDataBus,
    DecDataBus,
    IncPC,
    AddToPC,
    MoveAddrToPc,
    LoadStackPointer,
    PushPC,
    PullPC,
    PullToStatus,
    SetBitTestFlags,
    Shift(Direction, DataSource),
    Rotate(Direction, DataSource),
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right
}

#[derive(Debug)]
pub enum DataSource {
    DataBus,
    Reg
}

#[derive(Debug)]
pub enum AddrSource {
    AddrBus,
    PC
}

pub struct InstructionExecutor<'a> {
    mem: &'a mut Memory,
    reg: &'a mut Registers,
    addr_bus: &'a mut Word,
    data_bus: &'a mut Byte,
}

impl<'a> InstructionExecutor<'a> {
    pub fn new(
        mem: &'a mut Memory,
        reg: &'a mut Registers,
        addr_bus: &'a mut Word,
        data_bus: &'a mut Byte,
    ) -> Self {
        Self {
            mem,
            reg,
            addr_bus,
            data_bus,
        }
    }

    pub fn execute_instruction(&mut self, instruction: &Instructions) {
        match instruction {
            Instructions::Idle => {}
            Instructions::MemToDataBus(source) => self.mem_to_data_bus(source),
            Instructions::RegToDataBus(ind_reg) => self.reg_to_data_bus(ind_reg),
            Instructions::DataBusToReg(ind_reg) => self.data_bus_to_reg(ind_reg),
            Instructions::DataBusToMem(source) => self.data_bus_to_mem(source),
            Instructions::CompareWithReg(ind_reg) => self.compare_with_reg(ind_reg),
            Instructions::LoadZPAddr => self.load_zp_addr(),
            Instructions::LoadAddr(source) => self.load_addr(source),
            Instructions::AddToAddrBus(ind_reg) => self.add_to_addr_bus(ind_reg),
            Instructions::AddToReg(ind_reg) => self.add_to_reg(ind_reg),
            Instructions::SubFromReg(ind_reg) => self.sub_from_reg(ind_reg),
            Instructions::ORWithReg(ind_reg) => self.or_with_reg(ind_reg),
            Instructions::ANDWithReg(ind_reg) => self.and_with_reg(ind_reg),
            Instructions::XORWithReg(ind_reg) => self.xor_with_reg(ind_reg),
            Instructions::IncReg(ind_reg) => self.inc_reg(ind_reg),
            Instructions::DecReg(ind_reg) => self.dec_reg(ind_reg),
            Instructions::TransferReg(from, to) => self.transfer_reg(from, to),
            Instructions::PullToReg(ind_reg) => self.pull_to_reg(ind_reg),
            Instructions::PushFromReg(ind_reg) => self.push_from_reg(ind_reg),
            Instructions::SetFlags(flags) => self.set_flags(flags),
            Instructions::ClearFlags(flags) => self.clear_flags(flags),
            Instructions::IncDataBus => self.inc_data_bus(),
            Instructions::DecDataBus => self.dec_data_bus(),
            Instructions::IncPC => self.inc_pc(),
            Instructions::AddToPC => self.add_to_pc(),
            Instructions::MoveAddrToPc => self.move_addr_to_pc(),
            Instructions::LoadStackPointer => self.load_stack_pointer(),
            Instructions::PushPC => self.push_pc(),
            Instructions::PullPC => self.pull_pc(),
            Instructions::PullToStatus => self.pull_to_status(),
            Instructions::SetBitTestFlags => self.set_bit_test_flags(),
            Instructions::Shift(dir, source) => self.shift(dir, source),
            Instructions::Rotate(dir, source) => self.rotate(dir, source),
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

    fn get_addr(&self, source: &AddrSource) -> Word {
        match source {
            AddrSource::AddrBus => *self.addr_bus,
            AddrSource::PC => self.reg.get_pc()
        }
    }

    fn get_mut_addr(&mut self, source: &AddrSource) -> &mut Word {
        match source {
            AddrSource::AddrBus => &mut self.addr_bus,
            AddrSource::PC => self.reg.get_mut_pc()
        }
    }

    fn mem_to_data_bus(&mut self, source: &AddrSource) {
        *self.data_bus = self.mem.read_byte(self.get_addr(source));
        if let AddrSource::PC  = source {
            self.reg.inc_pc();
        }
    }

    fn reg_to_data_bus(&mut self, ind_reg: &IndexedReg) {
        *self.data_bus = self.get_reg(ind_reg);
    }

    fn data_bus_to_reg(&mut self, ind_reg: &IndexedReg) {
        *self.get_mut_reg(ind_reg) = *self.data_bus;
    }

    fn data_bus_to_mem(&mut self, source: &AddrSource) {
        self.mem
            .write_byte(self.get_addr(source), *self.data_bus);
    }

    fn transfer_reg(&mut self, from: &IndexedReg, to: &IndexedReg) {
        let val = self.get_reg(from);
        let reg = self.get_mut_reg(to);
        *reg = val;
    }

    fn shift_left_data_bus(&mut self) {
        self.reg.get_mut_p().c = *self.data_bus & 0x80 != 0x0;
        *self.data_bus = *self.data_bus << 1;
    }

    fn shift(&mut self, dir: &Direction, source: &DataSource) {
        match (dir, source) {
            (Direction::Left, DataSource::Reg) => self.shift_left_reg(),
            (Direction::Left, DataSource::DataBus) => self.shift_left_data_bus(),
            (Direction::Right, DataSource::Reg) => self.shift_right_reg(),
            (Direction::Right, DataSource::DataBus) => self.shift_right_data_bus(),
        }
    }

    fn rotate(&mut self, dir: &Direction, source: &DataSource) {
        match (dir, source) {
            (Direction::Left, DataSource::Reg) => self.rotate_left_reg(),
            (Direction::Left, DataSource::DataBus) => self.rotate_left_data_bus(),
            (Direction::Right, DataSource::Reg) => self.rotate_right_reg(),
            (Direction::Right, DataSource::DataBus) => self.rotate_right_data_bus(),
        }
    }

    fn shift_left_reg(&mut self) {
        let val = self.get_reg(&IndexedReg::A);
        *self.get_mut_reg(&IndexedReg::A) = val << 1;
        self.reg.get_mut_p().c = val & 0x80 != 0x0;
    }

    fn shift_right_data_bus(&mut self) {
        self.reg.get_mut_p().c = *self.data_bus & 0x01 != 0x0;
        *self.data_bus = *self.data_bus >> 1;
    }

    fn shift_right_reg(&mut self) {
        let val = self.get_reg(&IndexedReg::A);
        *self.get_mut_reg(&IndexedReg::A) = val >> 1;
        self.reg.get_mut_p().c = val & 0x01 != 0x0;
    }

    fn rotate_left_data_bus(&mut self) {
        self.reg.get_mut_p().c = *self.data_bus & 0x80 != 0x0;
        *self.data_bus = self.data_bus.rotate_left(1);
    }

    fn rotate_left_reg(&mut self) {
        let val = self.get_reg(&IndexedReg::A);
        *self.get_mut_reg(&IndexedReg::A) = val.rotate_left(1);
        self.reg.get_mut_p().c = val & 0x80 != 0x0;
    }

    fn rotate_right_data_bus(&mut self) {
        self.reg.get_mut_p().c = *self.data_bus & 0x01 != 0x0;
        *self.data_bus = self.data_bus.rotate_right(1);
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

    fn load_zp_addr(&mut self) {
        let l_byte = self.mem.read_byte(self.reg.get_pc());
        *self.reg.get_mut_pc() = self.reg.get_pc().wrapping_add(1);
        *self.addr_bus = u16::from_le_bytes([l_byte, 0]);
    }

    fn load_addr(&mut self, source: &AddrSource) {
        let l_byte = self.mem.read_byte(self.get_addr(source));
        *self.get_mut_addr(source) = self.get_addr(source).wrapping_add(1);
        let h_byte = self.mem.read_byte(self.get_addr(source));
        *self.get_mut_addr(source) = self.get_addr(source).wrapping_add(1);
        *self.addr_bus = u16::from_le_bytes([l_byte, h_byte]);
        println!("ADDR {}", *self.addr_bus);
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

    fn set_flags(&mut self, flag: &Flag) {
        let flag = self.get_mut_flag(flag);
        *flag = true;
    }

    fn clear_flags(&mut self, flag: &Flag) {
        let flag = self.get_mut_flag(flag);
        *flag = false;
    }

    fn inc_data_bus(&mut self) {
        *self.data_bus = self.data_bus.wrapping_add(1);
    }

    fn dec_data_bus(&mut self) {
        *self.data_bus = self.data_bus.wrapping_sub(1);
    }

    fn inc_pc(&mut self) {
        self.reg.inc_pc();
    }

    fn add_to_pc(&mut self) {
        let mut operand: u16 = *self.data_bus as u16;
        if *self.data_bus >> 7 == 1 {
            operand |= 0xFF00;
        }
        *self.reg.get_mut_pc() = self.reg.get_pc().wrapping_add(operand);
    }

    fn move_addr_to_pc(&mut self) {
        *self.reg.get_mut_pc() = *self.addr_bus;
    }

    fn load_stack_pointer(&mut self) {
        *self.addr_bus = u16::from_le_bytes([self.reg.get_s(), SP]);
    }

    fn push_pc(&mut self) {
        let pc = self.reg.get_pc().wrapping_add(1);
        let val = pc.to_le_bytes();
        self.push(val[1]);
        self.push(val[0]);
    }

    fn pull_pc(&mut self) {
        let l_byte = self.pull();
        let h_byte = self.pull();
        *self.reg.get_mut_pc() = u16::from_le_bytes([l_byte, h_byte]);
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
