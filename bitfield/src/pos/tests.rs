//! 此测试应与 tests/bits_pos.rs 保持一致

use crate::{Basic, BitsU16, BitsU32, BitsU64, BitsU8, SetGet};

#[test]
fn test_56_18_26_14_8() {
    // [56, 18, 26, 14, 8]
    const SIZE: usize = (56 + 18 + 26 + 14 + 8) / 8 + ((56 + 18 + 26 + 14 + 8) % 8 != 0) as usize;
    type Bit56 = BitsU64<56, 0, SIZE>;
    type Bit18 = BitsU32<18, 56, SIZE>;
    type Bit26 = BitsU32<26, 74, SIZE>;
    type Bit14 = BitsU16<14, 100, SIZE>;
    type Bit08 = BitsU8<8, 114, SIZE>;

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

    let mut arr = [0; SIZE];

    Bit08::SET(&mut arr, 250);
    assert_eq!(Bit08::GET(&arr), 250);
    Bit08::SET(&mut arr, 115);
    assert_eq!(Bit08::GET(&arr), 115);
    Bit08::SET(&mut arr, 10);
    assert_eq!(Bit08::GET(&arr), 10);

    Bit14::SET(&mut arr, 0b10_1111_0011_1001);
    assert_eq!(Bit14::GET(&arr), 0b10_1111_0011_1001);

    Bit18::SET(&mut arr, 0b10_1111_0011_1001);
    assert_eq!(Bit18::GET(&arr), 0b10_1111_0011_1001);
    Bit18::SET(&mut arr, 0b10_0001_1111_0011_1001);
    assert_eq!(Bit18::GET(&arr), 0b10_0001_1111_0011_1001);

    Bit26::SET(&mut arr, 0b10_0001_1111_0011_1001);
    assert_eq!(Bit26::GET(&arr), 0b10_0001_1111_0011_1001);
    Bit26::SET(&mut arr, 0b10_1111_0011_1001_1111_0011_1001);
    assert_eq!(Bit26::GET(&arr), 0b10_1111_0011_1001_1111_0011_1001);
    Bit26::SET(&mut arr, u32::MAX);
    assert_eq!(Bit26::GET(&arr), u32::MAX >> (32 - 26));

    Bit56::SET(&mut arr, 0b10_0001_1111_0011_1001);
    assert_eq!(Bit56::GET(&arr), 0b10_0001_1111_0011_1001);
    Bit56::SET(&mut arr, u64::MAX);
    assert_eq!(Bit56::GET(&arr), u64::MAX >> (64 - 56));

    Bit56::SET(&mut arr, 0);
    Bit18::SET(&mut arr, 0);
    Bit26::SET(&mut arr, 0);
    Bit14::SET(&mut arr, 0);
    Bit08::SET(&mut arr, 0);
    assert_eq!(arr.into_iter().sum::<u8>(), 0);
}

#[test]
fn test_1_32_64() {
    const SIZE: usize = (1 + 32 + 64) / 8 + ((1 + 32 + 64) % 8 != 0) as usize;
    type Bit01 = BitsU8<1, 0, SIZE>;
    type Bit32 = BitsU32<32, 1, SIZE>;
    type Bit64 = BitsU64<64, 33, SIZE>;

    let mut arr = [0; SIZE];

    Bit01::SET(&mut arr, 1);
    assert_eq!(Bit01::GET(&arr), 1);
    Bit32::SET(&mut arr, u32::MAX);
    assert_eq!(Bit32::GET(&arr), u32::MAX);
    Bit64::SET(&mut arr, u64::MAX);
    assert_eq!(Bit64::GET(&arr), u64::MAX);
}

#[test]
fn test_16_32_64() {
    const SIZE: usize = (16 + 32 + 64) / 8 + ((16 + 32 + 64) % 8 != 0) as usize;
    type Bit16 = BitsU16<16, 0, SIZE>;
    type Bit32 = BitsU32<32, 16, SIZE>;
    type Bit64 = BitsU64<64, 48, SIZE>;

    let mut arr = [0; SIZE];

    Bit16::SET(&mut arr, u16::MAX);
    assert_eq!(Bit16::GET(&arr), u16::MAX);
    Bit32::SET(&mut arr, u32::MAX);
    assert_eq!(Bit32::GET(&arr), u32::MAX);
    Bit64::SET(&mut arr, u64::MAX);
    assert_eq!(Bit64::GET(&arr), u64::MAX);
}

