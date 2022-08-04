use super::addressing_mode::AddressingMode;
use super::flags::Flags_6502;
use super::instruction::Instruction;
use super::lookup_table::LookupTable;
use super::opcode::Opcode;
use crate::devices::bus::Bus;

pub struct Cpu {
    pub status: u8,
    pub sp: u8,
    pub pc: u16,
    pub acc: u8,
    pub x: u8,
    pub y: u8,
    pub lookup: Box<Vec<Instruction>>,
    pub fetched: u8,      // Represents the working input value to the ALU
    pub temp: u16,        // A convenience variable used everywhere
    pub addr_abs: u16,    // All used memory addresses end up in here
    pub addr_rel: u16,    // Represents absolute address following a branch
    pub opcode: u8,       // Is the instruction byte
    pub cycles: u8,       // Counts how many cycles the instruction has remaining
    pub clock_count: u32, // A global accumulation of the number of clocks
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            status: 0u8,
            sp: 0u8,
            pc: 0u16,
            acc: 0u8,
            x: 0u8,
            y: 0u8,
            lookup: LookupTable::new().0,
            fetched: 0x00,
            temp: 0x0000,
            addr_abs: 0x0000,
            addr_rel: 0x00,
            opcode: 0x00,
            cycles: 0,
            clock_count: 0,
        }
    }

    // settings status flags
    pub fn get_flag(&self, f: Flags_6502) -> u8 {
        return if (self.status & (f as u8)) > 0u8 {
            1u8
        } else {
            0u8
        };
    }

    pub fn set_flag(&mut self, f: Flags_6502, v: bool) {
        if v {
            self.status |= f as u8;
        } else {
            self.status &= !(f as u8);
        }
    }

    // External event functions. In hardware these represent pins that are asserted
    // to produce a change in state.
    // Reset Interrupt - Forces CPU into known state
    pub fn reset(&mut self, bus: &mut Bus) {
        // address to set the pc to
        self.addr_abs = 0xFFFC;
        let lo: u8 = bus.read(self.addr_abs + 0, false);
        let hi: u8 = bus.read(self.addr_abs + 1, false);

        // Set pc
        self.pc = ((hi << 8) | lo) as u16;

        // rest internal registers
        self.acc = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.status = 0x00 | Flags_6502::U as u8;

        // clear addr variables
        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched = 0x00;

        // Reset interrupt takes time
        self.cycles = 8;
    }

    // Interrupt Request - Executes an instruction at a specific location
    pub fn irq(&mut self, bus: &mut Bus) {
        if self.get_flag(Flags_6502::I) == 0 {
            // Push the program counter to the stack. Its 16bits don't forget so that takes 2 pushs
            bus.write(0x0100 + self.sp as u16, (self.pc as u8 >> 8) & 0x00FF);
            self.sp -= 1u8;
            bus.write(0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
            self.sp -= 1u8;

            // push the status register to the stack
            self.set_flag(Flags_6502::B, false);
            self.set_flag(Flags_6502::U, true);
            self.set_flag(Flags_6502::I, true);
            bus.write(0x0100 + self.sp as u16, self.status);
            self.sp -= 1u8;

            // read new program counter location from fixed address
            self.addr_abs = 0xFFFE;
            let lo = bus.read(self.addr_abs + 0, false);
            let hi = bus.read(self.addr_abs + 1, false);
            self.pc = ((hi as u16) << 8u16) | lo as u16;

            // Interrupt takes time to complete
            self.cycles = 7;
        }
    }

    // Non-Maskable Interrupt Request - As above, but cannot be disabled
    pub fn nmi(&mut self, bus: &mut Bus) {
        bus.write(0x0100 + self.sp as u16, (self.pc as u8 >> 8) & 0x00FF);
        self.sp -= 1;
        bus.write(0x0100 + self.sp as u16, self.pc as u8 & 0x00FF);
        self.sp -= 1;

        self.set_flag(Flags_6502::B, false);
        self.set_flag(Flags_6502::U, true);
        self.set_flag(Flags_6502::I, true);

        bus.write(0x0100 + self.sp as u16, self.status);
        self.pc += 1;

        self.addr_abs = 0x0FFFA;
        let lo = bus.read(self.addr_abs + 0, false);
        let hi = bus.read(self.addr_abs + 1, false);
        self.pc = ((hi << 8) | lo) as u16;

        self.cycles = 8;
    }

    // Perform one clock cycle's worth of update
    pub fn clock(&mut self, bus: &mut Bus) {
        if self.cycles == 0 {
            let opcode = bus.read(self.pc, false);

            // let log_pc = self.pc as u16;

            self.set_flag(Flags_6502::U, true);

            self.pc += 1;

            self.cycles = self.lookup[opcode as usize].cycles as u8;

            let addr_cycle_count = self.lookup[opcode as usize].addrmode.map(|x| x(self, bus));
            let operate_cycle_count = self.lookup[opcode as usize].operate.map(|x| x(self, bus));

            if let (Some(addr_cycle_count), Some(operate_cycle_count)) =
                (addr_cycle_count, operate_cycle_count)
            {
                self.cycles += addr_cycle_count & operate_cycle_count;
            }
            self.set_flag(Flags_6502::U, true);
        }

        self.clock_count += 1;
        self.cycles -= 1;
    }

    // Indicates the current instruction has completed by returning true. This is
    // a utility function to enable "step-by-step" execution, without manually
    // clocking every cycle
    pub fn complete(&self) -> bool {
        self.cycles == 0
    }

    pub fn fetch(&mut self, bus: &mut Bus) -> u8 {
        if self.lookup[self.opcode as usize]
            .addrmode
            .filter(|x| Cpu::IMP as usize == *x as usize)
            .is_some()
        {
            self.fetched = bus.read(self.addr_abs, false);
        }
        self.fetched
    }
}

impl AddressingMode for Cpu {}
impl Opcode for Cpu {}
