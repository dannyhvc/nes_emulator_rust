use super::{
    addressing_mode::AddressingMode, instruction::Instruction, opcode::Opcode, processor::Cpu,
};

pub struct LookupTable(Box<Vec<Instruction>>);

impl LookupTable {
    pub fn new() -> LookupTable {
        let lookup = Box::new(Vec::<Instruction>::new());
        Self(lookup)
    }
}
