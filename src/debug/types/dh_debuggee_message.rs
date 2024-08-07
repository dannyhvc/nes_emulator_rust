#[derive(Debug, Clone, PartialEq)]
pub enum DebuggeeMessage {
    Start,
    KeyPressed(iced::keyboard::Key),
    SyncHeader(iced::widget::scrollable::AbsoluteOffset),
    End,
}
