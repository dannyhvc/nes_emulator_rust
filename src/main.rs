mod components;
mod util;

fn main() {}

mod tests {
    #![allow(non_snake_case)]
    use crate::components::{
        bus::Bus,
        dh6502_cpu::M6502,
        types::{CpuFlags, M6502AddrModes, M6502Opcodes},
    };

    #[test]
    fn test_clock() {
        let mut cpu: M6502 = M6502::new();
        let mut bus: Bus = Bus::new();
        M6502::reset(&mut cpu, &bus);
        for _ in 0..8 {
            M6502::clock(&mut cpu, &mut bus);
        }
        assert!(cpu.cycles == 0);
    }

    #[test]
    fn test_LDA() {
        let mut cpu = M6502::new();
        let mut bus = Bus::new();

        M6502::reset(&mut cpu, &bus);
        cpu.cycles = 0;

        cpu.pc = 0xFFFC;
        bus.write(cpu.pc, 0xA9); // index 169 of lookup table

        M6502::clock(&mut cpu, &mut bus);
        cpu.pc = 10;
        M6502::clock(&mut cpu, &mut bus);
        dbg!(cpu);
    }

    #[test]
    fn test_reset() {}

    #[test]
    fn test_new() {
        let _cpu: M6502 = M6502::new();
    }

    #[test]
    fn test_disassemble() {
        let mut cpu = M6502::new();
        let mut bus = Bus::new();

        M6502::reset(&mut cpu, &bus);
        cpu.cycles = 0;

        for i in 0..0x00ff {
            bus.write(i, 0xa9); // 169 LDA
        }

        let dis = M6502::disassemble(&mut bus, 0x0000, 0x00ff);
        dbg!(dis);
    }

    #[test]
    fn test_hex() {
        let string_rep: String = format!("#${:x} {{imm}}", 100u8 as u32);
        dbg!(string_rep);
    }
}
