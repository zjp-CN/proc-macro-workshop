const ERR: &str = "slice 转化 array 时失败";
type Range = std::ops::RangeInclusive<usize>;

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

pub mod u16;
pub mod u32;
pub mod u64;
pub mod u8;

#[cfg(test)]
mod tests;
