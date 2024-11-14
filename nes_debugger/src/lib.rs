use debug::types::dh_debuggee::debuggee;
pub mod debug;

pub fn run() {
    let settings = iced::Settings::<()> {
        window: iced::window::Settings {
            size: iced::Size::new(800.0, 800.0),
            resizable: true,
            exit_on_close_request: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // Run the application with custom settings
    debuggee::Debuggees::run(settings).unwrap();
}
