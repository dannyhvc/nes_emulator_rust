mod components;

#[cfg(test)]
mod tests;

mod macros;

use nes_debug as nesd;

fn ui(frame: &mut nesd::Frame) {}

fn main() -> nesd::Result<()> {
    let mut terminal: nesd::Terminal = nesd::setup_terminal()?;

    let result = nesd::run(&mut terminal, ui);

    if let Err(err) = result {
        eprintln!("{err:?}");
    }

    Ok(())
}
