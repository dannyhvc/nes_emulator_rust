mod components;

#[cfg(test)]
mod tests;

mod macros;

mod debugger_util;

use debugger_util as nesd;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

fn ui(f: &mut nesd::Frame) {
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

fn main() -> nesd::Result<()> {
    let mut terminal: nesd::Terminal = nesd::setup_terminal()?;

    let result = nesd::run(&mut terminal, ui);

    if let Err(err) = result {
        eprintln!("{err:?}");
    }

    nesd::restore_terminal(terminal)
}
