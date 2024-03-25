use crossterm::{
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::{
    error::Error,
    io::{stdout, Stdout},
};

use ratatui::{self, prelude::*};

use super::tui::{handle_events, ui};

// These type aliases are used to make the code more readable by reducing repetition of the generic
// types. They are not necessary for the functionality of the code.
pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn setup_terminal() -> Result<Terminal> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(mut terminal: Terminal) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn run(terminal: &mut Terminal, ui: fn(&mut Frame)) -> Result<()> {
    loop {
        terminal.draw(ui)?;
        if handle_events()?.is_break() {
            return Ok(());
        }
    }
}

pub fn start() -> Result<()> {
    let mut terminal: Terminal = setup_terminal()?;

    let result = run(&mut terminal, ui);

    if let Err(err) = result {
        eprintln!("{err:?}");
    }

    restore_terminal(terminal)
}
