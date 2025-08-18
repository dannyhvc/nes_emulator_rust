#![allow(non_snake_case, unused)]

#[cfg(feature = "debug")]
use debug;

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! bs {
    ($($x:expr),*) => (
        // Kinda wanna switch this to a runtime sized array instead of a
        // conversion like this
        vec![$($x),*].into_boxed_slice()
    );
}

use eyre::Result as ErrorOr;

fn main() -> ErrorOr<()> {
    #[cfg(feature = "debug")]
    {
        use std::env::set_var;
        pretty_env_logger::init();

        set_var("RUST_LOG", "info");
        debug::run()?;
    }

    #[cfg(not(feature = "debug"))]
    println!("starting nes-emulator-rs");
    Ok(())
}
