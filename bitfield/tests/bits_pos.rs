pub struct BitsPos<const WIDTH: usize, const ACC: usize>;

type Range = std::ops::RangeInclusive<usize>;

impl<const WIDTH: usize, const ACC: usize> BitsPos<WIDTH, ACC> {
    const ACROSS_U08: bool = Self::RANGE_BITS > u8::BITS;
    const ACROSS_U16: bool = Self::RANGE_BITS > u16::BITS;
    const ACROSS_U32: bool = Self::RANGE_BITS > u32::BITS;
    const ACROSS_U64: bool = Self::RANGE_BITS > u64::BITS;
    const ERR: &'static str = "slice 转化 array 时失败";
    const LIMIT_U08: u8 = (u8::MAX >> (8 - WIDTH)) << Self::OFFSET;
    // const LIMIT_U08_END: u8 = u8::MAX >> (8 - Self::OFFSET_END_);
    const LIMIT_U16: u16 = (u16::MAX >> (16 - WIDTH)) << Self::OFFSET;
    // const LIMIT_U16_END: u16 = u16::MAX >> (16 - Self::OFFSET_END_);
    const LIMIT_U32: u32 = (u32::MAX >> (32 - WIDTH)) << Self::OFFSET;
    // const LIMIT_U32_END: u32 = u32::MAX >> (32 - Self::OFFSET_END_);
    const LIMIT_U64: u64 = (u64::MAX >> (64 - WIDTH)) << Self::OFFSET;
    // const LIMIT_U64_END: u64 = (u64::MAX >> (64 - Self::OFFSET_END_) as u8);
    pub const OFFSET: usize = ACC % 8;
    // const OFFSET_END: usize = WIDTH - Self::OFFSET_END_;
    const OFFSET_END: usize = if WIDTH < Self::OFFSET_END_ {
        // 这里其实永远不会使用到：但在字节数过少时可能跳入此情况
        WIDTH
    } else {
        WIDTH - Self::OFFSET_END_
    };
    const OFFSET_END_: usize = (ACC + WIDTH) % 8;
    pub const RANGE: Range = Self::RANGE_LHS..=Self::RANGE_RHS;
    const RANGE_ACROSS: Range = if Self::RANGE_RHS == 0 {
        // 这里其实永远不会使用到：如果为 across，那么一定是第二种情况
        Self::RANGE
    } else {
        Self::RANGE_LHS..=(Self::RANGE_RHS - 1)
    };
    const RANGE_BITS: u32 = (Self::RANGE_LEN * 8) as u32;
    const RANGE_LEN: usize = Self::RANGE_RHS - Self::RANGE_LHS + 1;
    const RANGE_LHS: usize = ACC / 8;
    const RANGE_RHS: usize = (WIDTH + ACC - 1) / 8;
    const RANGE_RHS2: Range = Self::RANGE_RHS..=Self::RANGE_RHS;
    pub const RANGE_U32: Range =
        if Self::RANGE_LEN == 4 { Self::RANGE } else { Self::RANGE_LHS..=Self::RANGE_LHS + 3 };
    const RANGE_U64: Range =
        if Self::RANGE_LEN == 8 { Self::RANGE } else { Self::RANGE_LHS..=Self::RANGE_LHS + 7 };

    pub fn get_u8(arr: &[u8]) -> u8 {
        if Self::ACROSS_U08 {
            let (num_start, num_end) = Self::get_across(arr);
            let num = (u8::from_ne_bytes(num_start) & Self::LIMIT_U08) >> Self::OFFSET;
            let num_end = (num_end as u8 & (u8::MAX >> (8 - Self::OFFSET_END_))) << Self::OFFSET_END;
            num | num_end
        } else {
            let num = u8::from_ne_bytes(Self::get(arr));
            (num & Self::LIMIT_U08) >> Self::OFFSET
        }
    }

    pub fn get_u16(arr: &[u8]) -> u16 {
        if Self::ACROSS_U16 {
            let (num_start, num_end) = Self::get_across(arr);
            let num = (u16::from_ne_bytes(num_start) & Self::LIMIT_U16) >> Self::OFFSET;
            let num_end = (num_end as u16 & (u16::MAX >> (16 - Self::OFFSET_END_))) << Self::OFFSET_END;
            num | num_end
        } else {
            let num = u16::from_ne_bytes(Self::get(arr));
            (num & Self::LIMIT_U16) >> Self::OFFSET
        }
    }

    pub fn get_u32(arr: &[u8]) -> u32 {
        if Self::ACROSS_U32 {
            let (num_start, num_end) = Self::get_across(arr);
            let num = (u32::from_ne_bytes(num_start) & Self::LIMIT_U32) >> Self::OFFSET;
            let num_end = (num_end as u32 & (u32::MAX >> (32 - Self::OFFSET_END_))) << Self::OFFSET_END;
            num | num_end
        } else {
            let num = Self::u32(arr).0;
            (num & Self::LIMIT_U32) >> Self::OFFSET
        }
    }

    fn u32(arr: &[u8]) -> (u32, Range, usize) {
        let len = arr[Self::RANGE_LHS..].len();
        if len < 4 {
            let mut tmp = [0; 4];
            let range = Self::RANGE_LHS..=len;
            tmp[..len].copy_from_slice(&arr[range.clone()]);
            (u32::from_le_bytes(tmp), range, len)
        } else {
            (u32::from_ne_bytes(arr[Self::RANGE_U32].try_into().expect(Self::ERR)), Self::RANGE_U32, 4)
        }
    }

