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
        let bus = BUS::new();
        CPU::reset(&mut cpu, &bus);
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

        let bus_col = iced::widget::Column::<Self::Message>::new()
            .push(Text::new("BUS DATA").size(30))
            .push(row!(Text::new(format!(
                "CPU-RAM: {}KB",
                self.bus.cpu_ram.len() / KB(1)
            ))))
            .padding(100);

        let scroll_area = iced::widget::Scrollable::new(bus_col);

        Container::new(row![cpu_col, scroll_area]).into()
    }
}

pub fn run() {
    Debuggees::run(Settings::default()).unwrap();
}
