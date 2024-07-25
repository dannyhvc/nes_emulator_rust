use crate::bs;
use crate::components::dh_bus;
use crate::components::dh_cpu::CPU;
use crate::components::{dh_bus::bus::BUS, KB};

use iced::Application;
use iced::{
    widget::{row, Container, Text},
    Element, Settings,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebuggeeMessage {
    Start,
    End,
}

#[derive(Debug, Clone)]
pub struct Debuggees {
    cpu: CPU,
    bus: BUS,
}

impl Application for Debuggees {
    type Message = DebuggeeMessage;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut cpu = CPU::new();
        let mut bus = BUS::new();
        CPU::reset(&mut cpu, &bus);
        mini_program(&mut cpu, &mut bus);

        (Self { cpu, bus }, iced::Command::none())
    }

    fn title(&self) -> String {
        "NES Debugging".into()
    }

    fn update(
        &mut self,
        message: Self::Message,
    ) -> iced::Command<Self::Message> {
        match message {
            DebuggeeMessage::Start => println!("Session Started"),
            DebuggeeMessage::End => println!("Session Ended"),
        }

        iced::Command::none()
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

        let r = dh_bus::ram_stats::read_access_hits();
        let w = dh_bus::ram_stats::write_access_hits();

        // iced_table::table();

        Container::new(iced::widget::Scrollable::new(row![cpu_col,])).into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }

    fn scale_factor(&self) -> f64 {
        1.0
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

    let _disasm: std::collections::HashMap<u16, String> =
        CPU::disassemble(&mut bus, START, STOP);

    let r = dh_bus::ram_stats::read_access_hits();
    let w = dh_bus::ram_stats::write_access_hits();
}

pub fn run() {
    let settings = Settings::<()> {
        window: iced::window::Settings {
            size: iced::Size::new(800.0, 800.0),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Run the application with custom settings
    Debuggees::run(settings).unwrap();
}
