use super::bus::Bus;
use super::types::{M6502AddrModes, M6502Opcodes};
use super::LOOKUP_TABLE; // will be used later

pub struct M6502 {
    // cpu Core registers, exposed as public here for ease of access from external
    // examinors. This is all the 6502 has.
    pub acc: u8,    // Accumulator Register
    pub x: u8,      // X Register
    pub y: u8,      // Y Register
    pub stkp: u8,   // Stack Pointer (points to location on cpu.bus)
    pub pc: u16,    // Program Counter
    pub status: u8, // Status Register

    // Assisstive variables to facilitate emulation
    pub fetched: u8,   // Represents the working input value to the ALU
    pub temp: u16,     // A convenience variable used everywhere
    pub addr_abs: u16, // All used memory addresses end up in here
    pub addr_rel: u16, // Represents absolute address following a branch
    pub opcode: u8,    // Is the instruction byte
    pub cycles: u8,    // Counts how many cycles the instruction has remaining
    pub clock_count: u32, // A global accumulation of the number of clocks
                       // pub logs: Option<File>,
}

impl M6502 {
    pub fn new() -> Self {
        Self {
            acc: 0x00,
            x: 0x00,
            y: 0x00,
            stkp: 0x00,
            pc: 0x0000,
            status: 0x00,
            fetched: 0x00,
            temp: 0x0000,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00,
            cycles: 0,
            clock_count: 0,
        }
    }

    // TODO: just noticed a possible problem with race conditions and rust borrowing rules
    // its possible that self.fetched might own the spot in the buses ram and not give it back before
    // the next cycle. I guess we'll have to test and see
    pub fn fetch(&mut self, bus: &mut Bus) -> u8 {
        if !(LOOKUP_TABLE[self.opcode as usize].2 as usize == M6502::imp as usize) {
            self.fetched = bus.read(self.addr_abs, false);
        }
        self.fetched
    }
}

impl M6502Opcodes for M6502 {
    fn adc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn and(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn asl(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn bcc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn bcs(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn beq(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn bit(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn bmi(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn bne(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn bpl(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn brk(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn bvc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn bvs(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn clc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn cld(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn cli(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn clv(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn cmp(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn cpx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn cpy(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn dec(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn dex(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn dey(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn eor(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn inc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn inx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn iny(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn jmp(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn jsr(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn lda(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn ldx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn ldy(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn lsr(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn nop(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn ora(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn pha(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn php(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn pla(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn plp(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn rol(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn ror(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn rti(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn rts(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn sbc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn sec(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn sed(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn sei(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn sta(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn stx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn sty(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn tax(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn tay(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn tsx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn txa(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn txs(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn tya(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }

    fn xxx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        todo!()
    }
}

impl M6502AddrModes for M6502 {
    fn imp(cpu: &mut M6502, _bus: &mut Bus) -> u8 {
        cpu.fetched = cpu.acc;
        0x00
    }

    fn imm(cpu: &mut M6502, _bus: &mut Bus) -> u8 {
        cpu.addr_abs = cpu.pc;
        0x00
    }

    fn zp0(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.addr_abs = bus.read(cpu.pc, false) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= 0x00FF; // checking if high bit is on a new page
        0x00
    }

    fn zpx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.addr_abs = bus.read(cpu.pc + cpu.x as u16, false) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= 0x00FF;
        0x00
    }

    fn zpy(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.addr_abs = bus.read(cpu.pc + cpu.y as u16, false) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= 0x00FF;
        0x00
    }

    fn abs(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let lo: u32 = bus.read(cpu.pc as u16, false).into();
        cpu.pc += 1;
        let hi: u32 = bus.read(cpu.pc as u16, false).into();
        cpu.pc += 1;
        cpu.addr_abs = ((hi << 8) | lo) as u16;
        0x00
    }

    fn abx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let lo: u32 = bus.read(cpu.pc as u16, false).into();
        cpu.pc += 1;
        let hi: u32 = bus.read(cpu.pc as u16, false).into();
        cpu.pc += 1;
        cpu.addr_abs = ((hi << 8) | lo) as u16;
        cpu.addr_abs += cpu.x as u16;

        return if (cpu.addr_abs & 0x00FF) != (hi << 8) as u16 {
            0x01
        } else {
            0x00
        };
    }

    fn aby(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let lo: u16 = bus.read(cpu.pc as u16, false).into();
        cpu.pc += 1;
        let hi: u16 = bus.read(cpu.pc as u16, false).into();
        cpu.pc += 1;
        cpu.addr_abs = ((hi << 8) | lo) as u16;
        cpu.addr_abs += cpu.y as u16;

        return if (cpu.addr_abs & 0x00FF) != (hi << 8) as u16 {
            0x01
        } else {
            0x00
        };
    }

    fn rel(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.addr_rel = bus.read(cpu.pc, false) as u16;
        cpu.pc += 1;
        if (cpu.addr_rel & 0x08) != 0 {
            cpu.addr_abs |= 0x00FF;
        }
        0x00
    }

    fn ind(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let pointer_lo = bus.read(cpu.pc, false) as u16;
        cpu.pc += 1;
        let pointer_hi = bus.read(cpu.pc as u16, false) as u16;
        cpu.pc += 1;

        let ptr = (pointer_hi << 8u16) | pointer_lo;

        let lo: u32;
        let hi: u32;
        if pointer_lo == 0x00FF {
            lo = (bus.read(ptr & 0x00FF, false) as u32) << 8;
            hi = bus.read(ptr + 0, false).into();
            cpu.addr_abs = (lo | hi) as u16;
        } else {
            lo = (bus.read(ptr + 1, false) as u32) << 8;
            hi = bus.read(ptr + 0, false).into();
            cpu.addr_abs = (lo | hi) as u16;
        }
        0x00
    }

    fn izx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let t = bus.read(cpu.pc, false);
        cpu.pc += 1;

        let lo: u32 = bus.read((t + cpu.x) as u16 & 0x00FF, false).into();
        let hi: u32 = bus.read((t + cpu.x + 1) as u16 & 0x00FF, false).into();

        cpu.addr_abs = ((hi << 8u8) | lo << 8u8) as u16 >> 8u16;
        0x00
    }

    fn izy(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let t = bus.read(cpu.pc, false);
        cpu.pc += 1;

        let lo = bus.read((t + cpu.y) as u16 & 0x00FF, false);
        let hi = bus.read((t + cpu.y + 1) as u16 & 0x00FF, false);

        cpu.addr_abs = (((hi as u16) << 8u16) | (lo as u16) << 8u16) as u16;
        cpu.addr_abs += cpu.y as u16;

        return if (cpu.addr_abs & 0xFF00) != ((hi as u16) << 8u8) as u16 {
            0x01
        } else {
            0x00
        };
    }
}
