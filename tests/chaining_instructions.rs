use rem6502::cpu::CPU;

#[test]
fn test_ldx_stx() {
    let mut cpu = CPU::new();

    // LDX #$34
    cpu.write_byte(0x0, 0xA2);
    cpu.write_byte(0x1, 0x34);

    // STX $12
    cpu.write_byte(0x2, 0x86);
    cpu.write_byte(0x3, 0x12);

    cpu.run_loop();

    assert_eq!(cpu.read_byte(0x12), 0x34);
    assert_eq!(cpu.get_registers().get_pc(), 0x4);
}

#[test]
fn test_ldx_inx_stx() {
    let mut cpu = CPU::new();

    // LDX #$34
    cpu.write_byte(0x0, 0xA2);
    cpu.write_byte(0x1, 0x34);

    // INX
    cpu.write_byte(0x2, 0xE8);

    // STX $12
    cpu.write_byte(0x3, 0x86);
    cpu.write_byte(0x4, 0x12);

    cpu.run_loop();

    assert_eq!(cpu.read_byte(0x12), 0x35);
    assert_eq!(cpu.get_registers().get_pc(), 0x5);
}

#[test]
fn test_ldx_inc_lda() {
    let mut cpu = CPU::new();

    // LDX #$12
    cpu.write_byte(0x0, 0xA2);
    cpu.write_byte(0x1, 0x12);

    // INC $1830,X
    cpu.write_byte(0x2, 0xFE);
    cpu.write_byte(0x3, 0x30);
    cpu.write_byte(0x4, 0x18);

    // LDA $1830,X
    cpu.write_byte(0x5, 0xBD);
    cpu.write_byte(0x6, 0x30);
    cpu.write_byte(0x7, 0x18);

    // Prepare memory
    cpu.write_byte(0x1842, 0x44);

    cpu.run_loop();

    assert_eq!(cpu.get_registers().get_a(), 0x45);
    assert_eq!(cpu.get_registers().get_pc(), 0x8);
}

#[test]
fn test_lda_adc() {
    let mut cpu = CPU::new();

    // LDA #$35
    cpu.write_byte(0x0, 0xA9);
    cpu.write_byte(0x1, 0x35);

    // ADC #$30
    cpu.write_byte(0x2, 0x69);
    cpu.write_byte(0x3, 0x30);

    cpu.run_loop();

    assert_eq!(cpu.get_registers().get_a(), 0x65);
    assert_eq!(cpu.get_registers().get_pc(), 0x4);
}

#[test]
fn test_loop_inx() {
    let mut cpu = CPU::new();

    // INX
    cpu.write_byte(0x0, 0xE8);

    // CPX #$32
    cpu.write_byte(0x1, 0xE0);
    cpu.write_byte(0x2, 0x32);

    // BNE
    cpu.write_byte(0x3, 0xD0);
    cpu.write_byte(0x4, 0b1111_1011);

    cpu.run_loop();

    assert_eq!(cpu.get_registers().get_x(), 0x32);
    assert_eq!(cpu.get_registers().get_pc(), 0x5);
}

#[test]
fn test_inx_jmp_inx() {
    let mut cpu = CPU::new();

    // INX
    cpu.write_byte(0x0, 0xE8);

    // JMP $4402
    cpu.write_byte(0x1, 0x4C);
    cpu.write_byte(0x2, 0x02);
    cpu.write_byte(0x3, 0x44);

    // INX
    cpu.write_byte(0x4402, 0xE8);

    cpu.run_loop();

    assert_eq!(cpu.get_registers().get_x(), 0x2);
    assert_eq!(cpu.get_registers().get_pc(), 0x4403);
}