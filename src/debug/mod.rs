use anyhow::Context;
use app::*;
use iced::Settings;
use types::DebuggerApp;

pub mod app;
pub mod styles;
pub mod types;
pub mod views;

pub fn run() -> anyhow::Result<()> {
    iced::application(DebuggerApp, DebuggerApp, DebuggerApp)
        .settings(Settings::default())
        .subscription(subscription)
        .exit_on_close_request(true)
        .run()
        .context("iced app encountered a critical failure")
}
