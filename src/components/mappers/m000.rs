use super::mapper::{self, MapperData};

pub struct M000(MapperData);
impl mapper::MapperFn for M000 {
    fn new(prg_bank: u8, chr_bank: u8) -> Self {
        Self(MapperData { prg_bank, chr_bank })
    }

    fn allow_cpu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        return match addr {
            0x8000..=0xFFFF => {
                let mapping: u32 = if self.0.prg_bank > 1 {
                    0x7FFFu32
                } else {
                    0x3FFFu32
                };
                *mapped_addr = addr as u32 & mapping;
                true
            }
            _ => false,
        };
    }

    fn allow_cpu_write(&mut self, addr: u16, mapped_addr: &mut u32) -> bool {
        return match addr {
            0x8000..=0xFFFF => {
                let mapping: u32 = if self.0.prg_bank > 1 {
                    0x7FFFu32
                } else {
                    0x3FFFu32
                };
                *mapped_addr = addr as u32 & mapping;
                true
            }
            _ => false,
        };
    }

    //no mapping for ppu treat as RAM
    fn allow_ppu_read(&mut self, addr: u16, mapped_addr: &mut u32) -> bool {
        return match addr {
            0x0000..=0x1FFF => {
                *mapped_addr = addr as u32;
                true
            }
            _ => false,
        };
    }

    fn allow_ppu_write(&mut self, addr: u16, mapped_addr: &mut u32) -> bool {
        return match addr {
            0x0000..=0x1FFF if self.0.chr_bank == 0 => {
                *mapped_addr = addr as u32;
                return true;
            }
            _ => false,
        };
    }
}
