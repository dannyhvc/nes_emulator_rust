pub trait MapperFn {
    fn allow_cpu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn allow_cpu_write(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn allow_ppu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn allow_ppu_write(&self, addr: u16, mapped_addr: &mut u32) -> bool;
}
