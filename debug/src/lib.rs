pub mod app;
pub mod data_preporation;
pub mod styles;
pub mod traits;
pub mod types;
pub mod views;
pub mod _experiment; // not useful for end product im just testing stuff
pub mod util;
pub mod widgets;

use eyre::{Context, Result as ErrorOr};

pub fn run() -> ErrorOr<()> {
    iced::application(app::title, app::update, app::view)
        .settings(iced::Settings {
            id: Some("main".into()),
            ..iced::Settings::default()
        })
        .theme(|_| iced::Theme::Dark)
        .subscription(app::subscription)
        .exit_on_close_request(true)
        .run()
        .context("iced app encountered a critical failure")
}
