#![allow(unused)]

const ERR: &str = "slice 转化 array 时失败";

pub struct BitsU8<const WIDTH: usize, const ACC: usize>;

trait Basic<const WIDTH: usize, const ACC: usize> {
    const ACROSS_U08: bool = Self::RANGE_BITS > u8::BITS;
    const OFFSET: usize = ACC % 8;
    // const OFFSET_END: usize = WIDTH - Self::OFFSET_END_;
    const OFFSET_END: usize = if WIDTH < Self::OFFSET_END_ {
        // 这里其实永远不会使用到：但在字节数过少时可能跳入此情况
        WIDTH
    } else {
        WIDTH - Self::OFFSET_END_
    };
    const OFFSET_END_: usize = (ACC + WIDTH) % 8;
    const RANGE: Range = Self::RANGE_LHS..=Self::RANGE_RHS;
    const RANGE_ACROSS: Range = if Self::RANGE_RHS == 0 {
        // 这里其实永远不会使用到
        Self::RANGE
    } else {
        Self::RANGE_LHS..=(Self::RANGE_RHS - 1)
    };
    const RANGE_BITS: u32 = (Self::RANGE_LEN * 8) as u32;
    const RANGE_LEN: usize = Self::RANGE_RHS - Self::RANGE_LHS + 1;
    const RANGE_LHS: usize = ACC / 8;
    const RANGE_RHS: usize = (WIDTH + ACC - 1) / 8;
}

impl<const WIDTH: usize, const ACC: usize> Basic<WIDTH, ACC> for BitsU8<WIDTH, ACC> {}

// impl<const WIDTH: usize, const ACC: usize> Bits<WIDTH, ACC> {
//     const ACROSS_U08: bool = Self::RANGE_BITS > u8::BITS;
//     const OFFSET: usize = ACC % 8;
//     const OFFSET_END: usize = WIDTH - Self::OFFSET_END_;
//     const OFFSET_END_: usize = (ACC + WIDTH) % 8;
//     const RANGE: Range = Self::RANGE_LHS..=Self::RANGE_RHS;
//     const RANGE_ACROSS: Range = if Self::RANGE_RHS == 0 {
//         // 这里其实永远不会使用到
//         Self::RANGE
//     } else {
//         Self::RANGE_LHS..=(Self::RANGE_RHS - 1)
//     };
//     const RANGE_BITS: u32 = (Self::RANGE_LEN * 8) as u32;
//     const RANGE_LEN: usize = Self::RANGE_RHS - Self::RANGE_LHS + 1;
//     const RANGE_LHS: usize = ACC / 8;
//     const RANGE_RHS: usize = (WIDTH + ACC - 1) / 8;
// }

trait SetGet<T> {
    const ACROSS: bool;
    const LIMIT: T;
    const RANGE_ALT: Range;
    const RANGE_RHS2: Range;
    const GET: fn(&[u8]) -> T;
    const SET: fn(&mut [u8], T);
    const U8_MAX_OFFSET: u8;
    fn set_across(arr: &mut [u8], num: T);
    fn set_no_across(arr: &mut [u8], num: T);
    fn get_across(arr: &[u8]) -> T;
    fn get_no_across(arr: &[u8]) -> T;
    fn across_end(arr: &mut [u8], num_end: u8) {
        let p = &mut arr[Self::RANGE_RHS2];
        let num_old = u8::from_ne_bytes(p.try_into().expect(ERR));
        let num_new = num_old & Self::U8_MAX_OFFSET | num_end;
        p.copy_from_slice(&num_new.to_ne_bytes());
    }
}

impl<const WIDTH: usize, const ACC: usize> SetGet<u8> for BitsU8<WIDTH, ACC> {
    const ACROSS: bool = Self::RANGE_BITS > u8::BITS;
    const GET: fn(&[u8]) -> u8 = if Self::ACROSS { Self::get_across } else { Self::get_no_across };
    const LIMIT: u8 = (u8::MAX >> (8 - WIDTH)) << Self::OFFSET;
    const RANGE_ALT: Range = Self::RANGE_RHS..=Self::RANGE_RHS;
    const RANGE_RHS2: Range = Self::RANGE_RHS..=Self::RANGE_RHS;
    const SET: fn(&mut [u8], u8) = if Self::ACROSS { Self::set_across } else { Self::set_no_across };
    const U8_MAX_OFFSET: u8 = !(u8::MAX >> (8 - Self::OFFSET_END_));

