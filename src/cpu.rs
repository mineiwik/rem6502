use crate::{
    constants::{Byte, Word},
    instructions::{InstructionExecutor, Instructions},
    memory::Memory,
    registers::Registers,
    sequencer,
};

pub struct CPU {
    mem: Memory,
    registers: Registers,
    addr_bus: Word,
    alu: Byte,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            mem: Memory::new(),
            registers: Registers::new(),
            addr_bus: 0x0,
            alu: 0x0,
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
            &mut self.alu,
        );
        instruction_executor.execute_instruction(instruction);
    }

    fn get_instruction(&mut self) -> Vec<Instructions> {
        let instruction = self.mem.read_byte(*self.registers.get_pc());
        self.registers.inc_pc();
        sequencer::get_seqeunce(instruction)
    }

    pub fn run(&mut self) {
        let instructions = self.get_instruction();
        for instruction in instructions {
            self.execute(&instruction);
            self.registers.inc_pc();
        }
    }
}
