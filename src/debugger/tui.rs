use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::{Block, Borders, Paragraph};
use std::{ops::ControlFlow, time::Duration};

use ratatui::{self, prelude::*};

use super::util;

pub fn handle_events() -> util::Result<ControlFlow<()>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(ControlFlow::Break(()));
            }
        }
    }
    Ok(ControlFlow::Continue(()))
}

pub fn ui(f: &mut Frame) {
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