    fn set_across(arr: &mut [u8], num: u8) {
        let p = &mut arr[Self::RANGE_ACROSS];
        let num_old = u8::from_ne_bytes(p.try_into().unwrap());
        let num_new = num_old & !Self::LIMIT | (num << Self::OFFSET);
        p.copy_from_slice(&num_new.to_ne_bytes());

        let num_end = (num >> (8 - Self::OFFSET)) as u8;
        Self::across_end(arr, num_end);
    }

    fn set_no_across(arr: &mut [u8], num: u8) {
        let p = &mut arr[Self::RANGE];
        let num_old = u8::from_ne_bytes(p.try_into().expect(ERR));
        let num_new = num_old & !Self::LIMIT | (num << Self::OFFSET);
        p.copy_from_slice(&num_new.to_ne_bytes());
    }

    fn get_across(arr: &[u8]) -> u8 {
        let num_start = u8::from_le_bytes(arr[Self::RANGE_ACROSS].try_into().expect(ERR));
        let num_start = (num_start & Self::LIMIT) >> Self::OFFSET;
        let num_end = u8::from_ne_bytes(arr[Self::RANGE_ALT].try_into().expect(ERR));
        let num_end = (num_end as u8 & (u8::MAX >> (8 - Self::OFFSET_END_))) << Self::OFFSET_END;
        num_start | num_end
    }

    fn get_no_across(arr: &[u8]) -> u8 {
        let num = u8::from_ne_bytes(arr[Self::RANGE].try_into().expect(ERR));
        (num & Self::LIMIT) >> Self::OFFSET
    }
}

pub struct BitsU16<const WIDTH: usize, const ACC: usize>;

impl<const WIDTH: usize, const ACC: usize> Basic<WIDTH, ACC> for BitsU16<WIDTH, ACC> {}
impl<const WIDTH: usize, const ACC: usize> BitsU16<WIDTH, ACC> {
    fn get_across2<'a, T: TryFrom<&'a [u8]>>(arr: &'a [u8]) -> (T, u8)
        where <T as TryFrom<&'a [u8]>>::Error: std::fmt::Debug {
        (T::try_from(&arr[Self::RANGE_ACROSS]).expect(ERR),
         u8::from_ne_bytes(arr[Self::RANGE_RHS2].try_into().expect(ERR)))
    }
}

impl<const WIDTH: usize, const ACC: usize> SetGet<u16> for BitsU16<WIDTH, ACC> {
    const ACROSS: bool = Self::RANGE_BITS > u16::BITS;
    const GET: fn(&[u8]) -> u16 = if Self::ACROSS { Self::get_across } else { Self::get_no_across };
    const LIMIT: u16 = (u16::MAX >> (16 - WIDTH)) << Self::OFFSET;
    const RANGE_ALT: Range = Self::RANGE_RHS..=Self::RANGE_RHS;
    const RANGE_RHS2: Range = Self::RANGE_RHS..=Self::RANGE_RHS;
    const SET: fn(&mut [u8], u16) = if Self::ACROSS { Self::set_across } else { Self::set_no_across };
    const U8_MAX_OFFSET: u8 = !(u8::MAX >> (8 - Self::OFFSET_END_));

    fn set_across(arr: &mut [u8], num: u16) {
        let p = &mut arr[Self::RANGE_ACROSS];
        let num_old = u16::from_ne_bytes(p.try_into().unwrap());
        let num_new = num_old & !Self::LIMIT | (num << Self::OFFSET);
        p.copy_from_slice(&num_new.to_ne_bytes());

        let num_end = (num >> (16 - Self::OFFSET)) as u8;
        Self::across_end(arr, num_end);
    }

    fn set_no_across(arr: &mut [u8], num: u16) {
        let p = &mut arr[Self::RANGE];
        let num_old = u16::from_ne_bytes(p.try_into().expect(ERR));
        let num_new = num_old & !Self::LIMIT | (num << Self::OFFSET);
        p.copy_from_slice(&num_new.to_ne_bytes());
    }

