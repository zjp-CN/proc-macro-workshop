use super::{Basic, Range, SetGet, ERR};

pub struct BitsU8<const WIDTH: usize, const ACC: usize, const SIZE: usize>;

impl<const WIDTH: usize, const ACC: usize, const SIZE: usize> Basic<WIDTH, ACC>
    for BitsU8<WIDTH, ACC, SIZE>
{
}

impl<const WIDTH: usize, const ACC: usize, const SIZE: usize> SetGet for BitsU8<WIDTH, ACC, SIZE> {
    type Target = u8;

    const ACROSS: bool = Self::RANGE_BITS > u8::BITS;
    const GET: fn(&[u8]) -> u8 = if Self::ACROSS {
        Self::get_across
    } else {
        Self::get_no_across
    };
    const LIMIT: u8 = (u8::MAX >> (8 - WIDTH)) << Self::OFFSET;
    const RANGE_ACROSS: Range = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
    const RANGE_ALT: Range = Self::RANGE_RHS..=Self::RANGE_RHS;
    const RANGE_RHS2: Range = Self::RANGE_RHS..=Self::RANGE_RHS;
    const SET: fn(&mut [u8], u8) = if Self::ACROSS {
        Self::set_across
    } else {
        Self::set_no_across
    };
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
