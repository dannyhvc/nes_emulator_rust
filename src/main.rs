mod components;

#[cfg(test)]
mod tests;

mod macros;

mod debugger;
use debugger::util::{start, Result};

fn main() -> Result<()> {
    let mut args = std::env::args();

    if args.any(|a| a == "dbg") {
        start()?;
    }

    Ok(())
}
