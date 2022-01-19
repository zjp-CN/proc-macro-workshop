const ERR: &str = "slice 转化 array 时失败";
type Range = std::ops::RangeInclusive<usize>;

pub struct BitsU8<const WIDTH: usize, const ACC: usize>;

pub trait Basic<const WIDTH: usize, const ACC: usize> {
    const OFFSET: usize = ACC % 8;
    const OFFSET_END: usize = WIDTH - Self::OFFSET_END_;
    const OFFSET_END_: usize = (ACC + WIDTH) % 8;
    const RANGE: Range = Self::RANGE_LHS..=Self::RANGE_RHS;
    const RANGE_BITS: u32 = (Self::RANGE_LEN * 8) as u32;
    const RANGE_LEN: usize = Self::RANGE_RHS - Self::RANGE_LHS + 1;
    const RANGE_LHS: usize = ACC / 8;
    const RANGE_RHS: usize = (WIDTH + ACC - 1) / 8;
}

impl<const WIDTH: usize, const ACC: usize> Basic<WIDTH, ACC> for BitsU8<WIDTH, ACC> {}

pub trait SetGet {
    type Target;
    const ACROSS: bool;
    const GET: fn(&[u8]) -> Self::Target;
    const LIMIT: Self::Target;
    const RANGE_ALT: Range;
    const RANGE_RHS2: Range;
    const RANGE_ACROSS: Range;
    const SET: fn(&mut [u8], Self::Target);
    const U8_MAX_OFFSET: u8;
    fn set_across(arr: &mut [u8], num: Self::Target);
    fn set_no_across(arr: &mut [u8], num: Self::Target);
    fn get_across(arr: &[u8]) -> Self::Target;
    fn get_no_across(arr: &[u8]) -> Self::Target;
    fn across_end(arr: &mut [u8], num_end: u8) {
        let p = &mut arr[Self::RANGE_RHS2];
        let num_old = u8::from_ne_bytes(p.try_into().expect(ERR));
        let num_new = num_old & Self::U8_MAX_OFFSET | num_end;
        p.copy_from_slice(&num_new.to_ne_bytes());
    }
    fn get_across_helper<'a, T: TryFrom<&'a [u8]>>(arr: &'a [u8]) -> (T, u8)
        where <T as TryFrom<&'a [u8]>>::Error: std::fmt::Debug {
        (T::try_from(&arr[Self::RANGE_ACROSS]).expect(ERR),
         u8::from_ne_bytes(arr[Self::RANGE_RHS2].try_into().expect(ERR)))
    }
}

impl<const WIDTH: usize, const ACC: usize> SetGet for BitsU8<WIDTH, ACC> {
    type Target = u8;

    const ACROSS: bool = Self::RANGE_BITS > u8::BITS;
    const GET: fn(&[u8]) -> u8 = if Self::ACROSS { Self::get_across } else { Self::get_no_across };
    const LIMIT: u8 = (u8::MAX >> (8 - WIDTH)) << Self::OFFSET;
    const RANGE_ACROSS: Range = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
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
        let (num_start, num_end) = Self::get_across_helper(arr);
        let num_start = (u8::from_ne_bytes(num_start) & Self::LIMIT) >> Self::OFFSET;
        let num_end = (num_end & (u8::MAX >> (8 - Self::OFFSET_END_))) << Self::OFFSET_END;
        num_start | num_end
    }

    fn get_no_across(arr: &[u8]) -> u8 {
        let num = u8::from_ne_bytes(arr[Self::RANGE].try_into().expect(ERR));
        (num & Self::LIMIT) >> Self::OFFSET
    }
}

pub struct BitsU16<const WIDTH: usize, const ACC: usize>;

impl<const WIDTH: usize, const ACC: usize> Basic<WIDTH, ACC> for BitsU16<WIDTH, ACC> {}

impl<const WIDTH: usize, const ACC: usize> SetGet for BitsU16<WIDTH, ACC> {
    type Target = u16;

    const ACROSS: bool = Self::RANGE_BITS > u16::BITS;
    const GET: fn(&[u8]) -> u16 = if Self::ACROSS { Self::get_across } else { Self::get_no_across };
    const LIMIT: u16 = (u16::MAX >> (16 - WIDTH)) << Self::OFFSET;
    const RANGE_ACROSS: Range = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
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
        let (num_start, num_end) = Self::get_across_helper(arr);
        let num_start = (u16::from_ne_bytes(num_start) & Self::LIMIT) >> Self::OFFSET;
        let num_end = (num_end as u16 & (u16::MAX >> (16 - Self::OFFSET_END_))) << Self::OFFSET_END;
        num_start | num_end
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
}

impl<const WIDTH: usize, const ACC: usize> SetGet for BitsU32<WIDTH, ACC> {
    type Target = u32;

    const ACROSS: bool = Self::RANGE_BITS > u32::BITS;
    const GET: fn(&[u8]) -> u32 = if Self::ACROSS { Self::get_across } else { Self::get_no_across };
    const LIMIT: u32 = (u32::MAX >> (32 - WIDTH)) << Self::OFFSET;
    const RANGE_ACROSS: Range = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
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
        let num_new = num_old & !Self::LIMIT | (num << Self::OFFSET);
        if len == 4 {
            p.copy_from_slice(&num_new.to_ne_bytes());
        } else {
            p.copy_from_slice(&num_new.to_le_bytes()[..len])
        }
    }

    fn get_across(arr: &[u8]) -> u32 {
        let (num_start, num_end) = Self::get_across_helper(arr);
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
}

impl<const WIDTH: usize, const ACC: usize> SetGet for BitsU64<WIDTH, ACC> {
    type Target = u64;

    const ACROSS: bool = Self::RANGE_BITS > u64::BITS;
    const GET: fn(&[u8]) -> u64 = if Self::ACROSS { Self::get_across } else { Self::get_no_across };
    const LIMIT: u64 = (u64::MAX >> (64 - WIDTH)) << Self::OFFSET;
    const RANGE_ACROSS: Range = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
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
        let num_new = num_old & !Self::LIMIT | (num << Self::OFFSET);
        if len == 8 {
            p.copy_from_slice(&num_new.to_ne_bytes());
        } else {
            p.copy_from_slice(&num_new.to_le_bytes()[..len])
        }
    }

    fn get_across(arr: &[u8]) -> u64 {
        let (num_start, num_end) = Self::get_across_helper(arr);
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
