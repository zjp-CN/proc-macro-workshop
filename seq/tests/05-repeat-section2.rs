seq::seq!(N in 2..5 {
    const fn f() -> usize {
        1 #( +N )* #( *N )*
    }
});

fn main() {
    assert_eq!(f(), 1 + 2 + 3 + 4 * 2 * 3 * 4);
}