    fn u64(arr: &[u8]) -> (u64, Range, usize) {
        let len = arr[Self::RANGE_LHS..].len();
        if len < 8 {
            let mut tmp = [0; 8];
            let range = Self::RANGE_LHS..=len;
            tmp[..len].copy_from_slice(&arr[range.clone()]);
            (u64::from_le_bytes(tmp), range, len)
        } else {
            (u64::from_ne_bytes(arr[Self::RANGE_U64].try_into().expect(Self::ERR)), Self::RANGE_U64, 8)
        }
    }

    pub fn get_u64(arr: &[u8]) -> u64 {
        if Self::ACROSS_U64 {
            let (num_start, num_end) = Self::get_across(arr);
            let num = (u64::from_ne_bytes(num_start) & Self::LIMIT_U64) >> Self::OFFSET;
            let num_end = ((num_end as u64) & (u64::MAX >> (64 - Self::OFFSET_END_))) << Self::OFFSET_END;
            num | num_end
        } else {
            // let num = u64::from_ne_bytes(arr[Self::RANGE_U64].try_into().expect(Self::ERR));
            let num = Self::u64(arr).0;
            (num & Self::LIMIT_U64) >> Self::OFFSET
        }
    }

    pub fn get<'a, T: TryFrom<&'a [u8]>>(arr: &'a [u8]) -> T
        where <T as TryFrom<&'a [u8]>>::Error: std::fmt::Debug {
        T::try_from(&arr[Self::RANGE]).expect(Self::ERR)
    }

    pub fn get_across<'a, T: TryFrom<&'a [u8]>>(arr: &'a [u8]) -> (T, u8)
        where <T as TryFrom<&'a [u8]>>::Error: std::fmt::Debug {
        (T::try_from(&arr[Self::RANGE_ACROSS]).expect(Self::ERR),
         u8::from_ne_bytes(arr[Self::RANGE_RHS2].try_into().expect(Self::ERR)))
    }

    pub fn across_end(arr: &mut [u8], num_end: u8) {
        let p = &mut arr[Self::RANGE_RHS2];
        let num_old = u8::from_ne_bytes(p.try_into().unwrap());
        let num_new = num_old & !(u8::MAX >> (8 - Self::OFFSET_END_)) | num_end;
        p.copy_from_slice(&num_new.to_ne_bytes());
    }

    pub fn set_u8(arr: &mut [u8], num: u8) {
        if Self::ACROSS_U08 {
            let p = &mut arr[Self::RANGE_ACROSS];
            let num_old = u8::from_ne_bytes(p.try_into().unwrap());
            let num_new = num_old & !Self::LIMIT_U08 | (num << Self::OFFSET);
            p.copy_from_slice(&num_new.to_ne_bytes());

            let num_end = (num >> (8 - Self::OFFSET)) as u8;
            Self::across_end(arr, num_end);
        } else {
            let p = &mut arr[Self::RANGE];
            let num_old = u8::from_ne_bytes(p.try_into().expect(Self::ERR));
            let num_new = num_old & !Self::LIMIT_U08 | (num << Self::OFFSET);
            p.copy_from_slice(&num_new.to_ne_bytes());
        }
    }

    pub fn set_u16(arr: &mut [u8], num: u16) {
        if Self::ACROSS_U16 {
            let p = &mut arr[Self::RANGE_ACROSS];
            let num_old = u16::from_ne_bytes(p.try_into().unwrap());
            let num_new = num_old & !Self::LIMIT_U16 | (num << Self::OFFSET);
            p.copy_from_slice(&num_new.to_ne_bytes());

            let num_end = (num >> (16 - Self::OFFSET)) as u8;
            Self::across_end(arr, num_end);
        } else {
            let p = &mut arr[Self::RANGE];
            let num_old = u16::from_ne_bytes(p.try_into().expect(Self::ERR));
            let num_new = num_old & !Self::LIMIT_U16 | (num << Self::OFFSET);
            p.copy_from_slice(&num_new.to_ne_bytes());
        }
    }

    pub fn set_u32(arr: &mut [u8], num: u32) {
        if Self::ACROSS_U32 {
            let p = &mut arr[Self::RANGE_ACROSS];
            let num_old = u32::from_ne_bytes(p.try_into().unwrap());
            let num_new = num_old & !Self::LIMIT_U32 | (num << Self::OFFSET);
            p.copy_from_slice(&num_new.to_ne_bytes());

            let num_end = (num >> (32 - Self::OFFSET)) as u8;
            Self::across_end(arr, num_end);
        } else {
            let (num_old, range, len) = Self::u32(arr);
            let p = &mut arr[range];
            // let num_old = u32::from_ne_bytes(p.try_into().expect(Self::ERR));
            let num_new = num_old & !Self::LIMIT_U32 | (num << Self::OFFSET);
            if len == 4 {
                p.copy_from_slice(&num_new.to_ne_bytes());
            } else {
                p.copy_from_slice(&num_new.to_le_bytes()[..len])
            }
        }
    }

    pub fn set_u64(arr: &mut [u8], num: u64) {
        if Self::ACROSS_U64 {
            let p = &mut arr[Self::RANGE_ACROSS];
            let num_old = u64::from_ne_bytes(p.try_into().unwrap());
            let num_new = num_old & !Self::LIMIT_U64 | (num << Self::OFFSET);
            p.copy_from_slice(&num_new.to_ne_bytes());

            let num_end = (num >> (64 - Self::OFFSET)) as u8;
            Self::across_end(arr, num_end);
        } else {
            let (num_old, range, len) = Self::u64(arr);
            let p = &mut arr[range];
            // let num_old = u64::from_ne_bytes(p.try_into().expect(Self::ERR));
            let num_new = num_old & !Self::LIMIT_U64 | (num << Self::OFFSET);
            if len == 8 {
                p.copy_from_slice(&num_new.to_ne_bytes());
            } else {
                p.copy_from_slice(&num_new.to_le_bytes()[..len])
            }
        }
    }
}

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
