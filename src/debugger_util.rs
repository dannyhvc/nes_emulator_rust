use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::widgets::{Block, Borders, Paragraph};
use std::{
    error::Error,
    io::{stdout, Stdout},
    ops::ControlFlow,
    time::Duration,
};

use ratatui::{self, prelude::*};

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

fn ui(f: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(f.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(outer_layout[1]);

    // testing out the supper simple layout system
    f.render_widget(
        Paragraph::new("outter 0").block(Block::new().borders(Borders::ALL)),
        outer_layout[0],
    );
    f.render_widget(
        Paragraph::new("inner 0").block(Block::new().borders(Borders::ALL)),
        inner_layout[0],
    );
    f.render_widget(
        Paragraph::new("inner 1").block(Block::new().borders(Borders::ALL)),
        inner_layout[1],
    );
}

pub fn run(terminal: &mut Terminal, ui: fn(&mut Frame)) -> Result<()> {
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
