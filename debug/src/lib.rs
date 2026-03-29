pub mod app;
pub mod data_preporation;
pub mod styles;
pub mod traits;
pub mod types;
pub mod views;
pub mod _experiment; // not useful for end product im just testing stuff
pub mod util;
pub mod widgets;
pub mod debugger;

use eyre::{Context, Result as ErrorOr};

pub fn run() -> iced::Result {
    debugger::run()
}
