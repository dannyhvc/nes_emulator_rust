// use super::cpu::CPU;


pub struct Bus {
    // devices
    pub ram: [u8; 64 * 1024], // 64Kb of fake memory
}

impl Bus {
    pub fn read(&mut self, addr: u16, b_read_only: bool) -> u8 {
        // start and end addresses for 64Kb of memory
        const START_ADDR: u16 = 0x0000u16;
        const END_ADDR: u16 = 0xFFFFu16;
        if addr >= START_ADDR && addr <= END_ADDR {
            return self.ram[addr as usize];
        }
        0x00u8
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        // start and end addresses for 64Kb of memory
        const START_ADDR: u16 = 0x0000u16;
        const END_ADDR: u16 = 0xFFFFu16;
        if addr >= START_ADDR && addr <= END_ADDR {
            self.ram[addr as usize] = data;
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            ram: [0x00_u8; 64 * 1024],
        }
    }
}
