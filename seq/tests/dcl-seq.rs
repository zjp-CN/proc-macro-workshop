macro_rules! seq {
    ($($lit:literal),*) => {
        $(
            compile_error!(concat!("error number ", stringify!($lit)));
        )*

    }
}

seq! { 0, 1, 2 }
fn main() {}
