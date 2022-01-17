use bitfield::BitsPos;

#[test]
fn test_56_18_26_14_8() {
    // [56, 18, 26, 14, 8]
    type Bit56 = BitsPos<56, 0>;
    type Bit18 = BitsPos<18, 56>;
    type Bit26 = BitsPos<26, 74>;
    type Bit14 = BitsPos<14, 100>;
    type Bit08 = BitsPos<8, 114>;

    assert_eq!(Bit56::RANGE, 0..=6);
    assert_eq!(Bit18::RANGE, 7..=9);
    assert_eq!(Bit26::RANGE, 9..=12);
    assert_eq!(Bit14::RANGE, 12..=14);
    assert_eq!(Bit08::RANGE, 14..=15);

    assert_eq!(Bit56::OFFSET, 0);
    assert_eq!(Bit18::OFFSET, 0);
    assert_eq!(Bit26::OFFSET, 2);
    assert_eq!(Bit14::OFFSET, 4);
    assert_eq!(Bit08::OFFSET, 2);

    let mut arr = [0; (56 + 18 + 26 + 14 + 8) / 8 + (56 + 18 + 26 + 14 + 8) % 8];

    Bit08::set_u8(&mut arr, 250);
    assert_eq!(Bit08::get_u8(&arr), 250);
    Bit08::set_u8(&mut arr, 115);
    assert_eq!(Bit08::get_u8(&arr), 115);
    Bit08::set_u8(&mut arr, 10);
    assert_eq!(Bit08::get_u8(&arr), 10);

    Bit14::set_u16(&mut arr, 0b10_1111_0011_1001);
    assert_eq!(Bit14::get_u16(&arr), 0b10_1111_0011_1001);

    Bit18::set_u32(&mut arr, 0b10_1111_0011_1001);
    assert_eq!(Bit18::get_u32(&arr), 0b10_1111_0011_1001);
    Bit18::set_u32(&mut arr, 0b10_0001_1111_0011_1001);
    assert_eq!(Bit18::get_u32(&arr), 0b10_0001_1111_0011_1001);

    Bit26::set_u32(&mut arr, 0b10_0001_1111_0011_1001);
    assert_eq!(Bit26::get_u32(&arr), 0b10_0001_1111_0011_1001);
    Bit26::set_u32(&mut arr, 0b10_1111_0011_1001_1111_0011_1001);
    assert_eq!(Bit26::get_u32(&arr), 0b10_1111_0011_1001_1111_0011_1001);
    Bit26::set_u32(&mut arr, u32::MAX);
    assert_eq!(Bit26::get_u32(&arr), u32::MAX >> (32 - 26));

    Bit56::set_u64(&mut arr, 0b10_0001_1111_0011_1001);
    assert_eq!(Bit56::get_u64(&arr), 0b10_0001_1111_0011_1001);
    Bit56::set_u64(&mut arr, u64::MAX);
    assert_eq!(Bit56::get_u64(&arr), u64::MAX >> (64 - 56));

    Bit56::set_u64(&mut arr, 0);
    Bit18::set_u32(&mut arr, 0);
    Bit26::set_u32(&mut arr, 0);
    Bit14::set_u16(&mut arr, 0);
    Bit08::set_u8(&mut arr, 0);
    assert_eq!(arr.into_iter().sum::<u8>(), 0);
}

#[test]
fn test_1_32_64() {
    type Bit01 = BitsPos<1, 0>;
    type Bit32 = BitsPos<32, 1>;
    type Bit64 = BitsPos<64, 33>;

    let mut arr = [0; 13];

    Bit01::set_u8(&mut arr, 1);
    assert_eq!(Bit01::get_u8(&arr), 1);
    Bit32::set_u32(&mut arr, u32::MAX);
    assert_eq!(Bit32::get_u32(&arr), u32::MAX);
    Bit64::set_u64(&mut arr, u64::MAX);
    assert_eq!(Bit64::get_u64(&arr), u64::MAX);
}

#[test]
fn test_16_32_64() {
    type Bit16 = BitsPos<16, 0>;
    type Bit32 = BitsPos<32, 16>;
    type Bit64 = BitsPos<64, 48>;

    let mut arr = [0; 14];

    Bit16::set_u16(&mut arr, u16::MAX);
    assert_eq!(Bit16::get_u16(&arr), u16::MAX);
    Bit32::set_u32(&mut arr, u32::MAX);
    assert_eq!(Bit32::get_u32(&arr), u32::MAX);
    Bit64::set_u64(&mut arr, u64::MAX);
    assert_eq!(Bit64::get_u64(&arr), u64::MAX);
}

