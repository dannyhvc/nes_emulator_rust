use crate::components::dh_bus::bus::BUS;

pub trait M6502AddrModes {
    fn IMP(&mut self, bus: &mut BUS) -> u8;
    fn IMM(&mut self, bus: &mut BUS) -> u8;
    fn ZP0(&mut self, bus: &mut BUS) -> u8;
    fn ZPX(&mut self, bus: &mut BUS) -> u8;
    fn ZPY(&mut self, bus: &mut BUS) -> u8;
    fn ABS(&mut self, bus: &mut BUS) -> u8;
    fn ABX(&mut self, bus: &mut BUS) -> u8;
    fn ABY(&mut self, bus: &mut BUS) -> u8;
    fn REL(&mut self, bus: &mut BUS) -> u8;
    fn IND(&mut self, bus: &mut BUS) -> u8;
    fn IZX(&mut self, bus: &mut BUS) -> u8;
    fn IZY(&mut self, bus: &mut BUS) -> u8;
}
