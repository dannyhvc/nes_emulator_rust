#[cfg(feature = "debug")]
pub mod ram_stats;
use log::info;
#[cfg(feature = "debug")]
use ram_stats::RamAccessType;
#[cfg(feature = "debug")]
use ram_stats::ADDRESS_HIT_COUNT;

use crate::dh_cpu::CPU;
use crate::{END_OF_RAM, KiB, START_OF_RAM};
use eyre::Result as ErrorOr;

#[derive(Debug, Clone, Hash)]
pub struct BUS {
    pub ram: [u8; KiB(64)],  // 2Kb of ram
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
    #[deprecated]
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

    /// load_program
    /// -----
    ///
    /// Loads a string of hex bytes into memory so that it can be executed.
    #[cfg(feature = "debug")]
    pub fn load_program(
        &mut self,
        data: &str,
        offset: &mut usize,
    ) -> ErrorOr<()> {
        // It automatically handles double spaces, tabs, and newlines
        for hex_str in data.split_whitespace() {
            // Prevent a panic if your program exceeds RAM limits
            if *offset >= self.ram.len() {
                panic!(
                    "load_program exceeded RAM capacity at offset {}",
                    offset
                );
            }

            // Parse directly to u8 instead of u16
            // Using match or expect gives a clearer error if typo'd a hex value.
            let byte = u8::from_str_radix(hex_str, 16)
                .expect(&format!("Failed to parse hex string: '{}'", hex_str));

            self.ram[*offset] = byte;
            *offset += 1;
        }

        Ok(())
    }

    /// Creates a new [`Bus`]. With 2Kb of MOS 6502 memory
    pub fn new() -> Self {
        Self {
            ram: [0u8; KiB(64)],
            sys_clock_counter: 0,
        }
    }

    #[cfg(feature = "debug")]
    pub fn ram(&self) -> &[u8; KiB(64)] {
        &self.ram
    }

    #[inline]
    pub fn read(&self, addr: u16, _b_read_only: bool) -> u8 {
        #[cfg(feature = "debug")]
        {
            unsafe {
                ADDRESS_HIT_COUNT
                    .entry(addr)
                    .or_insert_with(Vec::new)
                    .push(RamAccessType::Read);
            }
        }

        if addr >= START_OF_RAM && addr <= END_OF_RAM {
            info!(
                "RAM Accessed - ram[0x{addr:X} 0d{addr}] ==> {:X} 0d{}",
                self.ram[addr as usize], self.ram[addr as usize]
            );
            return self.ram[addr as usize];
        }
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
        {
            unsafe {
                ADDRESS_HIT_COUNT
                    .entry(addr)
                    .or_insert_with(Vec::new)
                    .push(RamAccessType::Write);
            }
        }
        self.ram[addr as usize] = data;
    }
}
