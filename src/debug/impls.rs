pub mod views;

use iced::keyboard::key;
use iced::widget::row;
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
            util: crate::debug::Utilities {
                table_header_id: iced::widget::scrollable::Id::unique(),
                table_body_id: iced::widget::scrollable::Id::unique(),
                table_footer_id: iced::widget::scrollable::Id::unique(),
            },
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
            DebuggeeMessage::SyncHeader(offset) => {
                return iced::Command::batch(vec![
                    iced::widget::scrollable::scroll_to(
                        iced::widget::scrollable::Id::unique(),
                        offset,
                    ),
                    iced::widget::scrollable::scroll_to(
                        iced::widget::scrollable::Id::unique(),
                        offset,
                    ),
                ]);
            }
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
        let res = iced::widget::responsive(|_s: iced::Size| {
            let cpu_debug = views::debug_cpu_view(self);
            let read_col = views::ram_read_hit_view();
            // write col init
            let write_col = views::ram_write_hit_view();

            row! {
                cpu_debug,
                read_col,
                write_col,
            }
            .into()
        });

        // collecting kv's into vec and sorting by key (address) s.t. it is a sorted
        // hashmap. Doing this for both Read and Write on ram so that we can display
        // them in a column and organize the them by access category

        // read col init
        // Container::new(row![cpu_col, bus_col, read_col, write_col, table]).into()
        res.into()
    }
}
