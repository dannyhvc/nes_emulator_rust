use iced::keyboard::key;
use iced::widget::{column, row, Container, Text};
use iced::Application;
use iced::Element;

use super::{mini_program, DebuggeeMessage, Debuggees};
use crate::components::dh_bus::bus::BUS;
use crate::components::dh_cpu::cpu::CPU;

impl Application for Debuggees {
    type Message = DebuggeeMessage;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut this = Self {
            cpu: CPU::new(),
            bus: BUS::new(),
        };
        CPU::reset(&mut this.cpu, &this.bus);
        mini_program(&mut this);

        (this, iced::Command::none())
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

    let bus_col = iced::widget::Column::<DebuggeeMessage>::new()
        .push(Text::new("BUS DATA").size(30))
        .push(row![Text::new(format!(
            "RAM: {}KB",
            app.bus.ram().len() / crate::components::KB(1)
        )),])
        .padding(100);

    // collecting kv's into vec and sorting by key (address) s.t. it is a sorted
    // hashmap. Doing this for both Read and Write on ram so that we can display
    // them in a column and organize the them by access category

    // read col init
    let read_col = {
        let mut read_col: iced::widget::Column<DebuggeeMessage> =
            column!(Text::new("Read")).padding(10);
        let mut r: Vec<_> =
            crate::components::dh_bus::ram_stats::read_access_hits()
                .into_iter()
                .filter(|&(_k, v)| v > 1)
                .collect();
        r.sort_by_key(|&(key, _)| key);

        for &(k_read, v_read) in r.iter() {
            read_col = read_col
                .push(column!(Text::new(format!("@{:x} #{}", k_read, v_read))))
                .padding(5);
        }
        read_col
    };

    // write col init
    let write_col = {
        let mut write_col: iced::widget::Column<DebuggeeMessage> =
            column!(Text::new("Write")).padding(10);

        let mut w: Vec<_> =
            crate::components::dh_bus::ram_stats::write_access_hits()
                .into_iter()
                .filter(|&(_k, v)| v > 1)
                .collect();
        w.sort_by_key(|&(key, _)| key);

        for &(k_write, v_write) in w.iter() {
            write_col = write_col
                .push(column!(Text::new(format!(
                    "@{:x} #{}",
                    k_write, v_write
                ))))
                .padding(5);
        }
        write_col
    };

    // iced_table::table();

    Container::new(iced::widget::Scrollable::new(row![
        cpu_col, bus_col, read_col, write_col
    ]))
    .into()
}
