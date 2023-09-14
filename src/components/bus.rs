use super::{dh6502_cpu::M6502, START_OF_RAM, KB, END_OF_RAM};

#[derive(Debug, Clone)]
pub struct Bus {
    pub cpu_ram: [u8; KB(64)],   // 2Kb of ram
    pub sys_clock_counter: u32, // motherboards clock for busses
}
impl Bus {
    /// Creates a new [`Bus`]. With 2Kb of MOS 6502 memory
    pub fn new() -> Self {
        Self {
            cpu_ram: [0u8; KB(64)],
            sys_clock_counter: 0,
        }
    }

    #[inline]
    pub fn read(&self, addr: u16, _b_read_only: bool) -> u8 {
        if addr >= START_OF_RAM && addr <= END_OF_RAM {
            return self.cpu_ram[addr as usize];
        }
        0x00
    }

    #[inline]
    pub fn write(&mut self, addr: u16, data: u8) {
        if addr >= START_OF_RAM && addr <= END_OF_RAM {
            self.cpu_ram[addr as usize] = data;
        }
    }

    #[inline]
    pub fn clock(&mut self, cpu: &mut M6502) {
        if self.sys_clock_counter % 3 == 0 {
            M6502::reset(cpu, self);
        }
        self.sys_clock_counter += 1;
    }

    pub fn reset(&mut self, cpu: &mut M6502) {
        M6502::reset(cpu, self);
        self.sys_clock_counter = 0;
    }
}
