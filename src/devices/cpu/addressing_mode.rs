use super::processor::Cpu;
use crate::devices::bus::Bus;

pub trait AddressingMode {
    fn IMP(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetched = cpu.acc;
        return 0;
    }

    fn IMM(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.addr_abs = cpu.pc;
        return 0;
    }

    fn ZP0(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.addr_abs = bus.read(cpu.pc, false) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= 0x00FF;
        return 0;
    }

    fn ZPX(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.addr_abs = bus.read(cpu.pc + cpu.x as u16, false) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= 0x00FF;
        return 0;
    }

    fn ZPY(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.addr_abs = bus.read(cpu.pc + cpu.y as u16, false) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= 0x00FF;
        return 0;
    }

    fn ABS(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let lo = bus.read(cpu.pc as u16, false);
        cpu.pc += 1;
        let hi = bus.read(cpu.pc as u16, false);
        cpu.pc += 1;
        cpu.addr_abs = ((hi << 8) | lo) as u16;
        return 0;
    }

    fn ABX(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let lo = bus.read(cpu.pc as u16, false);
        cpu.pc += 1;
        let hi = bus.read(cpu.pc as u16, false);
        cpu.pc += 1;
        cpu.addr_abs = ((hi << 8) | lo) as u16;
        cpu.addr_abs += cpu.x as u16;

        return if (cpu.addr_abs & 0x00FF) != (hi << 8) as u16 {
            1
        } else {
            0
        };
    }

    fn ABY(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let lo = bus.read(cpu.pc as u16, false);
        cpu.pc += 1;
        let hi = bus.read(cpu.pc as u16, false);
        cpu.pc += 1;
        cpu.addr_abs = ((hi << 8) | lo) as u16;
        cpu.addr_abs += cpu.y as u16;

        return if (cpu.addr_abs & 0x00FF) != (hi << 8) as u16 {
            1
        } else {
            0
        };
    }

    fn REL(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.addr_rel = bus.read(cpu.pc, false) as u16;
        cpu.pc += 1;
        if (cpu.addr_rel & 0x08) != 0 {
            cpu.addr_abs |= 0x00FF;
        }
        return 0;
    }

    fn IND(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let pointer_lo = bus.read(cpu.pc, false) as u16;
        cpu.pc += 1;
        let pointer_hi = bus.read(cpu.pc as u16, false) as u16;
        cpu.pc += 1;

        let ptr = (pointer_hi << 8u16) | pointer_lo;

        if pointer_lo == 0x00FF {
            cpu.addr_abs = (bus.read(ptr & 0x00FF, false) << 8 | bus.read(ptr + 0, false)) as u16;
        } else {
            cpu.addr_abs = (bus.read(ptr + 1, false) << 8 | bus.read(ptr + 0, false)) as u16;
        }
        return 0;
    }

    fn IZX(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let t = bus.read(cpu.pc, false);
        cpu.pc += 1;

        let lo = bus.read((t + cpu.x) as u16 & 0x00FF, false);
        let hi = bus.read((t + cpu.x + 1) as u16 & 0x00FF, false);

        cpu.addr_abs = ((hi << 8u8) | lo << 8u8) as u16;
        return 0;
    }

    fn IZY(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let t = bus.read(cpu.pc, false);
        cpu.pc += 1;

        let lo = bus.read((t + cpu.y) as u16 & 0x00FF, false);
        let hi = bus.read((t + cpu.y + 1) as u16 & 0x00FF, false);

        cpu.addr_abs = ((hi << 8u8) | lo) as u16;
        cpu.addr_abs += cpu.y as u16;

        return if (cpu.addr_abs & 0xFF00) != (hi << 8u8) as u16 {
            1
        } else {
            0
        };
    }
}

#[test]
fn overflow_test() {
    println!("0xff0000 << 8 = {}", 0xff00 << 8u8);
    println!("0x00ff00 << 8 = {}", 0x00ff << 8u8);
    println!(
        "0xffff00 >> 8 = {}",
        ((0xff00 << 8u8) | (0x00ff << 8u8)) >> 8u8
    );
}