#[test]
fn test_1_3_4_24() {
    const SIZE: usize = (1 + 3 + 4 + 24) / 8 + ((1 + 3 + 4 + 24) % 8 != 0) as usize;
    type Bit1 = BitsU8<1, 0, SIZE>;
    type Bit3 = BitsU8<3, 1, SIZE>;
    type Bit4 = BitsU8<4, 4, SIZE>;
    type Bit24 = BitsU32<24, 8, SIZE>;

    assert_eq!(Bit1::RANGE, 0..=0);
    assert_eq!(Bit3::RANGE, 0..=0);
    assert_eq!(Bit4::RANGE, 0..=0);
    assert_eq!(Bit24::RANGE, 1..=3);

    assert_eq!(Bit1::OFFSET, 0);
    assert_eq!(Bit3::OFFSET, 1);
    assert_eq!(Bit4::OFFSET, 4);
    assert_eq!(Bit24::OFFSET, 0);

    assert_eq!(Bit24::RANGE_ALT, 1..=4);

    let mut arr = [0; SIZE];

    Bit1::SET(&mut arr, u8::MAX);
    assert_eq!(Bit1::GET(&arr), u8::MAX >> (8 - 1));
    Bit3::SET(&mut arr, u8::MAX);
    assert_eq!(Bit3::GET(&arr), u8::MAX >> (8 - 3));
    Bit4::SET(&mut arr, u8::MAX);
    assert_eq!(Bit4::GET(&arr), u8::MAX >> (8 - 4));
    Bit24::SET(&mut arr, u32::MAX);
    assert_eq!(Bit24::GET(&arr), u32::MAX >> (32 - 24));
}

#[test]
fn test_1_3_4_55_1() {
    const SIZE: usize = (1 + 3 + 4 + 55 + 1) / 8 + ((1 + 3 + 4 + 55 + 1) % 8 != 0) as usize;
    type Bit1 = BitsU8<1, 0, SIZE>;
    type Bit3 = BitsU8<3, 1, SIZE>;
    type Bit4 = BitsU8<4, 4, SIZE>;
    type Bit55 = BitsU64<55, 8, SIZE>;
    type Bit1_ = BitsU8<1, 63, SIZE>;

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

    let mut arr = [0; SIZE];

    Bit1::SET(&mut arr, u8::MAX);
    assert_eq!(Bit1::GET(&arr), u8::MAX >> (8 - 1));
    Bit3::SET(&mut arr, u8::MAX);
    assert_eq!(Bit3::GET(&arr), u8::MAX >> (8 - 3));
    Bit4::SET(&mut arr, u8::MAX);
    assert_eq!(Bit4::GET(&arr), u8::MAX >> (8 - 4));
    Bit55::SET(&mut arr, u64::MAX);
    assert_eq!(Bit55::GET(&arr), u64::MAX >> (64 - 55));
    Bit1_::SET(&mut arr, u8::MAX);
    assert_eq!(Bit1_::GET(&arr), u8::MAX >> (8 - 1));

    Bit55::SET(&mut arr, u32::MAX as u64);
    assert_eq!(Bit55::GET(&arr), u32::MAX as u64);
    Bit1_::SET(&mut arr, 0);
    assert_eq!(Bit1_::GET(&arr), 0);

    Bit1::SET(&mut arr, 0);
    Bit3::SET(&mut arr, 0);
    Bit4::SET(&mut arr, 0);
    Bit55::SET(&mut arr, 0);

    assert_eq!(arr.iter().copied().sum::<u8>(), 0);

    Bit1::SET(&mut arr, u8::MAX);
    Bit3::SET(&mut arr, u8::MAX);
    Bit4::SET(&mut arr, u8::MAX);
    Bit55::SET(&mut arr, u64::MAX);
    Bit1_::SET(&mut arr, u8::MAX);
    assert_eq!(
        arr.iter().map(|&a| a as usize).sum::<usize>(),
        arr.len() * u8::MAX as usize
    );
}

// This is on the contrary with test 04-multiple-of-8bits.
#[test]
fn test_1_3_4_23() {
    const SIZE: usize = (1 + 3 + 4 + 23) / 8 + ((1 + 3 + 4 + 23) % 8 != 0) as usize;
    type Bit1 = BitsU8<1, 0, SIZE>;
    type Bit3 = BitsU8<3, 1, SIZE>;
    type Bit4 = BitsU8<4, 4, SIZE>;
    type Bit23 = BitsU32<23, 8, SIZE>;

    assert_eq!(Bit1::RANGE, 0..=0);
    assert_eq!(Bit3::RANGE, 0..=0);
    assert_eq!(Bit4::RANGE, 0..=0);
    assert_eq!(Bit23::RANGE, 1..=3);

    assert_eq!(Bit1::OFFSET, 0);
    assert_eq!(Bit3::OFFSET, 1);
    assert_eq!(Bit4::OFFSET, 4);
    assert_eq!(Bit23::OFFSET, 0);

    let mut arr = [0; SIZE];

    Bit1::SET(&mut arr, u8::MAX);
    assert_eq!(Bit1::GET(&arr), u8::MAX >> (8 - 1));
    Bit3::SET(&mut arr, u8::MAX);
    assert_eq!(Bit3::GET(&arr), u8::MAX >> (8 - 3));
    Bit4::SET(&mut arr, u8::MAX);
    assert_eq!(Bit4::GET(&arr), u8::MAX >> (8 - 4));
    Bit23::SET(&mut arr, u32::MAX);
    assert_eq!(Bit23::GET(&arr), u32::MAX >> (32 - 23));

    assert_eq!(
        arr.iter().map(|&a| a as usize).sum::<usize>(),
        arr.len() * u8::MAX as usize
    );
}
