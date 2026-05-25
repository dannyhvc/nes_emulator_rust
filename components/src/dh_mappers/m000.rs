use std::ops::{Deref, DerefMut};

use crate::dh_mappers::dh_mapper::mapper::Mapper;
use crate::dh_mappers::traits::mapper_fn::MapperFn;

#[derive(Debug, Clone, Copy)]
pub struct M000(Mapper);
impl M000 {
    pub fn new(chr_bank: u8, prg_bank: u8) -> Self {
        Self(Mapper { chr_bank, prg_bank })
    }
}

impl Deref for M000 {
    type Target = Mapper;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for M000 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MapperFn for M000 {
    /// ```txt
    /// if PRGROM is 16KB:
    ///     CPU Address Bus          PRG ROM
    ///     0x8000 -> 0xBFFF: Map    0x0000 -> 0x3FFF
    ///     0xC000 -> 0xFFFF: Mirror 0x0000 -> 0x3FFF
    /// if PRGROM is 32KB:
    ///     CPU Address Bus          PRG ROM
    ///     0x8000 -> 0xFFFF: Map    0x0000 -> 0x7FFF
    /// ```
    fn allow_cpu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        match addr {
            0x8000..=0xFFFF => {
                let mapping: u32 =
                    if self.prg_bank > 1 { 0x7FFF } else { 0x3FFF };
                *mapped_addr = addr as u32 & mapping;
                true
            }
            _ => false,
        }
    }

    fn allow_cpu_write(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        match addr {
            0x8000..=0xFFFF => {
                let mapping: u32 =
                    if self.prg_bank > 1 { 0x7FFF } else { 0x3FFF };
                *mapped_addr = addr as u32 & mapping;
                true
            }
            _ => false,
        }
    }

    //no mapping for ppu treat as RAM
    fn allow_ppu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        // There is no mapping required for PPU
        // PPU Address Bus          CHR ROM
        // 0x0000 -> 0x1FFF: Map    0x0000 -> 0x1FFF
        match addr {
            0x0000..=0x1FFF => {
                *mapped_addr = addr as u32;
                true
            }
            _ => false,
        }
    }

    fn allow_ppu_write(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        match addr {
            0x0000..=0x1FFF if self.chr_bank == 0 => {
                *mapped_addr = addr as u32;
                true
            }
            _ => false,
        }
    }
}
