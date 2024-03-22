pub mod bus;
pub mod cartridge;
pub mod dh_cpu;
pub mod dh_ppu;
pub mod mappers;
pub mod types;

use self::dh_cpu::Cpu;
use self::types::AddrModeMneumonic;
use self::types::CpuInstruction;
use self::types::M6502AddrModes;
use self::types::M6502Opcodes;
use self::types::OpcodeMneumonic;
use crate::components::types::InstructionMneumonic;
use once_cell::sync::Lazy;

const START_OF_RAM: u16 = 0x0000;
const END_OF_RAM: u16 = 0xFFFF;
const LOW_BYTE: u16 = 0x00FF;
const HIGH_BYTE: u16 = 0xFF00;
const TOP_BIT_THRESH: u16 = 0x0080;

#[allow(non_snake_case)]
#[inline(always)]
pub const fn KB(n: u32) -> usize {
    const SIZEOF_1KB: u32 = 1024;
    return (n * SIZEOF_1KB) as usize;
}

macro_rules! imneumonic {
    ($op_code_ident: ident, $am_name: ident) => {
        InstructionMneumonic::new(
            stringify!($op_code_ident),
            OpcodeMneumonic::$op_code_ident,
            AddrModeMneumonic::$am_name,
        )
    };
}

macro_rules! cins {
    ($op_code_ident:ident $am_name:ident $cycles:literal) => {
        CpuInstruction {
            mneumonic: imneumonic!($op_code_ident, $am_name),
            op_code: Cpu::$op_code_ident,
            addr_mode: Cpu::$am_name,
            cycles: $cycles,
        }
    };
}

