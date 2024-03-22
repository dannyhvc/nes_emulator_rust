mod components;

#[cfg(test)]
mod tests;

mod macros;

use nes_debug::{self as nesd, ratatui::widgets as w};
use nesd::Stylize;

fn ui(frame: &mut nesd::Frame) {
    let (area, layout) = nesd::calc_layout(frame.size());

    // made a random paragraph
    let p = w::Paragraph::new("Dan made a paragraph")
        .alignment(nesd::Alignment::Center);

    // made a random table
    let mut table_state = w::TableState::default();
    let rows = [
        w::Row::new(vec!["Row11", "Row12", "Row13"]),
        w::Row::new(vec!["Row21", "Row22", "Row23"]),
        w::Row::new(vec!["Row31", "Row32", "Row33"]),
    ];
    let widths = [
        nesd::Constraint::Length(5),
        nesd::Constraint::Length(5),
        nesd::Constraint::Length(10),
    ];
    let table = w::Table::new(rows, widths)
        .block(w::Block::default().title("Table"))
        .highlight_style(
            nesd::Style::new().add_modifier(nesd::Modifier::REVERSED),
        )
        .highlight_symbol(">>");
    frame.render_stateful_widget(table, area, &mut table_state);
}

fn main() -> nesd::Result<()> {
    let mut terminal: nesd::Terminal = nesd::setup_terminal()?;

    let result = nesd::run(&mut terminal, ui);

    if let Err(err) = result {
        eprintln!("{err:?}");
    }

    Ok(())
}
