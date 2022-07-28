use crate::devices::bus::Bus;
use super::processor::Cpu;

pub trait AddressingMode {
    fn IMP(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn ZP0(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn ZPY(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn ABS(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn ABY(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn IZX(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn IMM(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn ZPX(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn REL(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn ABX(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn IND(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn IZY(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
}
