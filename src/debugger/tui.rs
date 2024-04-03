use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::{Block, Borders, Paragraph};
use std::{ops::ControlFlow, time::Duration};

use ratatui::{self, prelude::*};

use super::util;

pub fn handle_events() -> util::Result<ControlFlow<()>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            // println!("{key:?}");
            if key.code == KeyCode::Char('q') {
                return Ok(ControlFlow::Break(()));
            }
        }
    }
    Ok(ControlFlow::Continue(()))
}

pub fn ui(f: &mut Frame) {
    let col_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50); 2])
        .split(f.size());

    let row_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(20, 3); 15])
        .split(col_layout[0]);

    // let inner_layout = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints(vec![
    //         Constraint::Percentage(25),
    //         Constraint::Percentage(75),
    //     ])
    //     .split(outer_layout[1]);

    // // testing out the supper simple layout system
    for &layo in row_layout.iter() {
        f.render_widget(
            Paragraph::new("outter 0")
                .block(Block::new().borders(Borders::ALL)),
            layo,
        );
    }
}
