#![allow(non_snake_case)]
mod components;
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

#[cfg(feature = "debug")]
mod debug {
    use dioxus::prelude::*;
    use dioxus_logger::tracing::Level;

    #[component]
    fn App() -> Element {
        // Build cool things ✌️

        rsx! {
            link { rel: "stylesheet", href: "main.css" }
            div { id: "links",
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            }
        }
    }

    pub fn run() {
        dioxus_logger::init(Level::INFO).expect("failed to init logger");
        dioxus::launch(App);
    }
}

fn main() {
    #[cfg(feature = "debug")]
    debug::run();

    #[cfg(not(feature = "debug"))]
    println!("starting nes-emulator-rs");
}
