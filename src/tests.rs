#![allow(non_snake_case)]
use crate::{
    bs,
    components::{bus::Bus, dh_cpu::Cpu},
};
use rstest::*;

#[fixture]
fn cpu_fix() -> Cpu {
    Cpu::new()
}

#[fixture]
fn bus_fix() -> Bus {
    Bus::new()
}

#[rstest]
fn test_clock(mut cpu_fix: Cpu, mut bus_fix: Bus) {
    Cpu::reset(&mut cpu_fix, &bus_fix);
    for _ in 0..8 {
        Cpu::clock(&mut cpu_fix, &mut bus_fix);
    }
    assert!(cpu_fix.cycles() == 0);
}

#[rstest]
fn test_LDA(mut cpu_fix: Cpu, mut bus_fix: Bus) {
    Cpu::reset(&mut cpu_fix, &bus_fix);
    cpu_fix.set_cycles(0);

    cpu_fix.set_pc(0xFFFC);
    bus_fix.write(cpu_fix.pc(), 0xA9); // index 169/LDA/IMM of lookup table

    Cpu::clock(&mut cpu_fix, &mut bus_fix);
    cpu_fix.set_pc(10);
    Cpu::clock(&mut cpu_fix, &mut bus_fix);
    dbg!(cpu_fix);
}

#[rstest]
fn test_new() {
    let _cpu: Cpu = Cpu::new();
}

#[rstest]
fn test_disassemble(mut cpu_fix: Cpu, mut bus_fix: Bus) {
    const START: u16 = 0x0000;
    const STOP: u16 = 0x000f;

    Cpu::reset(&mut cpu_fix, &bus_fix);
    cpu_fix.set_cycles(0);

    for i in START..STOP {
        bus_fix.write(i, 0xa9); // 169 LDA
        Cpu::clock(&mut cpu_fix, &mut bus_fix);
        cpu_fix.set_pc(cpu_fix.pc() + 1);

        Cpu::clock(&mut cpu_fix, &mut bus_fix);

        // dbg!(cpu.opcode());
    }

    let dis_asm = Cpu::disassemble(&mut bus_fix, START, STOP);
    dbg!(dis_asm);
}

/// - `$C000`  `A5` `00`      ;LDA $00   Load the value at memory location $00 into the accumulator
/// - `$C002`  `85` `02`      ;STA $02   Store the value in the accumulator at memory location $02
/// - `$C004`  `A5` `01`      ;LDA $01   Load the value at memory location $01 into the accumulator
/// - `$C006`  `85` `03`      ;STA $03   Store the value in the accumulator at memory location $03
/// - `$C008`  `A5` `02`      ;LDA $02   Load the value at memory location $02 into the accumulator
/// - `$C00A`  `65` `03`      ;ADC $03   Add the value at memory location $03 to the accumulator
/// - `$C00C`  `85` `04`      ;STA $04   Store the result in memory location $04
/// - `$C00E`  `4C` `00` `C0` ;JMP $C000 Jump back to the instruction at memory location $C000
#[rstest]
fn test_mini_program(mut cpu_fix: Cpu, mut bus_fix: Bus) {
    const START: u16 = 0xC000;
    const STOP: u16 = 0xC00E;

    // "preloaded" data in ram
    bus_fix.write(0x00, 0xA);
    bus_fix.write(0x01, 0x14);
    bus_fix.write(0x02, 0x1E);
    bus_fix.write(0x03, 0x28);

    let tape = bs![
        //  addr        opc   operand(s)
        bs![0xC000_u16, 0xA5, 0x0],
        bs![0xC002_u16, 0x85, 0x2],
        bs![0xC004_u16, 0xA5, 0x1],
        bs![0xC006_u16, 0x85, 0x3],
        bs![0xC008_u16, 0xA5, 0x2],
        bs![0xC00A_u16, 0x65, 0x3],
        bs![0xC00C_u16, 0x85, 0x4],
        bs![0xC00E_u16, 0x4C, 0x00, 0x0C]
    ];

    // is there a better way to do this?
    Cpu::reset(&mut cpu_fix, &bus_fix);
    bus_fix.load_instruction_mem(tape);
    cpu_fix.set_cycles(0);
    let disasm = Cpu::disassemble(&mut bus_fix, START, STOP);

    dbg!(disasm);
}

#[rstest]
fn test_gex_fmt() {
    let string_rep: String = format!("#${:x} {{imm}}", 100u8 as u32);
    dbg!(string_rep);
}
