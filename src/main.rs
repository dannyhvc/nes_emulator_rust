<<<<<<< HEAD
#![allow(non_snake_case)]
mod components;

#[cfg(feature = "debug")]
pub mod debug;

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

fn main() {
    #[cfg(feature = "debug")]
    debug::run();

    #[cfg(not(feature = "debug"))]
    println!("starting nes-emulator-rs");
}
=======
#![allow(non_snake_case)]

mod components;

#[cfg(feature = "debug")]
pub mod debug;

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

fn main() -> anyhow::Result<()> {
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
>>>>>>> debugger
