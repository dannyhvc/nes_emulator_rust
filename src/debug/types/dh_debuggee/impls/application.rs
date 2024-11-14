use iced::application::{Title, Update, View};
// iced imports
use iced::keyboard::key;
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
// use crate::debug::widgets::movable_nodes;
// use crate::debug::widgets::movable_nodes::MovableNodes;
// use crate::debug::widgets::ram_widgets::read_hits::ram_read_hit_view;
// use crate::debug::widgets::ram_widgets::write_hits::ram_write_hit_view;

impl Debuggees {
    pub fn new() -> Self {
        let mut this = Self {
            cpu: CPU::new(),
            bus: BUS::new(),
            util: Utilities {},
        };
        CPU::reset(&mut this.cpu, &this.bus);
        mini_program(&mut this);

        this
    }
}

pub struct DebuggerApp;
impl Title<Debuggees> for DebuggerApp {
    fn title(&self, _state: &Debuggees) -> String {
        "NES Debugger".into()
    }
}

impl Update<Debuggees, DebuggeeMessage> for DebuggerApp {
    fn update(
        &self,
        state: &mut Debuggees,
        message: DebuggeeMessage,
    ) -> impl Into<iced::Task<DebuggeeMessage>> {
        match message {
            DebuggeeMessage::SyncHeader(offset) => {
                return iced::Task::batch(vec![
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
                            iced::window::Id::unique(),
                        );
                    }

                    if c == "r" {
                        state.cpu.reset(&state.bus);
                    }
                } else if let iced::keyboard::Key::Named(n) = key {
                    if n == iced::keyboard::key::Named::Escape {
                        return iced::window::close::<DebuggeeMessage>(
                            iced::window::Id::unique(),
                        );
                    }

                    if n == iced::keyboard::key::Named::Space {
                        if !state.cpu.complete() {
                            state.cpu.clock(&mut state.bus);
                        }
                    }
                } else {
                    unimplemented!()
                }
            }
        }
        iced::Task::none()
    }
}

impl<'a> View<'a, Debuggees, DebuggeeMessage, iced::Theme, iced::Renderer>
    for DebuggerApp
{
    fn view(
        &self,
        _state: &'a Debuggees,
    ) -> impl Into<Element<'a, DebuggeeMessage, iced::Theme, iced::Renderer>>
    {
        iced::widget::Container::new(iced::widget::text("beans"))
    }
}

// fn subscription(&self) -> iced::Subscription<Self::Message> {
//     iced::keyboard::on_key_press(|key, _modifier| match key {
//         key::Key::Named(..) => Some(DebuggeeMessage::KeyPressed(key)),
//         key::Key::Character(..) => Some(DebuggeeMessage::KeyPressed(key)),
//         key::Key::Unidentified => None,
//     })
// }

// fn view(&self) -> Element<'_, Self::Message> {
//     let mut mn = movable_nodes::MovableNodes::new();
//     mn.nodes
//         .push(MovableNodes::new_node_at(iced::Point::new(100.0, 100.0)));
//     mn.into()
// }
