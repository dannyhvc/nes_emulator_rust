use super::addressing_mode::AddressingMode;
use super::flags::Flags_6502;
use super::instruction::Instruction;
use super::opcode::Opcode;

pub struct Cpu {
    pub status: u8,
    pub sp: u8,
    pub pc: u16,
    pub acc: u8,
    pub x: u8,
    pub y: u8,
    pub lookup: Box<Vec<Instruction>>,
}

impl Cpu {
    pub fn new() -> Self {
        let lookup = Box::new(vec![
            Instruction::new("BRK".to_string(), Some(Cpu::BRK), Some(Cpu::IMM), 7),
        ]);

        // read from instruction csv all the instruction text and convert them to coded instruction
        // structs to be passed as lookup instructions to the cpu.

        Self {
            status: 0u8,
            sp: 0u8,
            pc: 0u16,
            acc: 0u8,
            x: 0u8,
            y: 0u8,
            lookup,
        }
    }

    // settings status flags
    pub fn get_flag(&self, f: Flags_6502) -> u8 {
        return if (self.status & (f as u8)) > 0u8 {
            1u8
        } else {
            0u8
        };
    }

    pub fn set_flag(&mut self, f: Flags_6502, v: bool) {
        if v {
            self.status |= f as u8;
        } else {
            self.status &= !(f as u8);
        }
    }
}

impl Opcode for Cpu {}

impl AddressingMode for Cpu {}
