use crate::{
    instructions::{InstructionExecutor, Instructions},
    memory::Memory,
    registers::Registers,
    sequencer, Byte, Word,
};

pub struct CPU {
    mem: Memory,
    registers: Registers,
    addr_bus: Word,
    data_bus: Byte,
    cycles: usize,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            mem: Memory::new(),
            registers: Registers::new(),
            addr_bus: 0x0,
            data_bus: 0x0,
            cycles: 0x1,
        }
    }

    pub fn write_byte(&mut self, addr: Word, val: Byte) {
        self.mem.write_byte(addr, val);
    }

    pub fn read_byte(&self, addr: Word) -> Byte {
        self.mem.read_byte(addr)
    }

    pub fn get_registers(&mut self) -> &mut Registers {
        &mut self.registers
    }

    pub fn execute(&mut self, instruction: &Instructions) {
        let mut instruction_executor = InstructionExecutor::new(
            &mut self.mem,
            &mut self.registers,
            &mut self.addr_bus,
            &mut self.data_bus,
        );
        instruction_executor.execute_instruction(instruction);
    }

    fn get_instruction(&mut self) -> Vec<Instructions> {
        let instruction = self.mem.read_byte(self.registers.get_pc());
        self.registers.inc_pc();
        sequencer::get_seqeunce(instruction, &self.registers, &self.mem)
    }

    pub fn run(&mut self) {
        let instructions = self.get_instruction();
        for instruction in instructions {
            self.cycles += 1;
            self.execute(&instruction);
        }
    }

    pub fn run_loop(&mut self) {
        loop {
            if self.mem.read_byte(self.registers.get_pc()) == 0 {
                break;
            }
            self.run();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lda_im() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xA9);
        cpu.write_byte(0x1, 0x34);

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x34);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
        assert_eq!(cpu.cycles, 3);
    }

    #[test]
    fn lda_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xA5);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x34, 0x68);

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x68);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn lda_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xAD);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x64);
        cpu.write_byte(0x6434, 0x24);

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x24);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
        assert_eq!(cpu.cycles, 5);
    }

    #[test]
    fn lda_zp_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xB5);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x0084, 0x32);
        *cpu.get_registers().get_mut_x() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x32);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
        assert_eq!(cpu.cycles, 5);
    }

    #[test]
    fn lda_zp_x_ind() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xA1);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x0084, 0x32);
        cpu.write_byte(0x0085, 0x33);
        cpu.write_byte(0x3332, 0x31);
        *cpu.get_registers().get_mut_x() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x31);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
        assert_eq!(cpu.cycles, 7);
    }

    #[test]
    fn lda_zp_y_ind() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xB1);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x0034, 0x32);
        cpu.write_byte(0x0035, 0x33);
        cpu.write_byte(0x3382, 0x28);
        *cpu.get_registers().get_mut_y() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x28);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
        //assert_eq!(cpu.cycles, 6);
    }

    #[test]
    fn lda_a_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xBD);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x64);
        cpu.write_byte(0x6434, 0x24);
        cpu.write_byte(0x6484, 0x32);
        *cpu.get_registers().get_mut_x() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x32);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
        //assert_eq!(cpu.cycles, 5);
    }

    #[test]
    fn lda_a_y() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xB9);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x64);
        cpu.write_byte(0x6434, 0x24);
        cpu.write_byte(0x6484, 0x32);
        *cpu.get_registers().get_mut_y() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x32);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
        //assert_eq!(cpu.cycles, 5);
    }

    #[test]
    fn adc_im() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x69);
        cpu.write_byte(0x1, 0x34);
        *cpu.get_registers().get_mut_a() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x84);
    }

    #[test]
    fn adc_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x65);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x34, 0x72);
        *cpu.get_registers().get_mut_a() = 0x14;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x86);
    }

    #[test]
    fn adc_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x6D);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x24);
        cpu.write_byte(0x2434, 0x23);
        *cpu.get_registers().get_mut_a() = 0x32;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x55);
    }

    #[test]
    fn adc_zp_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x75);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x36, 0x23);
        *cpu.get_registers().get_mut_x() = 0x02;
        *cpu.get_registers().get_mut_a() = 0x32;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x55);
    }

    #[test]
    fn adc_a_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x7D);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x64);
        cpu.write_byte(0x6484, 0x32);
        *cpu.get_registers().get_mut_x() = 0x50;
        *cpu.get_registers().get_mut_a() = 0x32;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x64);
    }

    #[test]
    fn adc_a_y() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x79);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x64);
        cpu.write_byte(0x6484, 0x32);
        *cpu.get_registers().get_mut_y() = 0x50;
        *cpu.get_registers().get_mut_a() = 0x32;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x64);
    }

    #[test]
    fn sub_im() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xE9);
        cpu.write_byte(0x1, 0x34);
        *cpu.get_registers().get_mut_a() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x1C);
    }

    #[test]
    fn sub_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xE5);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x34, 0x14);
        *cpu.get_registers().get_mut_a() = 0x72;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x5E);
    }

    #[test]
    fn sub_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xED);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x24);
        cpu.write_byte(0x2434, 0x23);
        *cpu.get_registers().get_mut_a() = 0x32;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0xF);
    }

    #[test]
    fn sub_zp_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xF5);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x36, 0x23);
        *cpu.get_registers().get_mut_x() = 0x02;
        *cpu.get_registers().get_mut_a() = 0x32;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0xF);
    }

    #[test]
    fn sub_a_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xFD);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x64);
        cpu.write_byte(0x6484, 0x32);
        *cpu.get_registers().get_mut_x() = 0x50;
        *cpu.get_registers().get_mut_a() = 0x34;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x2);
    }

    #[test]
    fn sub_a_y() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xF9);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x64);
        cpu.write_byte(0x6484, 0x32);
        *cpu.get_registers().get_mut_y() = 0x50;
        *cpu.get_registers().get_mut_a() = 0x33;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x1);
    }

    #[test]
    fn ora_im() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x09);
        cpu.write_byte(0x1, 0x34);
        *cpu.get_registers().get_mut_a() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x34 | 0x50);
    }

    #[test]
    fn ora_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x05);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x34, 0x28);
        *cpu.get_registers().get_mut_a() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x28 | 0x50);
    }

    #[test]
    fn or_zp_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x15);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x37, 0x28);
        *cpu.get_registers().get_mut_a() = 0x50;
        *cpu.get_registers().get_mut_x() = 0x3;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x28 | 0x50);
    }

    #[test]
    fn or_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x0D);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x38);
        cpu.write_byte(0x3834, 0x23);
        *cpu.get_registers().get_mut_a() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x23 | 0x50);
    }

    #[test]
    fn or_a_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x1D);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x38);
        cpu.write_byte(0x3847, 0x23);
        *cpu.get_registers().get_mut_a() = 0x50;
        *cpu.get_registers().get_mut_x() = 0x13;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x23 | 0x50);
    }

    #[test]
    fn or_a_y() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x19);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x38);
        cpu.write_byte(0x3847, 0x23);
        *cpu.get_registers().get_mut_a() = 0x50;
        *cpu.get_registers().get_mut_y() = 0x13;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x23 | 0x50);
    }

    #[test]
    fn and_im() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x29);
        cpu.write_byte(0x1, 0x34);
        *cpu.get_registers().get_mut_a() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x34 & 0x50);
    }

    #[test]
    fn and_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x25);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x34, 0x28);
        *cpu.get_registers().get_mut_a() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x28 & 0x50);
    }

    #[test]
    fn and_zp_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x35);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x37, 0x28);
        *cpu.get_registers().get_mut_a() = 0x50;
        *cpu.get_registers().get_mut_x() = 0x3;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x28 & 0x50);
    }

    #[test]
    fn and_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x2D);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x38);
        cpu.write_byte(0x3834, 0x23);
        *cpu.get_registers().get_mut_a() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x23 & 0x50);
    }

    #[test]
    fn and_a_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x3D);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x38);
        cpu.write_byte(0x3847, 0x23);
        *cpu.get_registers().get_mut_a() = 0x50;
        *cpu.get_registers().get_mut_x() = 0x13;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x23 & 0x50);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
    }

    #[test]
    fn and_a_y() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x39);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x38);
        cpu.write_byte(0x3847, 0x23);
        *cpu.get_registers().get_mut_a() = 0x50;
        *cpu.get_registers().get_mut_y() = 0x13;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x23 & 0x50);
    }

    #[test]
    fn eor_im() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x49);
        cpu.write_byte(0x1, 0x34);
        *cpu.get_registers().get_mut_a() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x34 ^ 0x50);
    }

    #[test]
    fn eor_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x45);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x34, 0x28);
        *cpu.get_registers().get_mut_a() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x28 ^ 0x50);
    }

    #[test]
    fn eor_zp_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x55);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x37, 0x28);
        *cpu.get_registers().get_mut_a() = 0x50;
        *cpu.get_registers().get_mut_x() = 0x3;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x28 ^ 0x50);
    }

    #[test]
    fn eor_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x4D);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x38);
        cpu.write_byte(0x3834, 0x23);
        *cpu.get_registers().get_mut_a() = 0x50;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x23 ^ 0x50);
    }

    #[test]
    fn eor_a_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x5D);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x38);
        cpu.write_byte(0x3847, 0x23);
        *cpu.get_registers().get_mut_a() = 0x50;
        *cpu.get_registers().get_mut_x() = 0x13;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x23 ^ 0x50);
    }

    #[test]
    fn eor_a_y() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x59);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x38);
        cpu.write_byte(0x3847, 0x23);
        *cpu.get_registers().get_mut_a() = 0x50;
        *cpu.get_registers().get_mut_y() = 0x13;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x23 ^ 0x50);
    }

    #[test]
    fn sta_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x85);
        cpu.write_byte(0x1, 0x34);
        *cpu.get_registers().get_mut_a() = 0x95;

        cpu.run();

        assert_eq!(cpu.read_byte(0x34), 0x95);
    }

    #[test]
    fn sta_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x8D);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x22);
        *cpu.get_registers().get_mut_a() = 0x92;

        cpu.run();

        assert_eq!(cpu.read_byte(0x2234), 0x92);
    }

    #[test]
    fn sta_a_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x9D);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x22);
        *cpu.get_registers().get_mut_a() = 0x92;
        *cpu.get_registers().get_mut_x() = 0x24;

        cpu.run();

        assert_eq!(cpu.read_byte(0x2234 + 0x24), 0x92);
    }

    #[test]
    fn sta_a_y() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x99);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x22);
        *cpu.get_registers().get_mut_a() = 0x92;
        *cpu.get_registers().get_mut_y() = 0x35;

        cpu.run();

        assert_eq!(cpu.read_byte(0x2234 + 0x35), 0x92);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
    }

    #[test]
    fn sta_zp_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x95);
        cpu.write_byte(0x1, 0x34);
        *cpu.get_registers().get_mut_a() = 0x92;
        *cpu.get_registers().get_mut_x() = 0x35;

        cpu.run();

        assert_eq!(cpu.read_byte(0x34 + 0x35), 0x92);
    }

    #[test]
    fn sta_zp_x_ind() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x81);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x69, 0x12);
        cpu.write_byte(0x6A, 0x14);
        *cpu.get_registers().get_mut_a() = 0x92;
        *cpu.get_registers().get_mut_x() = 0x35;

        cpu.run();

        assert_eq!(cpu.read_byte(0x1412), 0x92);
    }

    #[test]
    fn sta_zp_y_ind() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x91);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x34, 0x12);
        cpu.write_byte(0x35, 0x14);
        *cpu.get_registers().get_mut_a() = 0x92;
        *cpu.get_registers().get_mut_y() = 0x31;

        cpu.run();

        assert_eq!(cpu.read_byte(0x1412 + 0x31), 0x92);
    }

    #[test]
    fn cmp_im() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xC9);
        cpu.write_byte(0x1, 0x34);
        *cpu.get_registers().get_mut_a() = 0x34;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, false);
        assert_eq!(cpu.get_registers().get_p().z, true);
        assert_eq!(cpu.get_registers().get_p().c, true);
    }

    #[test]
    fn cmp_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xC5);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x34, 0x34);
        *cpu.get_registers().get_mut_a() = 0x34;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, false);
        assert_eq!(cpu.get_registers().get_p().z, true);
        assert_eq!(cpu.get_registers().get_p().c, true);
    }

    #[test]
    fn cmp_zp_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xD5);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x68, 0x34);
        *cpu.get_registers().get_mut_a() = 0x34;
        *cpu.get_registers().get_mut_x() = 0x34;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, false);
        assert_eq!(cpu.get_registers().get_p().z, true);
        assert_eq!(cpu.get_registers().get_p().c, true);
    }

    #[test]
    fn cmp_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xCD);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x38);
        cpu.write_byte(0x3834, 0x34);
        *cpu.get_registers().get_mut_a() = 0x34;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, false);
        assert_eq!(cpu.get_registers().get_p().z, true);
        assert_eq!(cpu.get_registers().get_p().c, true);
    }

    #[test]
    fn cmp_a_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xDD);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x38);
        cpu.write_byte(0x3868, 0x34);
        *cpu.get_registers().get_mut_a() = 0x34;
        *cpu.get_registers().get_mut_x() = 0x34;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, false);
        assert_eq!(cpu.get_registers().get_p().z, true);
        assert_eq!(cpu.get_registers().get_p().c, true);
    }

    #[test]
    fn cmp_a_y() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xD9);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x38);
        cpu.write_byte(0x3866, 0x34);
        *cpu.get_registers().get_mut_a() = 0x34;
        *cpu.get_registers().get_mut_y() = 0x32;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, false);
        assert_eq!(cpu.get_registers().get_p().z, true);
        assert_eq!(cpu.get_registers().get_p().c, true);
    }

    #[test]
    fn cpx_im() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xE0);
        cpu.write_byte(0x1, 0x34);
        *cpu.get_registers().get_mut_x() = 0x34;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, false);
        assert_eq!(cpu.get_registers().get_p().z, true);
        assert_eq!(cpu.get_registers().get_p().c, true);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
        assert_eq!(cpu.cycles, 3);
    }

    #[test]
    fn cpx_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xE4);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x34, 0x33);
        *cpu.get_registers().get_mut_x() = 0x33;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, false);
        assert_eq!(cpu.get_registers().get_p().z, true);
        assert_eq!(cpu.get_registers().get_p().c, true);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn cpx_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xEC);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x16);
        cpu.write_byte(0x1634, 0x33);
        *cpu.get_registers().get_mut_x() = 0x33;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, false);
        assert_eq!(cpu.get_registers().get_p().z, true);
        assert_eq!(cpu.get_registers().get_p().c, true);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
        assert_eq!(cpu.cycles, 5);
    }

    #[test]
    fn cpy_im() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xC0);
        cpu.write_byte(0x1, 0x34);
        *cpu.get_registers().get_mut_y() = 0x34;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, false);
        assert_eq!(cpu.get_registers().get_p().z, true);
        assert_eq!(cpu.get_registers().get_p().c, true);
    }

    #[test]
    fn asl_acc() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x0A);
        *cpu.get_registers().get_mut_a() = 0x34;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x68);
        assert_eq!(cpu.get_registers().get_p().c, false);
    }

    #[test]
    fn asl_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x0E);
        cpu.write_byte(0x1, 0x12);
        cpu.write_byte(0x2, 0x13);
        cpu.write_byte(0x1312, 0x85);

        cpu.run();

        assert_eq!(cpu.read_byte(0x1312), 0x0A);
        assert_eq!(cpu.get_registers().get_p().c, true);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
    }

    #[test]
    fn lsr_acc() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x4A);
        *cpu.get_registers().get_mut_a() = 0x34;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x1A);
        assert_eq!(cpu.get_registers().get_p().c, false);
    }

    #[test]
    fn lsr_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x4E);
        cpu.write_byte(0x1, 0x12);
        cpu.write_byte(0x2, 0x13);
        cpu.write_byte(0x1312, 0x85);

        cpu.run();

        assert_eq!(cpu.read_byte(0x1312), 0x42);
        assert_eq!(cpu.get_registers().get_p().c, true);
    }

    #[test]
    fn rol_acc() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x2A);
        *cpu.get_registers().get_mut_a() = 0x34;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x68);
        assert_eq!(cpu.get_registers().get_p().c, false);
    }

    #[test]
    fn rol_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x2E);
        cpu.write_byte(0x1, 0x12);
        cpu.write_byte(0x2, 0x13);
        cpu.write_byte(0x1312, 0x85);

        cpu.run();

        assert_eq!(cpu.read_byte(0x1312), 0x0B);
        assert_eq!(cpu.get_registers().get_p().c, true);
    }

    #[test]
    fn ror_acc() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x6A);
        *cpu.get_registers().get_mut_a() = 0x34;

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x1A);
        assert_eq!(cpu.get_registers().get_p().c, false);
        assert_eq!(cpu.get_registers().get_pc(), 0x1);
    }

    #[test]
    fn ror_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x6E);
        cpu.write_byte(0x1, 0x12);
        cpu.write_byte(0x2, 0x13);
        cpu.write_byte(0x1312, 0x85);

        cpu.run();

        assert_eq!(cpu.read_byte(0x1312), 0xC2);
        assert_eq!(cpu.get_registers().get_p().c, true);
    }

    #[test]
    fn stx_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x86);
        cpu.write_byte(0x1, 0x12);
        *cpu.get_registers().get_mut_x() = 0x14;

        cpu.run();

        assert_eq!(cpu.read_byte(0x12), 0x14);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn stx_zp_y() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x96);
        cpu.write_byte(0x1, 0x12);
        *cpu.get_registers().get_mut_x() = 0x14;
        *cpu.get_registers().get_mut_y() = 0x22;

        cpu.run();

        assert_eq!(cpu.read_byte(0x34), 0x14);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
        assert_eq!(cpu.cycles, 5);
    }

    #[test]
    fn stx_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x8E);
        cpu.write_byte(0x1, 0x12);
        cpu.write_byte(0x2, 0x13);
        *cpu.get_registers().get_mut_x() = 0x14;

        cpu.run();

        assert_eq!(cpu.read_byte(0x1312), 0x14);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
        assert_eq!(cpu.cycles, 5);
    }

    #[test]
    fn sty_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x84);
        cpu.write_byte(0x1, 0x12);
        *cpu.get_registers().get_mut_y() = 0x14;

        cpu.run();

        assert_eq!(cpu.read_byte(0x12), 0x14);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn sty_zp_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x94);
        cpu.write_byte(0x1, 0x12);
        *cpu.get_registers().get_mut_y() = 0x14;
        *cpu.get_registers().get_mut_x() = 0x22;

        cpu.run();

        assert_eq!(cpu.read_byte(0x34), 0x14);
    }

    #[test]
    fn sty_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x8C);
        cpu.write_byte(0x1, 0x12);
        cpu.write_byte(0x2, 0x13);
        *cpu.get_registers().get_mut_y() = 0x14;

        cpu.run();

        assert_eq!(cpu.read_byte(0x1312), 0x14);
    }

    #[test]
    fn ldx_im() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xA2);
        cpu.write_byte(0x1, 0x34);

        cpu.run();

        assert_eq!(cpu.get_registers().get_x(), 0x34);
        assert_eq!(cpu.cycles, 3);
    }

    #[test]
    fn ldx_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xA6);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x34, 0x68);

        cpu.run();

        assert_eq!(cpu.get_registers().get_x(), 0x68);
    }

    #[test]
    fn ldx_zp_y() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xB6);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x58, 0x68);
        *cpu.get_registers().get_mut_y() = 0x24;

        cpu.run();

        assert_eq!(cpu.get_registers().get_x(), 0x68);
    }

    #[test]
    fn ldx_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xAE);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x64);
        cpu.write_byte(0x6434, 0x24);

        cpu.run();

        assert_eq!(cpu.get_registers().get_x(), 0x24);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
    }

    #[test]
    fn ldx_a_y() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xBE);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x64);
        cpu.write_byte(0x645B, 0x24);
        *cpu.get_registers().get_mut_y() = 0x27;

        cpu.run();

        assert_eq!(cpu.get_registers().get_x(), 0x24);
    }

    #[test]
    fn ldy_im() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xA0);
        cpu.write_byte(0x1, 0x34);

        cpu.run();

        assert_eq!(cpu.get_registers().get_y(), 0x34);
    }

    #[test]
    fn ldy_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xA4);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x34, 0x68);

        cpu.run();

        assert_eq!(cpu.get_registers().get_y(), 0x68);
    }

    #[test]
    fn ldy_zp_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xB4);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x58, 0x68);
        *cpu.get_registers().get_mut_x() = 0x24;

        cpu.run();

        assert_eq!(cpu.get_registers().get_y(), 0x68);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
    }

    #[test]
    fn ldy_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xAC);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x64);
        cpu.write_byte(0x6434, 0x24);

        cpu.run();

        assert_eq!(cpu.get_registers().get_y(), 0x24);
    }

    #[test]
    fn ldy_a_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xBC);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x2, 0x64);
        cpu.write_byte(0x645B, 0x24);
        *cpu.get_registers().get_mut_x() = 0x27;

        cpu.run();

        assert_eq!(cpu.get_registers().get_y(), 0x24);
    }

    #[test]
    fn inx() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xE8);
        *cpu.get_registers().get_mut_x() = 0x41;

        cpu.run();

        assert_eq!(cpu.get_registers().get_x(), 0x42);
        assert_eq!(cpu.get_registers().get_pc(), 0x1);
    }

    #[test]
    fn iny() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xC8);
        *cpu.get_registers().get_mut_y() = 0x41;

        cpu.run();

        assert_eq!(cpu.get_registers().get_y(), 0x42);
    }

    #[test]
    fn dex() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xCA);
        *cpu.get_registers().get_mut_x() = 0x43;

        cpu.run();

        assert_eq!(cpu.get_registers().get_x(), 0x42);
    }

    #[test]
    fn dey() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x88);
        *cpu.get_registers().get_mut_y() = 0x43;

        cpu.run();

        assert_eq!(cpu.get_registers().get_y(), 0x42);
        assert_eq!(cpu.get_registers().get_pc(), 0x1);
    }

    #[test]
    fn inc_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xE6);
        cpu.write_byte(0x1, 0x32);
        cpu.write_byte(0x32, 0x44);

        cpu.run();

        assert_eq!(cpu.read_byte(0x32), 0x45);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
        assert_eq!(cpu.cycles, 6);
    }

    #[test]
    fn inc_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xEE);
        cpu.write_byte(0x1, 0x32);
        cpu.write_byte(0x2, 0x18);
        cpu.write_byte(0x1832, 0x44);

        cpu.run();

        assert_eq!(cpu.read_byte(0x1832), 0x45);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
        assert_eq!(cpu.cycles, 7);
    }

    #[test]
    fn inc_a_x() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xFE);
        cpu.write_byte(0x1, 0x30);
        cpu.write_byte(0x2, 0x18);
        cpu.write_byte(0x1832, 0x44);
        *cpu.registers.get_mut_x() = 0x2;

        cpu.run();

        assert_eq!(cpu.read_byte(0x1832), 0x45);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
        assert_eq!(cpu.cycles, 8);
    }

    #[test]
    fn dec_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xC6);
        cpu.write_byte(0x1, 0x32);
        cpu.write_byte(0x32, 0x44);

        cpu.run();

        assert_eq!(cpu.read_byte(0x32), 0x43);
    }

    #[test]
    fn dec_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xCE);
        cpu.write_byte(0x1, 0x32);
        cpu.write_byte(0x2, 0x18);
        cpu.write_byte(0x1832, 0x44);

        cpu.run();

        assert_eq!(cpu.read_byte(0x1832), 0x43);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
    }

    #[test]
    fn beq_do_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xF0);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.z = true;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x34);
    }

    #[test]
    fn beq_do_not_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xF0);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.z = false;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x2);
    }

    #[test]
    fn bne_do_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xD0);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.z = false;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x34);
    }

    #[test]
    fn bne_do_not_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xD0);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.z = true;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x2);
    }

    #[test]
    fn bcs_do_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xB0);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.c = true;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x34);
    }

    #[test]
    fn bcs_do_not_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xB0);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.c = false;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x2);
    }

    #[test]
    fn bcc_do_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x90);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.c = false;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x34);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn bcc_do_branch_with_page_crossing() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x90);
        cpu.write_byte(0x1, 0xFD);
        cpu.registers.p.c = false;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0xFFFF);
        assert_eq!(cpu.cycles, 5);
    }

    #[test]
    fn bcc_do_not_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x90);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.c = true;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x2);
        assert_eq!(cpu.cycles, 3);
    }

    #[test]
    fn bvs_do_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x70);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.v = true;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x34);
    }

    #[test]
    fn bvs_do_not_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x70);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.v = false;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x2);
    }

    #[test]
    fn bvc_do_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x50);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.v = false;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x34);
    }

    #[test]
    fn bvc_do_not_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x50);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.v = true;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x2);
    }

    #[test]
    fn bmi_do_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x30);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.n = true;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x34);
    }

    #[test]
    fn bmi_do_not_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x30);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.n = false;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x2);
    }

    #[test]
    fn bpl_do_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x10);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.n = false;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x34);
    }

    #[test]
    fn bpl_do_not_branch() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x10);
        cpu.write_byte(0x1, 0x32);
        cpu.registers.p.n = true;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x2);
    }

    #[test]
    fn jmp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x4C);
        cpu.write_byte(0x1, 0x32);
        cpu.write_byte(0x2, 0x24);

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x2432);
    }

    #[test]
    fn jmp_abs() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x6C);
        cpu.write_byte(0x1, 0x32);
        cpu.write_byte(0x2, 0x24);
        cpu.write_byte(0x2432, 0x18);
        cpu.write_byte(0x2433, 0x76);

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x7618);
    }

    #[test]
    fn jsr() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x20);
        cpu.write_byte(0x1, 0x32);
        cpu.write_byte(0x2, 0x24);

        cpu.run();

        assert_eq!(cpu.mem.read_byte(0x01FE), 0x02);
        assert_eq!(cpu.mem.read_byte(0x01FF), 0x00);
        assert_eq!(cpu.get_registers().get_pc(), 0x2432);
    }

    #[test]
    fn rts() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x60);
        cpu.write_byte(0x01fe, 0x24);
        cpu.write_byte(0x01ff, 0x32);
        *cpu.registers.get_mut_s() = 0xFD;

        cpu.run();

        assert_eq!(cpu.get_registers().get_pc(), 0x3225);
    }

    #[test]
    fn rti() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x40);
        cpu.write_byte(0x01fd, 0xD4);
        cpu.write_byte(0x01fe, 0x22);
        cpu.write_byte(0x01ff, 0x30);
        *cpu.registers.get_mut_s() = 0xFC;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p_byte(), 0xD4);
        assert_eq!(cpu.get_registers().get_pc(), 0x3022);
    }

    #[test]
    fn bit_zp_not_zero() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x24);
        cpu.write_byte(0x1, 0xD4);
        cpu.write_byte(0xD4, 0xF2);
        *cpu.registers.get_mut_a() = 0xFC;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, true);
        assert_eq!(cpu.get_registers().get_p().z, false);
        assert_eq!(cpu.get_registers().get_p().v, true);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
    }

    #[test]
    fn bit_zp_zero() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x24);
        cpu.write_byte(0x1, 0xD4);
        cpu.write_byte(0xD4, 0x12);
        *cpu.registers.get_mut_a() = 0x0;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, false);
        assert_eq!(cpu.get_registers().get_p().z, true);
        assert_eq!(cpu.get_registers().get_p().v, false);
        assert_eq!(cpu.get_registers().get_pc(), 0x2);
    }

    #[test]
    fn bit_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x2C);
        cpu.write_byte(0x1, 0xD4);
        cpu.write_byte(0x2, 0x88);
        cpu.write_byte(0x88D4, 0xF2);
        *cpu.registers.get_mut_a() = 0xFC;

        cpu.run();

        assert_eq!(cpu.get_registers().get_p().n, true);
        assert_eq!(cpu.get_registers().get_p().z, false);
        assert_eq!(cpu.get_registers().get_p().v, true);
        assert_eq!(cpu.get_registers().get_pc(), 0x3);
    }
}
