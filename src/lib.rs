mod constants;
pub mod cpu;
mod instructions;
mod memory;
mod registers;
mod sequencer;

use constants::*;

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    #[test]
    fn lda_im() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xA9);
        cpu.write_byte(0x1, 0x34);

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x34);
    }

    #[test]
    fn lda_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xA5);
        cpu.write_byte(0x1, 0x34);
        cpu.write_byte(0x34, 0x68);

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0x68);
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

        assert_eq!(cpu.get_registers().get_a(), 0x0A);
        assert_eq!(cpu.get_registers().get_p().c, true);
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

        assert_eq!(cpu.get_registers().get_a(), 0x42);
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

        assert_eq!(cpu.get_registers().get_a(), 0x0B);
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
    }

    #[test]
    fn ror_a() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x6E);
        cpu.write_byte(0x1, 0x12);
        cpu.write_byte(0x2, 0x13);
        cpu.write_byte(0x1312, 0x85);

        cpu.run();

        assert_eq!(cpu.get_registers().get_a(), 0xC2);
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
    }

    #[test]
    fn sty_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0x84);
        cpu.write_byte(0x1, 0x12);
        *cpu.get_registers().get_mut_y() = 0x14;

        cpu.run();

        assert_eq!(cpu.read_byte(0x12), 0x14);
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
    }

    #[test]
    fn inc_zp() {
        let mut cpu = CPU::new();
        cpu.write_byte(0x0, 0xE6);
        cpu.write_byte(0x1, 0x32);
        cpu.write_byte(0x32, 0x44);

        cpu.run();

        assert_eq!(cpu.read_byte(0x32), 0x45);
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
    }
}
