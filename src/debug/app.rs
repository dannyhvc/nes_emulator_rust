//logging import
use log::{debug, info};

// iced imports
use iced::application::{Title, Update, View};
use iced::keyboard::key;
use iced::{Element, Subscription};

// debug imports
use crate::debug::types::DebuggerApp;
use crate::debug::types::DebuggerMsg;
use crate::debug::types::DebuggerState;

impl Title<DebuggerState> for DebuggerApp {
    fn title(&self, _state: &DebuggerState) -> String {
        "NES Debugger".into()
    }
}

impl Update<DebuggerState, DebuggerMsg> for DebuggerApp {
    fn update(
        &self,
        state: &mut DebuggerState,
        message: DebuggerMsg,
    ) -> impl Into<iced::Task<DebuggerMsg>> {
        match message {
            DebuggerMsg::SyncHeader(offset) => {
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
            DebuggerMsg::Start => debug!("Session Started"),
            DebuggerMsg::End => debug!("Session Ended"),
            DebuggerMsg::KeyPressed(key) => match key {
                iced::keyboard::Key::Character(c) => {
                    if c == "q" {
                        info!("exiting...");
                        // return iced::window::close::<DebuggerMsg>(
                        //     iced::window::Id::unique(),
                        // );
                        std::process::exit(0);
                    }

                    if c == "r" {
                        info!("reseting cpu state...");
                        state.cpu.reset(&state.bus);
                    }
                }
                iced::keyboard::Key::Named(n) => {
                    if n == iced::keyboard::key::Named::Escape {
                        info!("exiting...");
                        return iced::window::close::<DebuggerMsg>(
                            iced::window::Id::unique(),
                        );
                    }

                    if n == iced::keyboard::key::Named::Space {
                        if !state.cpu.complete() {
                            state.cpu.clock(&mut state.bus);
                        }
                    }
                }
                _ => unimplemented!(),
            },
            _ => {}
        }
        iced::Task::none()
    }
}

impl<'a> View<'a, DebuggerState, DebuggerMsg, iced::Theme, iced::Renderer>
    for DebuggerApp
{
    fn view(
        &self,
        _state: &'a DebuggerState,
    ) -> impl Into<Element<'a, DebuggerMsg, iced::Theme, iced::Renderer>> {
        iced::widget::Container::new(iced::widget::text("beans"))
    }
}

pub fn subscription(state: &DebuggerState) -> Subscription<DebuggerMsg> {
    iced::keyboard::on_key_press(|key, _modifier| match key {
        key::Key::Named(_) => Some(DebuggerMsg::KeyPressed(key)),
        key::Key::Character(_) => Some(DebuggerMsg::KeyPressed(key)),
        key::Key::Unidentified => None,
    })
}
