use super::menu_drop_down;
use crate::debug::types::{DebuggerMsg, DebuggerState};
use iced::widget::column as col;
use iced::widget::*;
use iced::*;

pub fn cpu_base<'a>(state: &DebuggerState) -> Element<'a, DebuggerMsg> {
    let mb = menu_drop_down();
    let main = col![mb];

    // let register_details = &state.cpu;
    // let register_details = [
    //     Text::new(),
    //     Text::new(),
    //     Text::new(),
    //     Text::new(),
    //     Text::new(),
    // ];
    // TODO: make a format view component for cpu

    main.push(Text::new(format!("{:?}", state.cpu)))
        .width(Length::Fill)
        .into()
}
