use std::{
    error::Error,
    io::{stdout, Stdout},
    ops::ControlFlow,
    time::Duration,
};

pub use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

pub use ratatui::{self, prelude::*};

// These type aliases are used to make the code more readable by reducing repetition of the generic
// types. They are not necessary for the functionality of the code.
pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn setup_terminal() -> Result<Terminal> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_terminal(mut terminal: Terminal) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

pub fn handle_events() -> Result<ControlFlow<()>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(ControlFlow::Break(()));
            }
        }
    }
    Ok(ControlFlow::Continue(()))
}

pub fn run(terminal: &mut Terminal, ui: fn(&mut Frame)) -> Result<()> {
    loop {
        terminal.draw(ui)?;
        if handle_events()?.is_break() {
            return Ok(());
        }
    }
}