#[test]
fn test_1_3_4_24() {
    type Bit1 = BitsPos<1, 0>;
    type Bit3 = BitsPos<3, 1>;
    type Bit4 = BitsPos<4, 4>;
    type Bit24 = BitsPos<24, 8>;

    assert_eq!(Bit1::RANGE, 0..=0);
    assert_eq!(Bit3::RANGE, 0..=0);
    assert_eq!(Bit4::RANGE, 0..=0);
    assert_eq!(Bit24::RANGE, 1..=3);

    assert_eq!(Bit1::OFFSET, 0);
    assert_eq!(Bit3::OFFSET, 1);
    assert_eq!(Bit4::OFFSET, 4);
    assert_eq!(Bit24::OFFSET, 0);

    assert_eq!(Bit24::RANGE_U32, 1..=4);

    let mut arr = [0; 4];

    Bit1::set_u8(&mut arr, u8::MAX);
    assert_eq!(Bit1::get_u8(&arr), u8::MAX >> (8 - 1));
    Bit3::set_u8(&mut arr, u8::MAX);
    assert_eq!(Bit3::get_u8(&arr), u8::MAX >> (8 - 3));
    Bit4::set_u8(&mut arr, u8::MAX);
    assert_eq!(Bit4::get_u8(&arr), u8::MAX >> (8 - 4));
    Bit24::set_u32(&mut arr, u32::MAX);
    assert_eq!(Bit24::get_u32(&arr), u32::MAX >> (32 - 24));
}

#[test]
fn test_1_3_4_55_1() {
    type Bit1 = BitsPos<1, 0>;
    type Bit3 = BitsPos<3, 1>;
    type Bit4 = BitsPos<4, 4>;
    type Bit55 = BitsPos<55, 8>;
    type Bit1_ = BitsPos<1, 63>;

    assert_eq!(Bit1::RANGE, 0..=0);
    assert_eq!(Bit3::RANGE, 0..=0);
    assert_eq!(Bit4::RANGE, 0..=0);
    assert_eq!(Bit55::RANGE, 1..=7);
    assert_eq!(Bit1_::RANGE, 7..=7);

    assert_eq!(Bit1::OFFSET, 0);
    assert_eq!(Bit3::OFFSET, 1);
    assert_eq!(Bit4::OFFSET, 4);
    assert_eq!(Bit55::OFFSET, 0);
    assert_eq!(Bit1_::OFFSET, 7);

    let mut arr = [0; 8];

    Bit1::set_u8(&mut arr, u8::MAX);
    assert_eq!(Bit1::get_u8(&arr), u8::MAX >> (8 - 1));
    Bit3::set_u8(&mut arr, u8::MAX);
    assert_eq!(Bit3::get_u8(&arr), u8::MAX >> (8 - 3));
    Bit4::set_u8(&mut arr, u8::MAX);
    assert_eq!(Bit4::get_u8(&arr), u8::MAX >> (8 - 4));
    Bit55::set_u64(&mut arr, u64::MAX);
    assert_eq!(Bit55::get_u64(&arr), u64::MAX >> (64 - 55));
    Bit1_::set_u8(&mut arr, u8::MAX);
    assert_eq!(Bit1_::get_u8(&arr), u8::MAX >> (8 - 1));

    Bit55::set_u64(&mut arr, u32::MAX as u64);
    assert_eq!(Bit55::get_u64(&arr), u32::MAX as u64);
    Bit1_::set_u8(&mut arr, 0);
    assert_eq!(Bit1_::get_u8(&arr), 0);

    Bit1::set_u8(&mut arr, 0);
    Bit3::set_u8(&mut arr, 0);
    Bit4::set_u8(&mut arr, 0);
    Bit55::set_u64(&mut arr, 0);

    assert_eq!(arr.iter().copied().sum::<u8>(), 0);

    Bit1::set_u8(&mut arr, u8::MAX);
    Bit3::set_u8(&mut arr, u8::MAX);
    Bit4::set_u8(&mut arr, u8::MAX);
    Bit55::set_u64(&mut arr, u64::MAX);
    Bit1_::set_u8(&mut arr, u8::MAX);
    assert_eq!(arr.iter().map(|&a| a as usize).sum::<usize>(), arr.len() * u8::MAX as usize);
}

// This is on the contrary with test 04-multiple-of-8bits.
// Because that test assumes the length of the array is a multiple of 8 bits,
// but the lib implementation sets less constrains on array length:
// array length can be equal or greater than needed bits.
#[test]
fn test_1_3_4_23() {
    type Bit1 = BitsPos<1, 0>;
    type Bit3 = BitsPos<3, 1>;
    type Bit4 = BitsPos<4, 4>;
    type Bit23 = BitsPos<23, 8>;

    assert_eq!(Bit1::RANGE, 0..=0);
    assert_eq!(Bit3::RANGE, 0..=0);
    assert_eq!(Bit4::RANGE, 0..=0);
    assert_eq!(Bit23::RANGE, 1..=3);

    assert_eq!(Bit1::OFFSET, 0);
    assert_eq!(Bit3::OFFSET, 1);
    assert_eq!(Bit4::OFFSET, 4);
    assert_eq!(Bit23::OFFSET, 0);

    let mut arr = [0; 4];

    Bit1::set_u8(&mut arr, u8::MAX);
    assert_eq!(Bit1::get_u8(&arr), u8::MAX >> (8 - 1));
    Bit3::set_u8(&mut arr, u8::MAX);
    assert_eq!(Bit3::get_u8(&arr), u8::MAX >> (8 - 3));
    Bit4::set_u8(&mut arr, u8::MAX);
    assert_eq!(Bit4::get_u8(&arr), u8::MAX >> (8 - 4));
    Bit23::set_u32(&mut arr, u32::MAX);
    assert_eq!(Bit23::get_u32(&arr), u32::MAX >> (32 - 23));

    assert_eq!(arr.iter().map(|&a| a as usize).sum::<usize>(), arr.len() * u8::MAX as usize);
}
