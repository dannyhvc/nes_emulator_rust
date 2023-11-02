use super::{dh_cpu::CPU, END_OF_RAM, KB, START_OF_RAM};

#[derive(Debug, Clone)]
pub struct Bus {
    pub cpu_ram: [u8; KB(64)],  // 2Kb of ram
    pub sys_clock_counter: u32, // motherboards clock for busses
}
impl Bus {
    /// Creates a new [`Bus`]. With 2Kb of MOS 6502 memory
    pub fn new() -> Self {
        Self {
            cpu_ram: [0u8; KB(64)],
            sys_clock_counter: 0,
        }
    }

    #[inline]
    pub fn read(&self, addr: u16, _b_read_only: bool) -> u8 {
        if addr >= START_OF_RAM && addr <= END_OF_RAM {
            return self.cpu_ram[addr as usize];
        }
        0x00
    }

    #[inline]
    pub fn write(&mut self, addr: u16, data: u8) {
        if addr >= START_OF_RAM && addr <= END_OF_RAM {
            self.cpu_ram[addr as usize] = data;
        }
    }

    #[inline]
    pub fn clock(&mut self, cpu: &mut CPU) {
        if self.sys_clock_counter % 3 == 0 {
            CPU::reset(cpu, self);
        }
        self.sys_clock_counter += 1;
    }

    pub fn reset(&mut self, cpu: &mut CPU) {
        CPU::reset(cpu, self);
        self.sys_clock_counter = 0;
    }

    #[cfg(feature = "debug")]
    pub fn load_instruction_mem(&mut self, data: Box<[Box<[u16]>]>) {
        // we subtract 1 because we want to exclude the instruction location by default

        // TODO: keep this for now so we can test dan;s brain
        use super::types::AddrModeMneumonic;
        use crate::components::types::OpcodeMneumonic;

        for ins in data.iter() {
            let current_instruction: &[u16] = ins.iter().as_slice();
            assert!(current_instruction.len() > 1, "instruction invalid");

            let ins_addr: u16 = current_instruction[0];
            let opcode: u8 = current_instruction[1] as u8;
            match current_instruction.len() - 1 {
                // 0 byte operand
                1 => {}

                // 1 byte operand
                2 => {
                    let operand: u8 = current_instruction[2] as u8;
                }

                // 2 byte operand
                3 => {
                    let high_byte_operand: u8 = current_instruction[2] as u8;
                    let low_byte_operand: u8 = current_instruction[3] as u8;
                }
                _ => panic!("Instruction operand length can be either 0,1, or 2 bytes"),
            };
        }
    }
}
