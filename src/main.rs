mod components;

#[cfg(test)]
mod tests;

mod macros;

mod debugger;
use debugger::util::{start, Result};

fn main() -> Result<()> {
    start()
}
