fn main() {
    let val = seq::seq!(N in -1..1 {
       1 #( * N )*
    });
    assert_eq!(val, 0);
}
