// iced imports
use iced::keyboard::key;
use iced::widget::row;
use iced::Application;
use iced::Element;

// nes components
use crate::components::dh_bus::bus::BUS;
use crate::components::dh_cpu::cpu::CPU;

// debug imports
use crate::debug::mini_program;
use crate::debug::types::dh_debuggee::debuggee::Debuggees;
use crate::debug::types::dh_debuggee_message::DebuggeeMessage;
use crate::debug::types::utilities::Utilities;
// use crate::debug::widgets::cpu_monitor_view::cpu_view;
use crate::debug::widgets::movable_nodes;
use crate::debug::widgets::movable_nodes::MovableNodes;
// use crate::debug::widgets::ram_widgets::read_hits::ram_read_hit_view;
// use crate::debug::widgets::ram_widgets::write_hits::ram_write_hit_view;

impl Application for Debuggees {
    type Message = DebuggeeMessage;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut this = Self {
            cpu: CPU::new(),
            bus: BUS::new(),
            util: Utilities {
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
        let mut mn = movable_nodes::MovableNodes::new();
        mn.nodes
            .push(MovableNodes::new_node_at(iced::Point::new(100.0, 100.0)));
        mn.into()
    }
}
