use super::{dh_cpu::CPU, END_OF_RAM, KB, START_OF_RAM};

#[derive(Debug, Clone)]
pub struct Bus {
    pub cpu_ram: [u8; KB(64)],  // 2Kb of ram
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
    pub fn clock(&mut self, cpu: &mut CPU) {
        if self.sys_clock_counter % 3 == 0 {
            CPU::reset(cpu, self);
        }
        self.sys_clock_counter += 1;
    }

    pub fn reset(&mut self, cpu: &mut CPU) {
        CPU::reset(cpu, self);
        self.sys_clock_counter = 0;
    }

    #[cfg(feature = "debug")]
    pub fn load_instruction_mem(&mut self, data: Box<[Box<[u16]>]>) {
        const PHYSICAL_ADDR: usize = 0;
        const OPCODE_ADDR: usize = 1;
        for cin in data.iter() {
            self.write(cin[PHYSICAL_ADDR], cin[OPCODE_ADDR] as u8);
        }
    }
}
