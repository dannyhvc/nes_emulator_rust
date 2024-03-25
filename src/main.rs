mod components;

#[cfg(test)]
mod tests;

mod macros;

mod debugger_util;

use debugger_util as nesd;

fn main() -> nesd::Result<()> {
    nesd::start()
}
