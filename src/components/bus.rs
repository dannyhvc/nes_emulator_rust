use super::{BOTTOM_OF_RAM, TOP_OF_RAM};

#[derive(Debug, Clone)]
pub struct Bus {
    pub ram: [u8; 64 * 1024], // for now
}
impl Bus {
    /// Creates a new [`Bus`]. With 2Kb of MOS 6502 memory
    pub fn new() -> Self {
        Self {
            ram: [0u8; 64 * 1024],
        }
    }

    #[inline]
    pub fn read(&self, addr: u16, _b_read_only: bool) -> u8 {
        if addr >= BOTTOM_OF_RAM && addr <= TOP_OF_RAM {
            return self.ram[addr as usize];
        }
        0x00
    }

    #[inline]
    pub fn write(&mut self, addr: u16, data: u8) {
        if addr >= BOTTOM_OF_RAM && addr <= TOP_OF_RAM {
            self.ram[addr as usize] = data;
        }
    }
}
