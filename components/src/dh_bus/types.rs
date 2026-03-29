#[cfg(features = "debug")]
use crate::dh_bus::ram_stats::RamAccessType;
use crate::dh_bus::ram_stats::{self, RamAccessType};

use crate::dh_cpu::CPU;
use crate::{END_OF_RAM, KB, START_OF_RAM};

#[derive(Debug, Clone, Hash)]
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
    pub fn load_instruction_mem(&mut self, data: Vec<Vec<u16>>) {
        // represents the index at which the entire instruction (opcode + operands)
        // will be stored at relatively to the instruction vec
        const CPU_INS_STORED_AT_INDX: usize = 0;
        // as follows the opcode index is stored directly after the address of the ins.
        const OPCODE_INDX: usize = 1;

        data.iter().for_each(|instruction| {
            //
            let segmented_ins: &[u16] = instruction.iter().as_slice();
            let mut ins_address: u16 = segmented_ins[CPU_INS_STORED_AT_INDX];
            let opcode = segmented_ins[OPCODE_INDX] as u8;

            // first part of the instruction is always the opcode address
            self.write(ins_address, opcode);

            if segmented_ins.len() > 2 {
                // move to the probable first operand
                ins_address += 1;

                // get the operands
                let operands = &segmented_ins[2..];
                operands.into_iter().for_each(|operand| {
                    // write each operand to the resulting incremented address
                    self.write(ins_address, *operand as u8);
                    ins_address += 1;
                });
            }
        });
    }

    /// Creates a new [`Bus`]. With 2Kb of MOS 6502 memory
    pub fn new() -> Self {
        Self {
            ram: [0u8; KB(64)],
            sys_clock_counter: 0,
        }
    }

    #[cfg(feature = "debug")]
    pub fn ram(&self) -> &[u8; KB(64)] {
        &self.ram
    }

    #[inline]
    pub fn read(&self, addr: u16, _b_read_only: bool) -> u8 {
        #[cfg(feature = "debug")]
        unsafe {
            crate::dh_bus::ram_stats::ADDRESS_HIT_COUNT
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
