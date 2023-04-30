use super::mapper::{self, MapperData};

#[derive(Debug, Clone, Copy)]
pub struct M000(MapperData);
impl mapper::MapperFn for M000 {
    fn new(prg_bank: u8, chr_bank: u8) -> Self {
        Self(MapperData { prg_bank, chr_bank })
    }

    fn allow_cpu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        // if PRGROM is 16KB
        //     CPU Address Bus          PRG ROM
        //     0x8000 -> 0xBFFF: Map    0x0000 -> 0x3FFF
        //     0xC000 -> 0xFFFF: Mirror 0x0000 -> 0x3FFF
        // if PRGROM is 32KB
        //     CPU Address Bus          PRG ROM
        //     0x8000 -> 0xFFFF: Map    0x0000 -> 0x7FFF
        return match addr {
            0x8000..=0xFFFF => {
                let mapping: u32 = if self.0.prg_bank > 1 { 0x7FFF } else { 0x3FFF };
                *mapped_addr = addr as u32 & mapping;
                true
            }
            _ => false,
        };
    }

    fn allow_cpu_write(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        return match addr {
            0x8000..=0xFFFF => {
                let mapping: u32 = if self.0.prg_bank > 1 { 0x7FFF } else { 0x3FFF };
                *mapped_addr = addr as u32 & mapping;
                true
            }
            _ => false,
        };
    }

    //no mapping for ppu treat as RAM
    fn allow_ppu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        // There is no mapping required for PPU
        // PPU Address Bus          CHR ROM
        // 0x0000 -> 0x1FFF: Map    0x0000 -> 0x1FFF
        return match addr {
            0x0000..=0x1FFF => {
                *mapped_addr = addr as u32;
                true
            }
            _ => false,
        };
    }

    fn allow_ppu_write(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        return match addr {
            0x0000..=0x1FFF if self.0.chr_bank == 0 => {
                *mapped_addr = addr as u32;
                return true;
            }
            _ => false,
        };
    }
}
