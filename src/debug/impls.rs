use iced::keyboard::key;
use iced::widget::{row, Container, Text};
use iced::Element;
use iced::{Application, Command};

use super::{mini_program, DebuggeeMessage, Debuggees};
use crate::components::dh_bus::bus::BUS;
use crate::components::dh_cpu::cpu::CPU;

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
            DebuggeeMessage::KeyPressed(key) => {
                if let iced::keyboard::Key::Character(c) = key {
                    if c == "q" {
                        return iced::window::close::<DebuggeeMessage>(
                            iced::window::Id::MAIN,
                        );
                    }

                    if c == "r" {
                        self.cpu.reset(&self.bus);
                    }
                } else if let iced::keyboard::Key::Named(n) = key {
                    if n == iced::keyboard::key::Named::Escape {
                        return iced::window::close::<DebuggeeMessage>(
                            iced::window::Id::MAIN,
                        );
                    }

                    if n == iced::keyboard::key::Named::Space {
                        if !self.cpu.complete() {
                            self.cpu.clock(&mut self.bus);
                        }
                    }
                } else {
                    unimplemented!()
                }
            }
        }

        iced::Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::keyboard::on_key_press(|key, _modifier| match key {
            key::Key::Named(..) => Some(DebuggeeMessage::KeyPressed(key)),
            key::Key::Character(..) => Some(DebuggeeMessage::KeyPressed(key)),
            key::Key::Unidentified => None,
        })
    }

    fn view(&self) -> Element<'_, Self::Message> {
        debug_cpu_view(&self).into()
    }
}

fn debug_cpu_view(app: &Debuggees) -> Container<DebuggeeMessage> {
    let cpu_col = iced::widget::Column::<DebuggeeMessage>::new()
        .push(Text::new("CPU DATA").size(30))
        .push(row!(Text::new(format!("{}", app.cpu)).size(20)))
        .padding(100);

    let mut bus_col = iced::widget::Column::<DebuggeeMessage>::new()
        .push(Text::new("BUS DATA").size(30))
        .push(row![Text::new(format!(
            "RAM: {}KB",
            app.bus.ram().len() / crate::components::KB(1)
        )),])
        .padding(100);

    let r = crate::components::dh_bus::ram_stats::read_access_hits();
    let w = crate::components::dh_bus::ram_stats::write_access_hits();

    // iced_table::table();

    Container::new(iced::widget::Scrollable::new(row![cpu_col,])).into()
}
