mod impls;

use crate::bs;
use iced::{Application, Settings};

#[derive(Debug, Clone, PartialEq)]
pub enum DebuggeeMessage {
    Start,
    KeyPressed(iced::keyboard::Key),
    End,
}

#[derive(Debug, Clone)]
pub struct Debuggees {
    cpu: crate::components::dh_cpu::cpu::CPU,
    bus: crate::components::dh_bus::bus::BUS,
}

fn mini_program(
    cpu: &mut crate::components::dh_cpu::cpu::CPU,
    mut bus: &mut crate::components::dh_bus::bus::BUS,
) {
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

    let _disasm: std::collections::HashMap<u16, String> =
        crate::components::dh_cpu::cpu::CPU::disassemble(&mut bus, START, STOP);

    let _r = crate::components::dh_bus::ram_stats::read_access_hits();
    let _w = crate::components::dh_bus::ram_stats::write_access_hits();
}

pub fn run() {
    let settings = Settings::<()> {
        window: iced::window::Settings {
            size: iced::Size::new(800.0, 800.0),
            resizable: true,
            exit_on_close_request: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Run the application with custom settings
    Debuggees::run(settings).unwrap();
}
