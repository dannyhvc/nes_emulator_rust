use iced::{
    widget::{Button, Text},
    Element,
};

use super::types::DebuggerMsg;

pub fn base<'a>() -> impl Into<Element<'a, DebuggerMsg>> {
    Button::new(Text::new("Dan"))
}