    fn get_across(arr: &[u8]) -> u16 {
        let (num_start, num_end) = Self::get_across2(arr);
        let num = (u16::from_ne_bytes(num_start) & Self::LIMIT) >> Self::OFFSET;
        let num_end = (num_end as u16 & (u16::MAX >> (16 - Self::OFFSET_END_))) << Self::OFFSET_END;
        num | num_end
    }

    fn get_no_across(arr: &[u8]) -> u16 {
        let num = u16::from_ne_bytes(arr[Self::RANGE].try_into().expect(ERR));
        (num & Self::LIMIT) >> Self::OFFSET
    }
}

pub struct BitsU32<const WIDTH: usize, const ACC: usize>;

impl<const WIDTH: usize, const ACC: usize> Basic<WIDTH, ACC> for BitsU32<WIDTH, ACC> {}

impl<const WIDTH: usize, const ACC: usize> BitsU32<WIDTH, ACC> {
    fn u32(arr: &[u8]) -> (u32, Range, usize) {
        let len = arr[Self::RANGE_LHS..].len();
        if len < 4 {
            let mut tmp = [0; 4];
            let range = Self::RANGE_LHS..=len;
            tmp[..len].copy_from_slice(&arr[range.clone()]);
            (u32::from_le_bytes(tmp), range, len)
        } else {
            (u32::from_ne_bytes(arr[Self::RANGE_ALT].try_into().expect(ERR)), Self::RANGE_ALT, 4)
        }
    }

    fn get_across2<'a, T: TryFrom<&'a [u8]>>(arr: &'a [u8]) -> (T, u8)
        where <T as TryFrom<&'a [u8]>>::Error: std::fmt::Debug {
        (T::try_from(&arr[Self::RANGE_ACROSS]).expect(ERR),
         u8::from_ne_bytes(arr[Self::RANGE_RHS2].try_into().expect(ERR)))
    }
}

impl<const WIDTH: usize, const ACC: usize> SetGet<u32> for BitsU32<WIDTH, ACC> {
    const ACROSS: bool = Self::RANGE_BITS > u32::BITS;
    const GET: fn(&[u8]) -> u32 = if Self::ACROSS { Self::get_across } else { Self::get_no_across };
    const LIMIT: u32 = (u32::MAX >> (32 - WIDTH)) << Self::OFFSET;
    const RANGE_ALT: Range =
        if Self::RANGE_LEN == 4 { Self::RANGE } else { Self::RANGE_LHS..=Self::RANGE_LHS + 3 };
    const RANGE_RHS2: Range = Self::RANGE_RHS..=Self::RANGE_RHS;
    const SET: fn(&mut [u8], u32) = if Self::ACROSS { Self::set_across } else { Self::set_no_across };
    const U8_MAX_OFFSET: u8 = !(u8::MAX >> (8 - Self::OFFSET_END_));

    fn set_across(arr: &mut [u8], num: u32) {
        let p = &mut arr[Self::RANGE_ACROSS];
        let num_old = u32::from_ne_bytes(p.try_into().unwrap());
        let num_new = num_old & !Self::LIMIT | (num << Self::OFFSET);
        p.copy_from_slice(&num_new.to_ne_bytes());

        let num_end = (num >> (32 - Self::OFFSET)) as u8;
        Self::across_end(arr, num_end);
    }

    fn set_no_across(arr: &mut [u8], num: u32) {
        let (num_old, range, len) = Self::u32(arr);
        let p = &mut arr[range];
        // let num_old = u32::from_ne_bytes(p.try_into().expect(Self::ERR));
        let num_new = num_old & !Self::LIMIT | (num << Self::OFFSET);
        if len == 4 {
            p.copy_from_slice(&num_new.to_ne_bytes());
        } else {
            p.copy_from_slice(&num_new.to_le_bytes()[..len])
        }
    }

