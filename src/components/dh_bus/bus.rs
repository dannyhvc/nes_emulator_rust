use crate::components::dh_bus::ram_stats::{self, RamAccessType};

use crate::components::dh_cpu::CPU;
use crate::components::{END_OF_RAM, KB, START_OF_RAM};

#[derive(Debug, Clone)]
pub struct BUS {
    ram: [u8; KB(64)],      // 2Kb of ram
    sys_clock_counter: u32, // motherboards clock for busses
}

impl Default for BUS {
    fn default() -> Self {
        BUS::new()
    }
}

impl BUS {
    #[inline]
    pub fn clock(&mut self, cpu: &mut CPU) {
        if self.sys_clock_counter % 3 == 0 {
            cpu.reset(self);
        }
        self.sys_clock_counter += 1;
    }

    #[cfg(feature = "debug")]
    pub fn load_instruction_mem(&mut self, data: Box<[Box<[u16]>]>) {
        const ADDRESS_INDEX: usize = 0;
        const OPCODE_INDEX: usize = 1;

        // go through each instruction
        for ins in data.iter() {
            let instruction_and_operands_bytes: &[u16] = ins.iter().as_slice();
            let mut address: u16 =
                instruction_and_operands_bytes[ADDRESS_INDEX];
            let opcode = instruction_and_operands_bytes[OPCODE_INDEX] as u8;

            // first part of the instruction is always the opcode address
            self.write(address, opcode);
            // move to the probable first operand
            address += 1;

            if instruction_and_operands_bytes.len() > 2 {
                let operands = &instruction_and_operands_bytes[2..];
                operands.into_iter().for_each(|operand| {
                    // write each operand to the resulting incremented address
                    self.write(address, *operand as u8);
                    address += 1;
                });
            }
        }
    }

    /// Creates a new [`Bus`]. With 2Kb of MOS 6502 memory
    pub fn new() -> Self {
        Self {
            ram: [0u8; KB(64)],
            sys_clock_counter: 0,
        }
    }

    #[cfg(feature = "debug")]
    pub fn ram(&self) -> [u8; KB(64)] {
        self.ram
    }

    #[inline]
    pub fn read(&self, addr: u16, _b_read_only: bool) -> u8 {
        #[cfg(feature = "debug")]
        unsafe {
            ram_stats::ADDRESS_HIT_COUNT
                .entry(addr)
                .or_insert_with(Vec::new)
                .push(RamAccessType::Read);
        }

        if addr >= START_OF_RAM && addr <= END_OF_RAM {
            return self.ram[addr as usize];
        }
        println!("Memory accessed out of bound: {:?}", addr);
        0x00
    }

    pub fn reset(&mut self, cpu: &mut CPU) {
        cpu.reset(self);
        self.sys_clock_counter = 0;
    }

    #[inline]
    pub fn write(&mut self, addr: u16, data: u8) {
        assert!(
            addr >= START_OF_RAM && addr <= END_OF_RAM,
            "can't write to address that is out of memory bounds"
        );

        #[cfg(feature = "debug")]
        unsafe {
            ram_stats::ADDRESS_HIT_COUNT
                .entry(addr)
                .or_insert_with(Vec::new)
                .push(RamAccessType::Write);
        }
        self.ram[addr as usize] = data;
    }
}
