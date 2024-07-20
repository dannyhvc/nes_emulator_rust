use crate::bs;
use crate::components::dh_bus;
use crate::components::dh_cpu::CPU;
use crate::components::{dh_bus::BUS, KB};

use iced::{
    widget::{row, Container, Text},
    Element, Sandbox, Settings,
};

#[derive(Debug, Clone, Copy)]
pub enum DebuggeeMessage {
    Start,
}

#[derive(Debug, Clone)]
pub struct Debuggees {
    cpu: CPU,
    bus: BUS,
}

impl Sandbox for Debuggees {
    type Message = DebuggeeMessage;

    fn new() -> Self {
        let mut cpu = CPU::new();
        let mut bus = BUS::new();
        CPU::reset(&mut cpu, &bus);
        mini_program(&mut cpu, &mut bus);

        Self { cpu, bus }
    }

    fn title(&self) -> String {
        "NES Debugging".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            DebuggeeMessage::Start => println!("Session Started"),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let cpu_col = iced::widget::Column::<Self::Message>::new()
            .push(Text::new("CPU DATA").size(30))
            .push(row!(Text::new(format!("{}", self.cpu)).size(20)))
            .padding(100);

        let mut bus_col = iced::widget::Column::<Self::Message>::new()
            .push(Text::new("BUS DATA").size(30))
            .push(row![Text::new(format!(
                "RAM: {}KB",
                self.bus.ram().len() / KB(1)
            )),])
            .padding(100);

        let mut heat_map: Vec<_> =
            dh_bus::get_addr_access_hit_count().into_iter().collect();
        heat_map.sort_by_key(|&(key, _)| key);

        bus_col = bus_col.push(row![
            Container::new(Text::new(format!("ADDR:"))).padding(5),
            Container::new(Text::new(format!("DATA:"))).padding(5)
        ]);
        for (k, v) in heat_map.iter() {
            bus_col = bus_col.push(row![
                Container::new(Text::new(format!("{:4x}", k))).padding(5),
                Container::new(Text::new(format!("{:4x}", v))).padding(5),
            ])
        }

        let scroll_area = iced::widget::Scrollable::new(bus_col);

        Container::new(row![cpu_col, scroll_area]).into()
    }
}

fn mini_program(cpu: &mut CPU, mut bus: &mut BUS) {
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

pub fn run() {
    Debuggees::run(Settings::default()).unwrap();
}