    fn get_across(arr: &[u8]) -> u32 {
        let (num_start, num_end) = Self::get_across2(arr);
        let num = (u32::from_ne_bytes(num_start) & Self::LIMIT) >> Self::OFFSET;
        let num_end = (num_end as u32 & (u32::MAX >> (32 - Self::OFFSET_END_))) << Self::OFFSET_END;
        num | num_end
    }

    fn get_no_across(arr: &[u8]) -> u32 {
        let num = Self::u32(arr).0;
        (num & Self::LIMIT) >> Self::OFFSET
    }
}

pub struct BitsU64<const WIDTH: usize, const ACC: usize>;

impl<const WIDTH: usize, const ACC: usize> Basic<WIDTH, ACC> for BitsU64<WIDTH, ACC> {}

impl<const WIDTH: usize, const ACC: usize> BitsU64<WIDTH, ACC> {
    fn u64(arr: &[u8]) -> (u64, Range, usize) {
        let len = arr[Self::RANGE_LHS..].len();
        if len < 8 {
            let mut tmp = [0; 8];
            let range = Self::RANGE_LHS..=len;
            tmp[..len].copy_from_slice(&arr[range.clone()]);
            (u64::from_le_bytes(tmp), range, len)
        } else {
            (u64::from_ne_bytes(arr[Self::RANGE_ALT].try_into().expect(ERR)), Self::RANGE_ALT, 8)
        }
    }

    fn get_across2<'a, T: TryFrom<&'a [u8]>>(arr: &'a [u8]) -> (T, u8)
        where <T as TryFrom<&'a [u8]>>::Error: std::fmt::Debug {
        (T::try_from(&arr[Self::RANGE_ACROSS]).expect(ERR),
         u8::from_ne_bytes(arr[Self::RANGE_RHS2].try_into().expect(ERR)))
    }
}

impl<const WIDTH: usize, const ACC: usize> SetGet<u64> for BitsU64<WIDTH, ACC> {
    const ACROSS: bool = Self::RANGE_BITS > u64::BITS;
    const GET: fn(&[u8]) -> u64 = if Self::ACROSS { Self::get_across } else { Self::get_no_across };
    const LIMIT: u64 = (u64::MAX >> (64 - WIDTH)) << Self::OFFSET;
    const RANGE_ALT: Range =
        if Self::RANGE_LEN == 8 { Self::RANGE } else { Self::RANGE_LHS..=Self::RANGE_LHS + 7 };
    const RANGE_RHS2: Range = Self::RANGE_RHS..=Self::RANGE_RHS;
    const SET: fn(&mut [u8], u64) = if Self::ACROSS { Self::set_across } else { Self::set_no_across };
    const U8_MAX_OFFSET: u8 = !(u8::MAX >> (8 - Self::OFFSET_END_));

    fn set_across(arr: &mut [u8], num: u64) {
        let p = &mut arr[Self::RANGE_ACROSS];
        let num_old = u64::from_ne_bytes(p.try_into().unwrap());
        let num_new = num_old & !Self::LIMIT | (num << Self::OFFSET);
        p.copy_from_slice(&num_new.to_ne_bytes());

        let num_end = (num >> (64 - Self::OFFSET)) as u8;
        Self::across_end(arr, num_end);
    }

    fn set_no_across(arr: &mut [u8], num: u64) {
        let (num_old, range, len) = Self::u64(arr);
        let p = &mut arr[range];
        // let num_old = u64::from_ne_bytes(p.try_into().expect(Self::ERR));
        let num_new = num_old & !Self::LIMIT | (num << Self::OFFSET);
        if len == 8 {
            p.copy_from_slice(&num_new.to_ne_bytes());
        } else {
            p.copy_from_slice(&num_new.to_le_bytes()[..len])
        }
    }

    fn get_across(arr: &[u8]) -> u64 {
        let (num_start, num_end) = Self::get_across2(arr);
        let num = (u64::from_ne_bytes(num_start) & Self::LIMIT) >> Self::OFFSET;
        let num_end = ((num_end as u64) & (u64::MAX >> (64 - Self::OFFSET_END_))) << Self::OFFSET_END;
        num | num_end
    }

    fn get_no_across(arr: &[u8]) -> u64 {
        let num = Self::u64(arr).0;
        (num & Self::LIMIT) >> Self::OFFSET
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

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
