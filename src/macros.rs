#[macro_export]
macro_rules! bs {
    ($($x:expr),*) => (
        // Kinda wanna switch this to a runtime sized array instead of a
        // conversion like this
        vec![$($x),*].into_boxed_slice()
    );
}

#[macro_export]
macro_rules! property {
    ($name:ident : $ty:ty) => {
        pub fn $name(&self) -> $ty {
            self.$name
        }

        paste! {
            pub fn [<set_ $name>](&mut self, $name: $ty) {
                self.$name = $name;
            }
        }
    };
}
