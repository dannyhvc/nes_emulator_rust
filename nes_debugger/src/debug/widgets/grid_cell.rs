use crate::debug::{
    styles::grid_cell::BorderedContainerStyle,
    types::dh_debuggee_message::DebuggeeMessage,
};

pub(crate) fn bordered_container<'a, T: 'a>(
    content: T,
) -> iced::widget::Container<'a, DebuggeeMessage>
where
    T: Into<iced::Element<'a, DebuggeeMessage>>,
{
    iced::widget::Container::new(content).style(BorderedContainerStyle)
}
