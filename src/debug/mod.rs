mod impls;

use crate::bs;
use iced::{Application, Settings};

#[derive(Debug, Clone, PartialEq)]
pub enum DebuggeeMessage {
    Start,
    KeyPressed(iced::keyboard::Key),
    SyncHeader(iced::widget::scrollable::AbsoluteOffset),
    End,
}

#[derive(Debug, Clone)]
pub struct Utilities {
    table_header_id: iced::widget::scrollable::Id,
    table_body_id: iced::widget::scrollable::Id,
    table_footer_id: iced::widget::scrollable::Id,
}

#[derive(Debug, Clone)]
pub struct Debuggees {
    cpu: crate::components::dh_cpu::cpu::CPU,
    bus: crate::components::dh_bus::bus::BUS,
    util: Utilities,
}

fn mini_program(Debuggees { cpu, bus, .. }: &mut Debuggees) {
    const START: u16 = 0x0000;
    const STOP: u16 = 0xFFFF;

    /*
     * A2 0A       LDX #10
     * 8E 00 00    STX $0000
     * A2 03       LDX #3
     * 8E 01 00    STX $0001
     * AC 00 00    LDY $0000
     * A9 00       LDA #0
     * 18          CLC
     * 6D 01 00    ADC $0001
     * 88          DEY
     * D0 FA       BNE loop -- FA is the relative offset for the branch
     * 8D 02 00    STA $0002
     * EA          NOP
     * EA          NOP
     * EA          NOP
     */
    let ttape = bs![
        bs![0x8000, 0xA2, 0x0A],
        bs![0x8002, 0x8E, 0x00, 0x00],
        bs![0x8005, 0xA2, 0x03],
        bs![0x8007, 0x8E, 0x01, 0x00],
        bs![0x800A, 0xAC, 0x00, 0x00],
        bs![0x800D, 0xA9, 0x00],
        bs![0x800F, 0x18],
        bs![0x8010, 0x6D, 0x01, 0x00],
        bs![0x8040, 0x88],
        bs![0x8050, 0xD0, 0xFA],
        bs![0x8070, 0x8D, 0x02, 0x00],
        bs![0x80A0, 0xEA],
        bs![0x80B0, 0xEA],
        bs![0x80C0, 0xEA]
    ];

    // Set reset vector (where the program will start exectuing from)
    bus.write(crate::components::RESET_VECTOR_LOW_BYTE, 0x00);
    bus.write(crate::components::RESET_VECTOR_HIGH_BYTE, 0x80);

    // is there a better way to do this?
    // NOTE this will add count of WRITE for all program instruction addresses.
    bus.load_instruction_mem(ttape.clone());

    // NOTE this will add count of READ for all locations between START and STOP
    let disasm: std::collections::HashMap<u16, String> =
        crate::components::dh_cpu::cpu::CPU::disassemble(bus, START, STOP);
    let disasm: Vec<_> = disasm
        .iter()
        .filter(|&(k, _v)| {
            let mut retval = false;
            ttape.iter().for_each(|ins| {
                if *k == ins[0] {
                    retval = true;
                }
            });

            retval
        })
        .collect();

    dbg!(disasm);
    cpu.reset(&bus);
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
