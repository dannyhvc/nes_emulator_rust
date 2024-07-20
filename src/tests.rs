#![allow(non_snake_case)]
use crate::{
    bs,
    components::{
        dh_bus::{self, BUS},
        dh_cpu::{self, CPU},
    },
};
use rstest::{fixture, rstest};

#[fixture]
fn cpu() -> dh_cpu::CPU {
    let mut cpu = CPU::new();
    let temp_bus = BUS::new();
    cpu.reset(&temp_bus);
    cpu.set_cycles(0);
    cpu
}

#[fixture]
fn bus() -> dh_bus::BUS {
    BUS::new()
}

// #[rstest]
fn test_clock(mut cpu: dh_cpu::CPU, mut bus: dh_bus::BUS) {
    cpu.reset(&bus);
    for _ in 0..8 {
        cpu.clock(&mut bus);
    }
    assert!(cpu.cycles() == 0);
}

// #[rstest]
fn test_LDA(mut cpu: dh_cpu::CPU, mut bus: dh_bus::BUS) {
    cpu.set_pc(0xFFFC);
    bus.write(cpu.pc(), 0xA9); // index 169/LDA/IMM of lookup table

    cpu.clock(&mut bus);
    cpu.set_pc(10);
    cpu.clock(&mut bus);
    dbg!(cpu);
}

// #[rstest]
fn test_disassemble(mut cpu: dh_cpu::CPU, mut bus: dh_bus::BUS) {
    const START: u16 = 0x0000;
    const STOP: u16 = 0x000f;

    for i in START..STOP {
        bus.write(i, 0xa9); // 169 LDA
        cpu.clock(&mut bus);
        cpu.set_pc(cpu.pc() + 1);

        cpu.clock(&mut bus);

        // dbg!(cpu.opcode());
    }

    let dis_asm = CPU::disassemble(&mut bus, START, STOP);
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
fn test_mini_program(mut cpu: dh_cpu::CPU, mut bus: dh_bus::BUS) {
    const START: u16 = 0xC000;
    const STOP: u16 = 0xC00E;

    // "preloaded" data in ram
    bus.write(0x00, 0xA);
    bus.write(0x01, 0x14);
    bus.write(0x02, 0x1E);
    bus.write(0x03, 0x28);

    let ttape = bs![
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
    bus.load_instruction_mem(ttape);
    for _ in 0..8 {
        cpu.clock(&mut bus);
    }

    let disasm: std::collections::HashMap<u16, String> =
        CPU::disassemble(&mut bus, START, STOP);

    dbg!(disasm);

    dbg!(dh_bus::get_addr_access_hit_count());
}

// #[rstest]
fn test_gex_fmt() {
    let string_rep: String = format!("#${:x} {{imm}}", 100u8 as u32);
    dbg!(string_rep);
}
