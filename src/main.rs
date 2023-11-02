mod components;
mod tests;

#[macro_export]
macro_rules! bs {
    ($($x:expr),*) => (
        vec![$($x),*].into_boxed_slice()
    );
}

fn main() {}
