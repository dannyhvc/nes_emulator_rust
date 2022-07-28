use crate::devices::bus::Bus;
use super::processor::Cpu;

pub struct Instruction {
    pub name: String,
    pub operate: Option<fn(&mut Cpu, &mut Bus) -> u8>,
    pub addrmode: Option<fn(&mut Cpu, &mut Bus) -> u8>,
    pub cycles: i32,
}

impl Instruction {
    pub fn new(
        name: String,
        operate: Option<fn(&mut Cpu, &mut Bus) -> u8>,
        addrmode: Option<fn(&mut Cpu, &mut Bus) -> u8>,
        cycles: i32,
    ) -> Self {
        Self {
            name,
            operate,
            addrmode,
            cycles,
        }
    }
}
