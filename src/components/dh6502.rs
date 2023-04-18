use std::collections::HashMap;
use std::iter;

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

    #[inline]
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
    pub fn set_flag(&mut self, f: M6502Flags, conditional_set: bool) {
        if conditional_set {
            self.status |= f as u8;
        } else {
            self.status |= !(f as u8) // flip da bits
        }
    }

    /**
    ### Returns the value of a specific bit of the status register
    */
    #[inline(always)]
    pub const fn get_flag(&self, f: M6502Flags) -> u8 {
        return if self.status & f as u8 > 0 { 1u8 } else { 0u8 };
    }

    #[inline(always)]
    pub const fn complete(&self) -> bool {
        self.cycles == 0
    }

    #[allow(dead_code)]
    pub fn disassemble(bus: &mut Bus, start: u16, stop: u16) -> HashMap<u16, String> {
        let mut address: u32 = start.into();
        let mut value: u8 = 0x00;
        let mut low: u8 = 0x00;
        let mut high: u8 = 0x00;
        let mut line_address: u16 = 0;

        let mut lined_maps: HashMap<u16, String> = HashMap::<u16, String>::new();

        let to_hex = |n: u32, d: u8| -> String {
            let mut s: Vec<char> = iter::repeat('0').take(d.into()).collect::<Vec<char>>();
            let mut num: u32 = n;
            let hex_alpha: Vec<char> = "0123456789ABCDEF".chars().collect::<Vec<char>>();
            for i in (0..=d - 1).rev() {
                num >>= 4;
                s[i as usize] = hex_alpha[(num & 0xF) as usize];
            }
            s.into_iter().collect()
        };

        while address <= stop as u32 {
            line_address = address as u16;

            let mut instruction_address: String = format!("${}{}", to_hex(address, 4), ": ");
            let opcode: u8 = bus.read(address as u16, true);

            address += 1;
            instruction_address.push_str(format!("{} ", LOOKUP_TABLE[opcode as usize].0).as_str());

            if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::imp as usize {
                instruction_address.push_str(" {IMP}");
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::imm as usize {
                value = bus.read(address as u16, true);
                address += 1;
                high = 0x00;
                let string_rep = format!("#${} {{imm}}", to_hex(low as u32, 2));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::zp0 as usize {
                low = bus.read(address as u16, true);
                address += 1;
                high = 0x00;
                let string_rep = format!("${} {{zp0}}", to_hex(low as u32, 2));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::zpx as usize {
                low = bus.read(address as u16, true);
                address += 1;
                high = 0x00;
                let string_rep = format!("${}, X {{zpx}}", to_hex(low as u32, 2));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::zpy as usize {
                low = bus.read(address as u16, true);
                address += 1;
                high = 0x00;
                let string_rep = format!("${}, Y {{zpy}}", to_hex(low as u32, 2));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::izx as usize {
                low = bus.read(address as u16, true);
                address += 1;
                high = 0x00;
                let string_rep = format!("(${}, X) {{izx}}", to_hex(low as u32, 2));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::izy as usize {
                low = bus.read(address as u16, true);
                address += 1;
                high = 0x00;
                let string_rep = format!("(${}), Y {{izy}}", to_hex(low as u32, 2));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::abs as usize {
                low = bus.read(address as u16, false);
                address += 1;
                high = bus.read(address as u16, false);
                address += 1;
                let string_rep = format!("${} {{abs}}", to_hex(((high << 8) | low) as u32, 4));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::abx as usize {
                low = bus.read(address as u16, false);
                address += 1;
                high = bus.read(address as u16, false);
                address += 1;
                let string_rep = format!("${} {{abx}}", to_hex(((high << 8) | low) as u32, 4));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::aby as usize {
                low = bus.read(address as u16, false);
                address += 1;
                high = bus.read(address as u16, false);
                address += 1;
                let string_rep = format!("${} {{aby}}", to_hex(((high << 8) | low) as u32, 4));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::ind as usize {
                low = bus.read(address as u16, false);
                address += 1;
                high = bus.read(address as u16, false);
                address += 1;
                let string_rep = format!("(${}) {{ind}}", to_hex(((high << 8) | low) as u32, 4));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::rel as usize {
                value = bus.read(address as u16, false);
                address += 1;
                let string_rep = format!(
                    "${} [${}] {{rel}}",
                    to_hex(value as u32, 2),
                    to_hex(address + value as u32, 4)
                );
                instruction_address.push_str(&string_rep);
            }
            lined_maps.insert(line_address, instruction_address.clone());
        }

        lined_maps
    }
}

impl M6502Opcodes for M6502 {
    fn adc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        // Grab the data that we are adding to the accumulator
        // Add is performed in 16-bit domain for emulation to capture any
        // carry bit, which will exist in bit 8 of the 16-bit word
        cpu.temp = (cpu.acc + cpu.fetch(bus) + cpu.get_flag(M6502Flags::C)).into();

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
        cpu.acc &= cpu.fetch(bus);
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, (cpu.acc & TOP_BIT_THRESH as u8) != 0);
        1u8
    }

    /**
    ### Instruction: Arithmetic Shift Left
    - Function:    A = C <- (A << 1) <- 0
    -Flags Out:   N, Z, C
    */
    #[inline]
    fn asl(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.fetch(bus) << 1).into();
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
    #[inline]
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
    #[inline]
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
    #[inline]
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

    #[inline]
    fn bit(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.acc & cpu.fetch(bus)) as u16;
        cpu.set_flag(M6502Flags::Z, (cpu.temp & LOW_BYTE) == 0x00);
        cpu.set_flag(M6502Flags::N, (cpu.fetched & (1 << 7)) != 0);
        cpu.set_flag(M6502Flags::V, (cpu.fetched & (1 << 6)) != 0);
        0_u8
    }

    /**
    ### Instruction: Branch if Negative
    Function:    if(N == 1) pc = address
    */
    #[inline]
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
    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn bvs(cpu: &mut M6502, _: &mut Bus) -> u8 {
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

    #[inline]
    fn clc(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::C, false);
        0x0u8
    }

    #[inline]
    fn cld(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::D, false);
        0u8
    }

    #[inline]
    fn cli(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::I, false);
        0u8
    }

    #[inline]
    fn clv(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::V, false);
        0u8
    }

    #[inline]
    fn cmp(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.acc - cpu.fetch(bus)).into();
        cpu.set_flag(M6502Flags::C, cpu.acc >= cpu.fetched);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        1u8
    }

    /// Compare X Register with Memory
    ///
    /// This instruction compares the contents of the X register with another
    /// memory held value and sets the zero and carry flags as appropriate.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the `M6502` struct representing the CPU
    /// * `bus` - A mutable reference to the `Bus` struct representing the system bus
    ///
    /// # Returns
    ///
    /// The result of the operation, which is always 0.
    #[inline]
    fn cpx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.x - cpu.fetch(bus)).into();
        cpu.set_flag(M6502Flags::C, cpu.x >= cpu.fetched);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        0u8
    }

    #[inline]
    fn cpy(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.y - cpu.fetch(bus)).into();
        cpu.set_flag(M6502Flags::C, cpu.y >= cpu.fetched);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        0u8
    }

    #[inline]
    fn dec(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = cpu.fetch(bus) as u16 - 1;
        bus.write(cpu.addr_abs, (cpu.temp & LOW_BYTE) as u8);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        0u8
    }

    #[inline]
    fn dex(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.x -= 1;
        cpu.set_flag(M6502Flags::Z, cpu.x == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.x & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn dey(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.y -= 1;
        cpu.set_flag(M6502Flags::Z, cpu.y == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.y & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn eor(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.acc ^= cpu.fetch(bus);
        cpu.set_flag(M6502Flags::Z, cpu.y == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.y & TOP_BIT_THRESH as u8 != 0x0000);
        1u8
    }

    #[inline]
    fn inc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = cpu.fetch(bus) as u16 + 1;
        bus.write(cpu.addr_abs, (cpu.temp & LOW_BYTE) as u8);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        0u8
    }

    #[inline]
    fn inx(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.x += 1;
        cpu.set_flag(M6502Flags::Z, cpu.x == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.x & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn iny(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.y += 1;
        cpu.set_flag(M6502Flags::Z, cpu.y == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.y & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn jmp(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.pc = cpu.addr_abs;
        0u8
    }

    #[inline]
    fn jsr(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.pc -= 1;

        bus.write(0x0100 + cpu.stkp as u16, (cpu.pc << 8 & LOW_BYTE) as u8);
        cpu.stkp -= 1;
        bus.write(0x0100 + cpu.stkp as u16, (cpu.pc & LOW_BYTE) as u8);
        cpu.stkp -= 1;

        cpu.pc = cpu.addr_abs;
        0u8
    }

    #[inline]
    fn lda(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.acc = cpu.fetch(bus);
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.acc & TOP_BIT_THRESH as u8 != 0x00);
        1u8
    }

    #[inline]
    fn ldx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.x = cpu.fetch(bus);
        cpu.set_flag(M6502Flags::Z, cpu.x == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.x & TOP_BIT_THRESH as u8 != 0x00);
        1u8
    }

    #[inline]
    fn ldy(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.y = cpu.fetch(bus);
        cpu.set_flag(M6502Flags::Z, cpu.y == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.y & TOP_BIT_THRESH as u8 != 0x00);
        1u8
    }

    #[inline]
    fn lsr(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.fetch(bus) >> 1) as u16;
        cpu.set_flag(M6502Flags::C, cpu.fetched & 0x0001 != 0x0000);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        if LOOKUP_TABLE[cpu.opcode as usize].2 as usize == M6502::imp as usize {
            cpu.acc = cpu.temp as u8 & LOW_BYTE as u8;
        } else {
            bus.write(cpu.addr_abs, (cpu.temp & LOW_BYTE) as u8);
        }
        0u8
    }

    #[inline]
    fn nop(cpu: &mut M6502, _: &mut Bus) -> u8 {
        return match cpu.opcode {
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => 1u8,
            _ => 0u8,
        };
    }

    #[inline]
    fn ora(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.acc |= cpu.fetch(bus);
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.acc & TOP_BIT_THRESH as u8 != 0x00);
        1u8
    }

    #[inline]
    fn pha(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        bus.write(0x0100 + cpu.stkp as u16, cpu.acc);
        cpu.stkp -= 1;
        0u8
    }

    #[inline]
    fn php(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        bus.write(
            0x0100 + cpu.stkp as u16,
            cpu.status | M6502Flags::B as u8 | M6502Flags::U as u8,
        );
        cpu.set_flag(M6502Flags::B, false);
        cpu.set_flag(M6502Flags::U, false);
        cpu.stkp -= 1;
        0u8
    }

    #[inline]
    fn pla(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.stkp += 1;
        cpu.status = bus.read(0x0100 + cpu.stkp as u16, false);
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.acc & TOP_BIT_THRESH as u8 == 0x00);
        0u8
    }

    #[inline]
    fn plp(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.stkp += 1;
        cpu.status = bus.read(0x0100 + cpu.stkp as u16, false);
        cpu.set_flag(M6502Flags::U, true);
        0u8
    }

    #[inline]
    fn rol(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.fetch(bus) << 1 | cpu.get_flag(M6502Flags::C)).into();
        cpu.set_flag(M6502Flags::C, cpu.temp & HIGH_BYTE != 0x0000);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        if LOOKUP_TABLE[cpu.opcode as usize].2 as usize == M6502::imp as usize {
            cpu.acc = (cpu.temp & LOW_BYTE) as u8;
        } else {
            bus.write(cpu.addr_abs, (cpu.temp & LOW_BYTE) as u8);
        }
        0u8
    }

    #[inline]
    fn ror(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.get_flag(M6502Flags::C) << 7 | cpu.fetch(bus) >> 1).into();
        cpu.set_flag(M6502Flags::C, cpu.fetched & 0x01 == 0x00);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x00);
        if LOOKUP_TABLE[cpu.opcode as usize].2 as usize == M6502::imp as usize {
            cpu.acc = (cpu.temp & LOW_BYTE) as u8;
        } else {
            bus.write(cpu.addr_abs, (cpu.temp & LOW_BYTE) as u8);
        }
        0u8
    }

    #[inline]
    fn rti(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.stkp += 1;
        cpu.status = bus.read(0x0100 + cpu.stkp as u16, false);
        cpu.status &= !(M6502Flags::B as u8);
        cpu.status &= !(M6502Flags::U as u8);

        cpu.stkp += 1;
        cpu.pc = bus.read(0x0100 + cpu.stkp as u16, false).into();
        cpu.stkp += 1;
        cpu.pc |= (bus.read(0x0100 + cpu.stkp as u16, false) as u16) << 8;
        0u8
    }

    #[inline]
    fn rts(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.stkp += 1;
        cpu.pc = bus.read(0x0100 + cpu.stkp as u16, false).into();
        cpu.stkp += 1;
        cpu.pc |= (bus.read(0x0100 + cpu.stkp as u16, false) as u16) << 8;

        cpu.pc += 1;
        0u8
    }

    #[inline]
    fn sbc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let value: u16 = cpu.fetch(bus) as u16 ^ LOW_BYTE;
        cpu.temp = cpu.acc as u16 + value + cpu.get_flag(M6502Flags::C) as u16;
        cpu.set_flag(M6502Flags::C, cpu.temp & HIGH_BYTE != 0x0000);
        cpu.set_flag(M6502Flags::Z, cpu.temp & HIGH_BYTE == 0x0000);
        cpu.set_flag(
            M6502Flags::V,
            (cpu.temp ^ cpu.acc as u16) & (cpu.temp ^ value) & TOP_BIT_THRESH != 0x0000,
        );
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH == 0x0000);
        cpu.acc = cpu.temp as u8 & LOW_BYTE as u8;
        1u8
    }

    #[inline]
    fn sec(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::C, true);
        0u8
    }

    #[inline]
    fn sed(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::D, true);
        0u8
    }

    #[inline]
    fn sei(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::I, true);
        0u8
    }

    #[inline]
    fn sta(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        bus.write(cpu.addr_abs, cpu.acc);
        0u8
    }

    #[inline]
    fn stx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        bus.write(cpu.addr_abs, cpu.x);
        0u8
    }

    #[inline]
    fn sty(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        bus.write(cpu.addr_abs, cpu.y);
        0u8
    }

    #[inline]
    fn tax(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.x = cpu.acc;
        cpu.set_flag(M6502Flags::Z, cpu.x == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.x & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn tay(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.y = cpu.acc;
        cpu.set_flag(M6502Flags::Z, cpu.y == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.y & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn tsx(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.x = cpu.stkp;
        cpu.set_flag(M6502Flags::Z, cpu.x == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.x & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn txa(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.acc = cpu.x;
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.acc & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline(always)]
    fn txs(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.stkp = cpu.x;
        0u8
    }

    #[inline]
    fn tya(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.acc = cpu.y;
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.acc & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline(always)]
    fn xxx(_: &mut M6502, _: &mut Bus) -> u8 {
        0u8
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

        let ptr: u16 = (pointer_hi << 8u16) | pointer_lo;

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

    /// Indirect Zero-Page Indexed with X Addressing Mode
    ///
    /// This addressing mode is used by certain instructions to access memory indirectly,
    /// using a zero page address that is added to the X register. The address is read
    /// from the zero page address (t + X) and the low byte is used as the lower 8 bits
    /// of the effective address, while the high byte is fetched from the next location
    /// in memory (t + X + 1), wrapping around if necessary. This results in an effective
    /// address that can range from 0x0000 to 0xFFFF, with the X register being added to
    /// the zero page address t, and the page boundary crossing detection being performed
    /// on the resulting effective address.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the [`M6502`] struct representing the CPU
    /// * `bus` - A mutable reference to the [`Bus`] struct representing the system bus
    ///
    /// # Returns
    ///
    /// The result of the operation, which is always 0x00.
    ///
    /// # Examples
    ///```rust
    /// use rust6502::M6502;
    /// use rust6502::Bus;
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    ///
    /// cpu.x = 0x04;
    /// bus.write(0x10, 0x05);
    /// bus.write(0x11, 0x06);
    /// bus.write(0x0605, 0x42);
    ///
    /// assert_eq!(cpu.pc, 0x0000);
    /// assert_eq!(cpu.addr_abs, 0x0000);
    ///
    /// M6502::izx(&mut cpu, &mut bus);
    /// assert_eq!(cpu.pc, 0x0001);
    /// assert_eq!(cpu.addr_abs, 0x4205);
    ///
    /// let result = bus.read(cpu.addr_abs, true);
    /// assert_eq!(result, 0x42);
    /// ```
    fn izx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let t: u8 = bus.read(cpu.pc, false);
        cpu.pc += 1;

        let lo: u32 = bus.read((t + cpu.x) as u16 & LOW_BYTE, false).into();
        let hi: u32 = bus.read((t + cpu.x + 1) as u16 & LOW_BYTE, false).into();

        cpu.addr_abs = ((hi << 8u8) | lo << 8u8) as u16 >> 8u16;
        0x00
    }

    /// Indirect Indexed with Y Addressing Mode
    ///
    /// This addressing mode is used by certain instructions to access memory
    /// indirectly, using a zero page address that is added to the Y register.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the [`M6502`] struct representing the CPU
    /// * `bus` - A mutable reference to the [`Bus`] struct representing the system bus
    ///
    /// # Returns
    ///
    /// The result of the operation, which is either 0 or 1 depending on whether
    /// the operation resulted in a page boundary crossing.
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
