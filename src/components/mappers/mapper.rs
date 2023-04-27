pub trait MapperFn {
    fn new(prg_bank: u8, chr_bank: u8) -> Self;
    fn allow_cpu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn allow_cpu_write(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn allow_ppu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn allow_ppu_write(&self, addr: u16, mapped_addr: &mut u32) -> bool;
}

pub struct MapperData {
    pub prg_bank: u8,
    pub chr_bank: u8,
}
impl MapperData {
    pub fn new(prg_bank: u8, chr_bank: u8) -> Self {
        Self { prg_bank, chr_bank }
    }
}
impl Default for MapperData {
    fn default() -> Self {
        Self {
            prg_bank: 0u8,
            chr_bank: 0u8,
        }
    }
}
