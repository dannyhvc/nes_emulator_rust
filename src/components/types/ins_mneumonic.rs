use super::{
    addr_mnuemonic::AddrModeMneumonic, opcode_mneumonics::OpcodeMneumonic,
};

/// `InstructionMneumonic` is a structure that represents the mnemonic of an instruction.
///
/// # Fields
///
/// * `name: &'static str` - This field represents the name of the instruction mnemonic.
/// * `op_code`: [`OpcodeMneumonic`] - This field represents the opcode of the instruction mnemonic.
/// * `am_name`: [`AddrModeMneumonic`] - This field represents the addressing mode of the instruction mnemonic.
#[derive(Debug)]
pub struct InstructionMneumonic {
    pub name: &'static str,
    pub op_code: OpcodeMneumonic,
    pub am_name: AddrModeMneumonic,
}
impl InstructionMneumonic {
    pub fn new(
        name: &'static str,
        op_name: OpcodeMneumonic,
        am_name: AddrModeMneumonic,
    ) -> Self {
        Self {
            name,
            op_code: op_name,
            am_name,
        }
    }
}
