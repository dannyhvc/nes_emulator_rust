#![allow(non_snake_case)]
use super::{bus::Bus, dh6502_cpu::M6502};

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
    pub op_code: for<'a, 'b> fn(&'a mut M6502, &'b mut Bus) -> u8, // OPCODE
    pub addr_mode: for<'a, 'b> fn(&'a mut M6502, &'b mut Bus) -> u8, // ADDRESSING MODE
    pub cycles: u8,                                                // CYCLE COUNT
}

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
    pub fn new(name: &'static str, op_name: OpcodeMneumonic, am_name: AddrModeMneumonic) -> Self {
        Self {
            name,
            op_code: op_name,
            am_name,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
impl Default for OpcodeMneumonic {
    fn default() -> Self {
        Self::XXX
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddrModeMneumonic {
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
impl Default for AddrModeMneumonic {
    fn default() -> Self {
        Self::IMP
    }
}

pub trait M6502Opcodes {
    fn ADC(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn AND(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ASL(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn BCC(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn BCS(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn BEQ(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn BIT(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn BMI(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn BNE(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn BPL(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn BRK(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn BVC(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn BVS(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn CLC(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn CLD(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn CLI(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn CLV(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn CMP(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn CPX(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn CPY(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn DEC(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn DEX(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn DEY(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn EOR(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn INC(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn INX(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn INY(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn JMP(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn JSR(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn LDA(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn LDX(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn LDY(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn LSR(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn NOP(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ORA(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn PHA(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn PHP(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn PLA(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn PLP(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ROL(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ROR(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn RTI(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn RTS(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn SBC(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn SEC(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn SED(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn SEI(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn STA(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn STX(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn STY(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn TAX(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn TAY(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn TSX(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn TXA(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn TXS(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn TYA(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn XXX(cpu: &mut M6502, bus: &mut Bus) -> u8;
}

pub trait M6502AddrModes {
    fn IMP(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn IMM(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ZP0(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ZPX(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ZPY(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ABS(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ABX(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ABY(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn REL(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn IND(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn IZX(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn IZY(cpu: &mut M6502, bus: &mut Bus) -> u8;
}
