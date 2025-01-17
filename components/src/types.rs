#![allow(non_snake_case)]
// use self::instruction_mneumonic::InstructionMneumonic;
use super::{dh_bus::BUS, dh_cpu::CPU};

/// ```no_run
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
pub enum CpuFlag {
    C = 0x01, // 1 << 0, // Carry Bit
    Z = 0x02, // 1 << 1, // Zero
    I = 0x04, // 1 << 2, // Disable Interrupts
    D = 0x08, // 1 << 3, // Decimal Mode (unused in this implementation)
    B = 0x10, // 1 << 4, // Break
    U = 0x20, // 1 << 5, // UNUSED!!!!!!!!!!!!!
    V = 0x40, // 1 << 6, // Overflow
    N = 0x80, // 1 << 7, // Negative
}

impl Into<u8> for CpuFlag {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for CpuFlag {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::C),
            0x02 => Ok(Self::Z),
            0x04 => Ok(Self::I),
            0x08 => Ok(Self::D),
            0x10 => Ok(Self::B),
            0x20 => Ok(Self::U),
            0x40 => Ok(Self::V),
            0x80 => Ok(Self::N),
            _ => Err(format!("Cannot convert value into a CpuFlag")),
        }
    }
}

impl Default for CpuFlag {
    fn default() -> Self {
        Self::U
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
    pub mneumonic: Mneumonic,
    pub op_code: for<'a, 'b> fn(&'a mut CPU, &'b mut BUS) -> u8, // OPCODE
    pub addr_mode: for<'a, 'b> fn(&'a mut CPU, &'b mut BUS) -> u8, // ADDRESSING MODE
    pub cycles: u8, // CYCLE COUNT
}

/// `InstructionMneumonic` is a structure that represents the mnemonic of an instruction.
///
/// # Fields
///
/// * `name: &'static str` - This field represents the name of the instruction mnemonic.
/// * `op_code`: [`OpcodeMneumonic`] - This field represents the opcode of the instruction mnemonic.
/// * `am_name`: [`AddrModeMneumonic`] - This field represents the addressing mode of the instruction mnemonic.
#[derive(Debug)]
pub struct Mneumonic {
    pub name: &'static str,
    pub op_code: OpcodeMneumonic,
    pub am_name: AddressingModeMneumonic,
}
impl Mneumonic {
    pub fn new(
        name: &'static str,
        op_name: OpcodeMneumonic,
        am_name: AddressingModeMneumonic,
    ) -> Self {
        Self {
            name,
            op_code: op_name,
            am_name,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AddressingModeMneumonic {
    IMP,
    IMM,
    ZP0,
    ZPX,
    ZPY,
    ABS,
    ABX,
    ABY,
    REL,
    IND,
    IZX,
    IZY,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpcodeMneumonic {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    XXX,
}

pub trait AddressingMode {
    fn IMP(&mut self, bus: &mut BUS) -> u8;
    fn IMM(&mut self, bus: &mut BUS) -> u8;
    fn ZP0(&mut self, bus: &mut BUS) -> u8;
    fn ZPX(&mut self, bus: &mut BUS) -> u8;
    fn ZPY(&mut self, bus: &mut BUS) -> u8;
    fn ABS(&mut self, bus: &mut BUS) -> u8;
    fn ABX(&mut self, bus: &mut BUS) -> u8;
    fn ABY(&mut self, bus: &mut BUS) -> u8;
    fn REL(&mut self, bus: &mut BUS) -> u8;
    fn IND(&mut self, bus: &mut BUS) -> u8;
    fn IZX(&mut self, bus: &mut BUS) -> u8;
    fn IZY(&mut self, bus: &mut BUS) -> u8;
}

pub trait Opcode {
    fn ADC(&mut self, bus: &mut BUS) -> u8;
    fn AND(&mut self, bus: &mut BUS) -> u8;
    fn ASL(&mut self, bus: &mut BUS) -> u8;
    fn BCC(&mut self, bus: &mut BUS) -> u8;
    fn BCS(&mut self, bus: &mut BUS) -> u8;
    fn BEQ(&mut self, bus: &mut BUS) -> u8;
    fn BIT(&mut self, bus: &mut BUS) -> u8;
    fn BMI(&mut self, bus: &mut BUS) -> u8;
    fn BNE(&mut self, bus: &mut BUS) -> u8;
    fn BPL(&mut self, bus: &mut BUS) -> u8;
    fn BRK(&mut self, bus: &mut BUS) -> u8;
    fn BVC(&mut self, bus: &mut BUS) -> u8;
    fn BVS(&mut self, bus: &mut BUS) -> u8;
    fn CLC(&mut self, bus: &mut BUS) -> u8;
    fn CLD(&mut self, bus: &mut BUS) -> u8;
    fn CLI(&mut self, bus: &mut BUS) -> u8;
    fn CLV(&mut self, bus: &mut BUS) -> u8;
    fn CMP(&mut self, bus: &mut BUS) -> u8;
    fn CPX(&mut self, bus: &mut BUS) -> u8;
    fn CPY(&mut self, bus: &mut BUS) -> u8;
    fn DEC(&mut self, bus: &mut BUS) -> u8;
    fn DEX(&mut self, bus: &mut BUS) -> u8;
    fn DEY(&mut self, bus: &mut BUS) -> u8;
    fn EOR(&mut self, bus: &mut BUS) -> u8;
    fn INC(&mut self, bus: &mut BUS) -> u8;
    fn INX(&mut self, bus: &mut BUS) -> u8;
    fn INY(&mut self, bus: &mut BUS) -> u8;
    fn JMP(&mut self, bus: &mut BUS) -> u8;
    fn JSR(&mut self, bus: &mut BUS) -> u8;
    fn LDA(&mut self, bus: &mut BUS) -> u8;
    fn LDX(&mut self, bus: &mut BUS) -> u8;
    fn LDY(&mut self, bus: &mut BUS) -> u8;
    fn LSR(&mut self, bus: &mut BUS) -> u8;
    fn NOP(&mut self, bus: &mut BUS) -> u8;
    fn ORA(&mut self, bus: &mut BUS) -> u8;
    fn PHA(&mut self, bus: &mut BUS) -> u8;
    fn PHP(&mut self, bus: &mut BUS) -> u8;
    fn PLA(&mut self, bus: &mut BUS) -> u8;
    fn PLP(&mut self, bus: &mut BUS) -> u8;
    fn ROL(&mut self, bus: &mut BUS) -> u8;
    fn ROR(&mut self, bus: &mut BUS) -> u8;
    fn RTI(&mut self, bus: &mut BUS) -> u8;
    fn RTS(&mut self, bus: &mut BUS) -> u8;
    fn SBC(&mut self, bus: &mut BUS) -> u8;
    fn SEC(&mut self, bus: &mut BUS) -> u8;
    fn SED(&mut self, bus: &mut BUS) -> u8;
    fn SEI(&mut self, bus: &mut BUS) -> u8;
    fn STA(&mut self, bus: &mut BUS) -> u8;
    fn STX(&mut self, bus: &mut BUS) -> u8;
    fn STY(&mut self, bus: &mut BUS) -> u8;
    fn TAX(&mut self, bus: &mut BUS) -> u8;
    fn TAY(&mut self, bus: &mut BUS) -> u8;
    fn TSX(&mut self, bus: &mut BUS) -> u8;
    fn TXA(&mut self, bus: &mut BUS) -> u8;
    fn TXS(&mut self, bus: &mut BUS) -> u8;
    fn TYA(&mut self, bus: &mut BUS) -> u8;
    fn XXX(&mut self, bus: &mut BUS) -> u8;
}
