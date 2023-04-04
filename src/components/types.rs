use std::{
    cell::{Ref, RefCell},
    rc::{Rc, Weak},
};

use super::{bus::Bus, dh6502::M6502};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CriticalCpuError {
    Segfault,
    StackOverflow,
    Race,
}

/* Weak refrence */
pub type Wref<T> = Weak<RefCell<T>>;
pub type OWref<T> = Option<Weak<RefCell<T>>>;
pub type RWref<V, E> = Result<Weak<RefCell<V>>, E>;

#[derive(PartialEq, Eq)]
pub enum M6502Flags {
    E = (0),      // Empty default
    C = (1 << 0), // Carry Bit
    Z = (1 << 1), // Zero
    I = (1 << 2), // Disable Interrupts
    D = (1 << 3), // Decimal Mode (unused in this implementation)
    B = (1 << 4), // Break
    U = (1 << 5), // Unused
    V = (1 << 6), // Overflow
    N = (1 << 7), // Negative
}

/**
 * TODO:
 *  add ppu
 *  add apu
 *  add cartrages
 *  add i/o
 */
pub struct NesDeviceStack {
    cpu: Rc<RefCell<M6502>>,
}

/**
```This structure and the following vector are used to compile and store
the opcode translation table. The 6502 can effectively have 256
different instructions. Each of these are stored in a table in numerical
order so they can be looked up easily, with no decoding required.

Each table entry holds:
   Pneumonic : A textual representation of the instruction (used for disassembly)
   Opcode Function: A function pointer to the implementation of the opcode
   Opcode Address Mode : A function pointer to the implementation of the
                         addressing mechanism used by the instruction
   Cycle Count : An integer that represents the base number of clock cycles the
                 self requires to perform the instruction
```
*/
pub struct M6502Instruction(
    pub &'static str, // TODO: might need to be a ref idk yet
    pub for<'a, 'b> fn(&'a mut M6502, &'b mut Bus) -> u8, // OPCODE
    pub for<'a, 'b> fn(&'a mut M6502, &'b mut Bus) -> u8, // ADDRESSING MODE
    pub u8,           // CYCLE COUNT
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
