use self::ins_mneumonic::InstructionMneumonic;
use super::{dh_bus::bus::BUS, dh_cpu::cpu::CPU};

/// ```no_run
/// E = 0       Empty Default
/// C = 1 << 0  Carry Bit
/// Z = 1 << 1  Zero
/// I = 1 << 2  Disable Interrupts
/// D = 1 << 3  Decminal Mode
/// B = 1 << 4  Break
/// U = 1 << 5  Unused
/// V = 1 << 6  Overflow
/// N = 1 << 7  Negative
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum CpuFlags {
    E = 0,      // Empty default
    C = 1 << 0, // Carry Bit
    Z = 1 << 1, // Zero
    I = 1 << 2, // Disable Interrupts
    D = 1 << 3, // Decimal Mode (unused in this implementation)
    B = 1 << 4, // Break
    U = 1 << 5, // UNUSED!!!!!!!!!!!!!
    V = 1 << 6, // Overflow
    N = 1 << 7, // Negative
}
impl Default for CpuFlags {
    fn default() -> Self {
        Self::E
    }
}

/// A struct representing an instruction for the MOS 6502 microprocessor.
///
/// This structure and the following vector are used to compile and store
/// the opcode translation table. The 6502 can effectively have 256
/// different instructions. Each of these are stored in a table in numerical
/// order so they can be looked up easily, with no decoding required.
/// This struct contains four fields:
///
/// - `0`: A string literal representing the mnemonic for the instruction.
/// - `1`: A function pointer representing the opcode implementation.
/// - `2`: A function pointer representing the addressing mode implementation.
/// - `3`: An unsigned 8-bit integer representing the cycle count for the instruction.
///
/// The `fn(&mut M6502, &mut Bus) -> u8` function pointers are expected to implement the
/// opcode and addressing mode logic for the instruction, respectively.
#[derive(Debug)]
pub struct CpuInstruction {
    pub mneumonic: InstructionMneumonic,
    pub op_code: for<'a, 'b> fn(&'a mut CPU, &'b mut BUS) -> u8, // OPCODE
    pub addr_mode: for<'a, 'b> fn(&'a mut CPU, &'b mut BUS) -> u8, // ADDRESSING MODE
    pub cycles: u8, // CYCLE COUNT
}

pub mod addr_mnuemonic;
pub mod addr_modes;
pub mod ins_mneumonic;
pub mod opcode_mneumonics;
pub mod opcodes;
