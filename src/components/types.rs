use super::{bus::Bus, dh6502_cpu::M6502};
use custom_error::custom_error;
use std::{
    cell::RefCell,
    rc::Weak,
};

// TODO: add an actual error call hierarchy
custom_error! {
    CriticalCpuError
    Bad      = "Something bad happened",
    Terrible = "This is a very serious error!!!"
}

/* Weak refrence */
pub type Wref<T> = Weak<RefCell<T>>;
pub type OWref<T> = Option<Weak<RefCell<T>>>;
pub type RWref<V, E> = Result<Weak<RefCell<V>>, E>;

#[derive(PartialEq, Eq)]
pub enum M6502Flags {
    E = 0,      // Empty default
    C = 1 << 0, // Carry Bit
    Z = 1 << 1, // Zero
    I = 1 << 2, // Disable Interrupts
    D = 1 << 3, // Decimal Mode (unused in this implementation)
    B = 1 << 4, // Break
    U = 1 << 5, // Unused
    V = 1 << 6, // Overflow
    N = 1 << 7, // Negative
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
///
/// # Examples
///
/// ```
/// # use crate::components::M6502;
/// # use crate::bus::Bus;
/// # fn example() {
/// let instruction = M6502Instruction(
///     "LDA",
///     M6502::lda,
///     M6502::imm,
///     2,
/// );
///
/// assert_eq!(instruction.0, "LDA");
/// ```
pub struct M6502Instruction(
    pub &'static str,
    pub for<'a, 'b> fn(&'a mut M6502, &'b mut Bus) -> u8, // OPCODE
    pub for<'a, 'b> fn(&'a mut M6502, &'b mut Bus) -> u8, // ADDRESSING MODE
    pub u8,                                               // CYCLE COUNT
);

pub trait M6502Opcodes {
    fn adc(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn and(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn asl(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn bcc(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn bcs(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn beq(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn bit(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn bmi(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn bne(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn bpl(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn brk(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn bvc(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn bvs(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn clc(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn cld(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn cli(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn clv(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn cmp(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn cpx(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn cpy(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn dec(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn dex(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn dey(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn eor(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn inc(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn inx(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn iny(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn jmp(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn jsr(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn lda(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ldx(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ldy(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn lsr(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn nop(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ora(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn pha(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn php(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn pla(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn plp(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn rol(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ror(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn rti(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn rts(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn sbc(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn sec(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn sed(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn sei(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn sta(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn stx(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn sty(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn tax(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn tay(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn tsx(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn txa(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn txs(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn tya(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn xxx(cpu: &mut M6502, bus: &mut Bus) -> u8;
}

pub trait M6502AddrModes {
    fn imp(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn imm(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn zp0(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn zpx(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn zpy(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn abs(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn abx(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn aby(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn rel(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn ind(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn izx(cpu: &mut M6502, bus: &mut Bus) -> u8;
    fn izy(cpu: &mut M6502, bus: &mut Bus) -> u8;
}
