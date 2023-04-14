use super::bus::Bus;
use super::types::{M6502AddrModes, M6502Flags, M6502Opcodes};
use super::{HIGH_BYTE, LOOKUP_TABLE, LOW_BYTE, TOP_BIT_THRESH};

#[derive(Debug, Clone)]
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
    #[inline]
    pub fn new() -> Self {
        Self {
            acc: 0x00,
            x: 0x00,
            y: 0x00,
            stkp: 0x00,
            pc: 0x0000,
            status: M6502Flags::E as u8,
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

    /**
    ### Sets or clears a specific bit of the status register
    */
    #[inline]
    pub fn set_flag(&mut self, f: M6502Flags, v: bool) {
        if v {
            self.status |= f as u8;
        } else {
            self.status |= !(f as u8) // flip da bits
        }
    }

    /**
    ### Returns the value of a specific bit of the status register
    */
    pub fn get_flag(&mut self, f: M6502Flags) -> u8 {
        return if (self.status & f as u8) > 0 {
            1u8
        } else {
            0u8
        };
    }
}

impl M6502Opcodes for M6502 {
    fn adc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        // Grab the data that we are adding to the accumulator
        cpu.fetch(bus);

        // Add is performed in 16-bit domain for emulation to capture any
        // carry bit, which will exist in bit 8 of the 16-bit word
        cpu.temp = cpu.acc as u16 + cpu.fetched as u16 + cpu.get_flag(M6502Flags::C) as u16;

        // The carry flag out exists in the high byte bit 0
        cpu.set_flag(M6502Flags::C, cpu.temp > 255);

        // The Zero flag is set if the result is 0
        cpu.set_flag(M6502Flags::Z, (cpu.temp & LOW_BYTE) == 0);

        // The signed Overflow flag is set based on all that up there! :D
        cpu.set_flag(
            M6502Flags::V,
            !(cpu.acc as u16 ^ cpu.fetched as u16) & (cpu.acc as u16 ^ cpu.temp) & 0x0080 != 0,
        );

        // The negative flag is set to the most significant bit of the result
        cpu.set_flag(M6502Flags::N, (cpu.temp & TOP_BIT_THRESH) != 0);

        // Load the result into the accumulator (it's 8-bit dont forget!)
        cpu.acc = ((cpu.temp as u16) & LOW_BYTE) as u8;

        // This instruction has the potential to require an additional clock cycle
        1u8
    }

    /**
    1) Fetch the data you are working with
    2) Perform calculation
    3) Store the result in desired place
    4) Set Flags of the status register
    5) Return if instruction has potential to require additional
        clock cycle
    ### Instruction: Bitwise Logic AND
    - Function:    A = A & M
    - Flags Out:   N, Z
    */
    fn and(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.acc &= cpu.fetched;
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, (cpu.acc & TOP_BIT_THRESH as u8) != 0);
        1u8
    }

    /**
    ### Instruction: Arithmetic Shift Left
    - Function:    A = C <- (A << 1) <- 0
    -Flags Out:   N, Z, C
    */
    fn asl(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.temp = (cpu.fetched << 1) as u16;
        cpu.set_flag(M6502Flags::C, (cpu.temp & HIGH_BYTE) > 0);
        cpu.set_flag(M6502Flags::Z, (cpu.temp & LOW_BYTE) == 0);
        cpu.set_flag(M6502Flags::N, (cpu.temp & TOP_BIT_THRESH) != 0);
        if LOOKUP_TABLE[cpu.opcode as usize].2 as usize == M6502::imp as usize {
            cpu.acc = (cpu.temp & LOW_BYTE) as u8;
        } else {
            bus.write(cpu.addr_abs, (cpu.temp & LOW_BYTE) as u8);
        }
        0u8
    }

    /**
    ### Instruction: Branch if Carry Clear
    - Function:    if(C == 0) pc = address
    */
    fn bcc(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::C) == 0_u8 {
            cpu.cycles += 1_u8;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1_u8;
            }
            cpu.pc = cpu.addr_abs;
        }
        0_u8
    }

    /**
    ### Instruction: Branch if Carry Set
    - Function:    if(C == 1) pc = address
    */
    fn bcs(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::C) == 1_u8 {
            cpu.cycles += 1_u8;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1_u8;
            }
            cpu.pc = cpu.addr_abs;
        }
        1u8
    }

    /**
    ### Instruction: Branch if Equal
    - Function:    if(Z == 1) pc = address
    */
    fn beq(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::Z) == 1_u8 {
            cpu.cycles += 1_u8;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1_u8;
            }
            cpu.pc = cpu.addr_abs;
        }
        0_u8
    }

    fn bit(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.temp = (cpu.acc & cpu.fetched) as u16;
        cpu.set_flag(M6502Flags::Z, (cpu.temp & LOW_BYTE) == 0x00);
        cpu.set_flag(M6502Flags::N, (cpu.fetched & (1 << 7)) != 0);
        cpu.set_flag(M6502Flags::V, (cpu.fetched & (1 << 6)) != 0);
        0_u8
    }

    /**
    ### Instruction: Branch if Negative
    Function:    if(N == 1) pc = address
    */
    fn bmi(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::N) == 1_u8 {
            cpu.cycles += 1_u8;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1_u8;
            }
            cpu.pc = cpu.addr_abs;
        }
        0_u8
    }

    /**
    ### Instruction: Branch if Not Equal
    - Function:    if(Z == 0) pc = address
    */
    fn bne(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::Z) == 0_u8 {
            cpu.cycles += 1_u8;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1_u8;
            }
            cpu.pc = cpu.addr_abs;
        }
        0_u8
    }

    fn bpl(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::N) == 0 {
            cpu.cycles += 1;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs;
        }
        0x0u8
    }

    fn brk(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.pc += 1;

        cpu.set_flag(M6502Flags::I, true);
        bus.write(
            (0x0100_u16 + cpu.stkp as u16).into(),
            (cpu.pc >> 8 & LOW_BYTE) as u8,
        );
        cpu.stkp -= 1;
        bus.write(
            (0x0100_u16 + cpu.stkp as u16).into(),
            (cpu.pc & LOW_BYTE) as u8,
        );
        cpu.stkp -= 1;

        cpu.set_flag(M6502Flags::B, true);
        bus.write((0x0100_u16 + cpu.stkp as u16).into(), cpu.status);
        cpu.stkp -= 1;
        cpu.set_flag(M6502Flags::B, true);

        cpu.pc = ((bus.read(0xFFFE, false) != 0x0u8) | (bus.read(0xFFFF, false) != 0x0u8)).into();
        0x0u8
    }

    fn bvc(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::V) == 0u8 {
            cpu.cycles += 1;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs;
        }
        0x0u8
    }

    fn bvs(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::V) == 1u8 {
            cpu.cycles += 1;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs;
        }
        0x0u8
    }

    fn clc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::C, false);
        0x0u8
    }

    fn cld(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::D, false);
        0u8
    }

    fn cli(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::I, false);
        0u8
    }

    fn clv(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::V, false);
        0u8
    }

    fn cmp(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.temp = (cpu.acc - cpu.fetched) as u16;
        cpu.set_flag(M6502Flags::C, cpu.acc >= cpu.fetched);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        1u8
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
    fn imp(cpu: &mut M6502, _: &mut Bus) -> u8 {
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
        cpu.addr_abs &= LOW_BYTE; // checking if high bit is on a new page
        0x00
    }

    fn zpx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.addr_abs = bus.read(cpu.pc + cpu.x as u16, false) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= LOW_BYTE;
        0x00
    }

    fn zpy(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.addr_abs = bus.read(cpu.pc + cpu.y as u16, false) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= LOW_BYTE;
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

        return if (cpu.addr_abs & LOW_BYTE) != (hi << 8) as u16 {
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

        return if (cpu.addr_abs & LOW_BYTE) != (hi << 8) as u16 {
            0x01
        } else {
            0x00
        };
    }

    fn rel(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.addr_rel = bus.read(cpu.pc, false) as u16;
        cpu.pc += 1;
        if (cpu.addr_rel & 0x08) != 0 {
            cpu.addr_abs |= LOW_BYTE;
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
        if pointer_lo == LOW_BYTE {
            lo = (bus.read(ptr & LOW_BYTE, false) as u32) << 8;
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

        let lo: u32 = bus.read((t + cpu.x) as u16 & LOW_BYTE, false).into();
        let hi: u32 = bus.read((t + cpu.x + 1) as u16 & LOW_BYTE, false).into();

        cpu.addr_abs = ((hi << 8u8) | lo << 8u8) as u16 >> 8u16;
        0x00
    }

    fn izy(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let t = bus.read(cpu.pc, false);
        cpu.pc += 1;

        let lo = bus.read((t + cpu.y) as u16 & LOW_BYTE, false);
        let hi = bus.read((t + cpu.y + 1) as u16 & LOW_BYTE, false);

        cpu.addr_abs = (((hi as u16) << 8u16) | (lo as u16) << 8u16) as u16;
        cpu.addr_abs += cpu.y as u16;

        return if (cpu.addr_abs & HIGH_BYTE) != ((hi as u16) << 8u8) as u16 {
            0x01
        } else {
            0x00
        };
    }
}