// mos 6502 lookup table
static LOOKUP_TABLE: Lazy<[CpuInstruction; 256]> = Lazy::new(|| {
    [
        //    OP  AD  C
        cins!(BRK IMM 7), //expands to: CINS{mneumonic: imneumonic!(BRK,IMM), op: M6502::BRK, am: M6502::IMM, cycles: 7},
        cins!(ORA IZX 6),
        cins!(XXX IMP 2), // illegal opcode
        cins!(XXX IMP 8), // illegal opcode
        cins!(NOP IMP 3),
        cins!(ORA ZP0 3),
        cins!(ASL ZP0 5),
        cins!(XXX IMP 5), // illegal opcode
        cins!(PHP IMP 3),
        cins!(ORA IMM 2),
        cins!(ASL IMP 2),
        cins!(XXX IMP 2), // illegal opcode
        cins!(NOP IMP 4),
        cins!(ORA ABS 4),
        cins!(ASL ABS 6),
        cins!(XXX IMP 6), // illegal opcode
        cins!(BPL REL 2),
        cins!(ORA IZY 5),
        cins!(XXX IMP 2), // illegal opcode
        cins!(XXX IMP 8), // illegal opcode
        cins!(NOP IMP 4),
        cins!(ORA ZPX 4),
        cins!(ASL ZPX 6),
        cins!(XXX IMP 6), // illegal opcode
        cins!(CLC IMP 2),
        cins!(ORA ABY 4),
        cins!(NOP IMP 2),
        cins!(XXX IMP 7), // illegal opcode
        cins!(NOP IMP 4),
        cins!(ORA ABX 4),
        cins!(ASL ABX 7),
        cins!(XXX IMP 7), // illegal opcode
        cins!(JSR ABS 6),
        cins!(AND IZX 6),
        cins!(XXX IMP 2), // illegal opcode
        cins!(XXX IMP 8), // illegal opcode
        cins!(BIT ZP0 3),
        cins!(AND ZP0 3),
        cins!(ROL ZP0 5),
        cins!(XXX IMP 5), // illegal opcode
        cins!(PLP IMP 4),
        cins!(AND IMM 2),
        cins!(ROL IMP 2),
        cins!(XXX IMP 2), // illegal opcode
        cins!(BIT ABS 4),
        cins!(AND ABS 4),
        cins!(ROL ABS 6),
        cins!(XXX IMP 6), // illegal opcode
        cins!(BMI REL 2),
        cins!(AND IZY 5),
        cins!(XXX IMP 2), // illegal opcode
        cins!(XXX IMP 8), // illegal opcode
        cins!(NOP IMP 4),
        cins!(AND ZPX 4),
        cins!(ROL ZPX 6),
        cins!(XXX IMP 6), // illegal opcode
        cins!(SEC IMP 2),
        cins!(AND ABY 4),
        cins!(NOP IMP 2),
        cins!(XXX IMP 7), // illegal opcode
        cins!(NOP IMP 4),
        cins!(AND ABX 4),
        cins!(ROL ABX 7),
        cins!(XXX IMP 7), // illegal opcode
        cins!(RTI IMP 6),
        cins!(EOR IZX 6),
        cins!(XXX IMP 2), // illegal opcode
        cins!(XXX IMP 8), // illegal opcode
        cins!(NOP IMP 3),
        cins!(EOR ZP0 3),
        cins!(LSR ZP0 5),
        cins!(XXX IMP 5), // illegal opcode
        cins!(PHA IMP 3),
        cins!(EOR IMM 2),
        cins!(LSR IMP 2),
        cins!(XXX IMP 2), // illegal opcode
        cins!(JMP ABS 3),
        cins!(EOR ABS 4),
        cins!(LSR ABS 6),
        cins!(XXX IMP 6), // illegal opcode
        cins!(BVC REL 2),
        cins!(EOR IZY 5),
        cins!(XXX IMP 2), // illegal opcode
        cins!(XXX IMP 8), // illegal opcode
        cins!(NOP IMP 4),
        cins!(EOR ZPX 4),
        cins!(LSR ZPX 6),
        cins!(XXX IMP 6), // illegal opcode
        cins!(CLI IMP 2),
        cins!(EOR ABY 4),
        cins!(NOP IMP 2),
        cins!(XXX IMP 7), // illegal opcode
        cins!(NOP IMP 4),
        cins!(EOR ABX 4),
        cins!(LSR ABX 7),
        cins!(XXX IMP 7), // illegal opcode
        cins!(RTS IMP 6),
        cins!(ADC IZX 6),
        cins!(XXX IMP 2), // illegal opcode
        cins!(XXX IMP 8), // illegal opcode
        cins!(NOP IMP 3),
        cins!(ADC ZP0 3),
        cins!(ROR ZP0 5),
        cins!(XXX IMP 5), // illegal opcode
        cins!(PLA IMP 4),
        cins!(ADC IMM 2),
        cins!(ROR IMP 2),
        cins!(XXX IMP 2), // illegal opcode
        cins!(JMP IND 5),
        cins!(ADC ABS 4),
        cins!(ROR ABS 6),
        cins!(XXX IMP 6), // illegal opcode
        cins!(BVS REL 2),
        cins!(ADC IZY 5),
        cins!(XXX IMP 2), // illegal opcode
        cins!(XXX IMP 8), // illegal opcode
        cins!(NOP IMP 4),
        cins!(ADC ZPX 4),
        cins!(ROR ZPX 6),
        cins!(XXX IMP 6), // illegal opcode
        cins!(SEI IMP 2),
        cins!(ADC ABY 4),
        cins!(NOP IMP 2),
        cins!(XXX IMP 7), // illegal opcode
        cins!(NOP IMP 4),
        cins!(ADC ABX 4),
        cins!(ROR ABX 7),
        cins!(XXX IMP 7), // illegal opcode
        cins!(NOP IMP 2),
        cins!(STA IZX 6),
        cins!(NOP IMP 2),
        cins!(XXX IMP 6), // illegal opcode
        cins!(STY ZP0 3),
        cins!(STA ZP0 3),
        cins!(STX ZP0 3),
        cins!(XXX IMP 3), // illegal opcode
        cins!(DEY IMP 2),
        cins!(NOP IMP 2),
        cins!(TXA IMP 2),
        cins!(XXX IMP 2), // illegal opcode
        cins!(STY ABS 4),
        cins!(STA ABS 4),
        cins!(STX ABS 4),
        cins!(XXX IMP 4), // illegal opcode
        cins!(BCC REL 2),
        cins!(STA IZY 6),
        cins!(XXX IMP 2), // illegal opcode
        cins!(XXX IMP 6), // illegal opcode
        cins!(STY ZPX 4),
        cins!(STA ZPX 4),
        cins!(STX ZPY 4),
        cins!(XXX IMP 4), // illegal opcode
        cins!(TYA IMP 2),
        cins!(STA ABY 5),
        cins!(TXS IMP 2),
        cins!(XXX IMP 5), // illegal opcode
        cins!(NOP IMP 5),
        cins!(STA ABX 5),
        cins!(XXX IMP 5), // illegal opcode
        cins!(XXX IMP 5), // illegal opcode
        cins!(LDY IMM 2),
        cins!(LDA IZX 6),
        cins!(LDX IMM 2),
        cins!(XXX IMP 6), // illegal opcode
        cins!(LDY ZP0 3),
        cins!(LDA ZP0 3),
        cins!(LDX ZP0 3),
        cins!(XXX IMP 3), // illegal opcode
        cins!(TAY IMP 2),
        cins!(LDA IMM 2),
        cins!(TAX IMP 2),
        cins!(XXX IMP 2), // illegal opcode
        cins!(LDY ABS 4),
        cins!(LDA ABS 4),
        cins!(LDX ABS 4),
        cins!(XXX IMP 4), // illegal opcode
        cins!(BCS REL 2),
        cins!(LDA IZY 5),
        cins!(XXX IMP 2), // illegal opcode
        cins!(XXX IMP 5), // illegal opcode
        cins!(LDY ZPX 4),
        cins!(LDA ZPX 4),
        cins!(LDX ZPY 4),
        cins!(XXX IMP 4), // illegal opcode
        cins!(CLV IMP 2),
        cins!(LDA ABY 4),
        cins!(TSX IMP 2),
        cins!(XXX IMP 4), // illegal opcode
        cins!(LDY ABX 4),
        cins!(LDA ABX 4),
        cins!(LDX ABY 4),
        cins!(XXX IMP 4), // illegal opcode
        cins!(CPY IMM 2),
        cins!(CMP IZX 6),
        cins!(NOP IMP 2),
        cins!(XXX IMP 8), // illegal opcode
        cins!(CPY ZP0 3),
        cins!(CMP ZP0 3),
        cins!(DEC ZP0 5),
        cins!(XXX IMP 5), // illegal opcode
        cins!(INY IMP 2),
        cins!(CMP IMM 2),
        cins!(DEX IMP 2),
        cins!(XXX IMP 2), // illegal opcode
        cins!(CPY ABS 4),
        cins!(CMP ABS 4),
        cins!(DEC ABS 6),
        cins!(XXX IMP 6), // illegal opcode
        cins!(BNE REL 2),
        cins!(CMP IZY 5),
        cins!(XXX IMP 2), // illegal opcode
        cins!(XXX IMP 8), // illegal opcode
        cins!(NOP IMP 4),
        cins!(CMP ZPX 4),
        cins!(DEC ZPX 6),
        cins!(XXX IMP 6), // illegal opcode
        cins!(CLD IMP 2),
        cins!(CMP ABY 4),
        cins!(NOP IMP 2),
        cins!(XXX IMP 7), // illegal opcode
        cins!(NOP IMP 4),
        cins!(CMP ABX 4),
        cins!(DEC ABX 7),
        cins!(XXX IMP 7), // illegal opcode
        cins!(CPX IMM 2),
        cins!(SBC IZX 6),
        cins!(NOP IMP 2),
        cins!(XXX IMP 8), // illegal opcode
        cins!(CPX ZP0 3),
        cins!(SBC ZP0 3),
        cins!(INC ZP0 5),
        cins!(XXX IMP 5), // illegal opcode
        cins!(INX IMP 2),
        cins!(SBC IMM 2),
        cins!(NOP IMP 2),
        cins!(SBC IMP 2),
        cins!(CPX ABS 4),
        cins!(SBC ABS 4),
        cins!(INC ABS 6),
        cins!(XXX IMP 6), // illegal opcode
        cins!(BEQ REL 2),
        cins!(SBC IZY 5),
        cins!(XXX IMP 2), // illegal opcode
        cins!(XXX IMP 8), // illegal opcode
        cins!(NOP IMP 4),
        cins!(SBC ZPX 4),
        cins!(INC ZPX 6),
        cins!(XXX IMP 6), // illegal opcode
        cins!(SED IMP 2),
        cins!(SBC ABY 4),
        cins!(NOP IMP 2),
        cins!(XXX IMP 7), // illegal opcode
        cins!(NOP IMP 4),
        cins!(SBC ABX 4),
        cins!(INC ABX 7),
        cins!(XXX IMP 7), // illegal opcode
    ]
});
