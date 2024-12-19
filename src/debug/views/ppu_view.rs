use super::menu_drop_down;
use crate::debug::types::{DebuggerMsg, DebuggerState};
use iced::widget::column as col;
use iced::widget::*;
use iced::*;

pub fn ppu_base<'a>(_state: &DebuggerState) -> Element<'a, DebuggerMsg> {
    let mb = menu_drop_down();
    let main = col![mb];
    main.push(Text::new("text")).width(Length::Fill).into()
}
