use super::{BOTTOM_OF_RAM, TOP_OF_RAM};

#[derive(Debug, Clone)]
pub struct Bus {
    pub ram: [u8; 64 * 1024], // for now
}
impl Bus {
    pub fn new() -> Self {
        todo!()
    }

    pub fn read(&mut self, addr: u16, _b_read_only: bool) -> u8 {
        if addr >= BOTTOM_OF_RAM && addr <= TOP_OF_RAM {
            return self.ram[addr as usize];
        }
        0x00
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if addr >= BOTTOM_OF_RAM && addr <= TOP_OF_RAM {
            self.ram[addr as usize] = data;
        }
    }
}
