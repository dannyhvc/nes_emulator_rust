use anyhow::Context;
use app::*;
use iced::Settings;
use types::DebuggerApp;

pub mod app;
pub mod data_preporation;
pub mod styles;
pub mod traits;
pub mod types;
pub mod views;

pub fn run() -> anyhow::Result<()> {
    iced::application(DebuggerApp, DebuggerApp, DebuggerApp)
        .settings(Settings {
            id: Some("main".into()),
            ..Settings::default()
        })
        .theme(|_| iced::Theme::Nord)
        .subscription(subscription)
        .exit_on_close_request(true)
        .run()
        .context("iced app encountered a critical failure")
}
