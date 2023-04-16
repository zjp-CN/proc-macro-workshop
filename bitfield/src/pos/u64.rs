use super::{Basic, Range, SetGet, ERR};

pub struct BitsU64<const WIDTH: usize, const ACC: usize, const SIZE: usize>;

impl<const WIDTH: usize, const ACC: usize, const SIZE: usize> Basic<WIDTH, ACC>
    for BitsU64<WIDTH, ACC, SIZE>
{
}

impl<const WIDTH: usize, const ACC: usize, const SIZE: usize> BitsU64<WIDTH, ACC, SIZE> {
    const HOLD: bool = !(Self::HOLD_LEN < 8);
    const HOLD_LEN: usize = SIZE - Self::RANGE_LHS;
    const RANGE_SIZED: Range = Self::RANGE_LHS..=SIZE - 1;
    const SET2: fn(&mut [u8], u64) = if Self::HOLD {
        Self::can_hold
    } else {
        Self::not_hold
    };
    const U64: fn(&[u8]) -> u64 = if Self::HOLD {
        Self::u64_can_hold
    } else {
        Self::u64_not_hold
    };

    fn can_hold(arr: &mut [u8], num: u64) {
        let num_old = Self::u64_can_hold(arr);
        let p = &mut arr[Self::RANGE_ALT];
        let num_new = num_old & !Self::LIMIT | (num << Self::OFFSET);
        p.copy_from_slice(&num_new.to_ne_bytes());
    }

    fn not_hold(arr: &mut [u8], num: u64) {
        let num_old = Self::u64_not_hold(arr);
        let p = &mut arr[Self::RANGE_SIZED];
        let num_new = num_old & !Self::LIMIT | (num << Self::OFFSET);
        p.copy_from_slice(&num_new.to_le_bytes()[..Self::HOLD_LEN]);
    }

    fn u64_can_hold(arr: &[u8]) -> u64 {
        u64::from_ne_bytes(arr[Self::RANGE_ALT].try_into().expect(ERR))
    }

    fn u64_not_hold(arr: &[u8]) -> u64 {
        let mut tmp = [0; 8];
        tmp[..Self::HOLD_LEN].copy_from_slice(&arr[Self::RANGE_SIZED]);
        u64::from_le_bytes(tmp)
    }
}

impl<const WIDTH: usize, const ACC: usize, const SIZE: usize> SetGet for BitsU64<WIDTH, ACC, SIZE> {
    type Target = u64;

    const ACROSS: bool = Self::RANGE_BITS > u64::BITS;
    const GET: fn(&[u8]) -> u64 = if Self::ACROSS {
        Self::get_across
    } else {
        Self::get_no_across
    };
    const LIMIT: u64 = (u64::MAX >> (64 - WIDTH)) << Self::OFFSET;
    const RANGE_ACROSS: Range = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
    const RANGE_ALT: Range = if Self::RANGE_LEN == 8 {
        Self::RANGE
    } else {
        Self::RANGE_LHS..=Self::RANGE_LHS + 7
    };
    const RANGE_RHS2: Range = Self::RANGE_RHS..=Self::RANGE_RHS;
    const SET: fn(&mut [u8], u64) = if Self::ACROSS {
        Self::set_across
    } else {
        Self::set_no_across
    };
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
        Self::SET2(arr, num)
    }

    fn get_across(arr: &[u8]) -> u64 {
        let (num_start, num_end) = Self::get_across_helper(arr);
        let num = (u64::from_ne_bytes(num_start) & Self::LIMIT) >> Self::OFFSET;
        let num_end =
            ((num_end as u64) & (u64::MAX >> (64 - Self::OFFSET_END_))) << Self::OFFSET_END;
        num | num_end
    }

    fn get_no_across(arr: &[u8]) -> u64 {
        let num = Self::U64(arr);
        (num & Self::LIMIT) >> Self::OFFSET
    }
}
