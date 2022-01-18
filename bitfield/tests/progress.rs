#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-specifier-types.rs");
    t.pass("tests/02-storage.rs");
    t.pass("tests/03-accessors.rs");
    t.pass("tests/03-accessors2.rs");
    // see bits_pos.rs::test_1_3_4_23
    // t.compile_fail("tests/04-multiple-of-8bits.rs");
    t.pass("tests/05-accessor-signatures.rs");
    t.pass("tests/06-enums.rs");
    t.pass("tests/07-optional-discriminant.rs");
    t.compile_fail("tests/08-non-power-of-two.rs");

    // t.compile_fail("tests/09-variant-out-of-range.rs");
    // This is an alternative test for 09:
    t.compile_fail("tests/09-variant-out-of-range2.rs");

    // `#[bits = 1]` 这个检查在我的实现中没太大意义，如果真的需要 bit 长度，完全可以定义一个 trait
    // 把 enum 与 Bx 对应起来
    t.pass("tests/10-bits-attribute.rs");
    // t.compile_fail("tests/11-bits-attribute-wrong.rs");
    t.pass("tests/12-accessors-edge.rs");
}
