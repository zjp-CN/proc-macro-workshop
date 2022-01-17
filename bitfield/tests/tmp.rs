#![allow(unused)]

fn main() {
    // let bits = 35;
    // let f = |num: u8, align: u8| (num + (align - 1)) & !(align - 1);
    // let g = |bits: u8| (bits >> 3) + ((bits % 8) != 0) as u8;
    // dbg!(bits, bits / 8, bits % 8, ((bits % 8) != 0) as u8, g(bits), f(g(bits), 2) << 3);
    // let d = 0b1_110_0000u8;
    // let r = 0b110;
    // dbg!(d, r, (d >> (8 - 4)) & 0b111, (d >> (8 - 1)) & 0b1);
    //
    // trait A {
    //     type T: Sized;
    // }
    // impl A for u8 {
    //     type T = u8;
    // }
    // dbg!(0 as <u8 as A>::T);
    //
    // let n = 2;
    // println!("{0:b}\t{}", u8::MAX >> (8 - n));
    //
    // use bitfield::Specifier;
    // dbg!(bitfield::B1::MAX);
    // println!("{:b}", bitfield::B17::MAX);
    // println!("{:b}", bitfield::B18::MAX);
    //
    // dbg!([1, 2, 3].iter()
    //               .scan(0, |acc, n| Some({
    //                   *acc += *n;
    //                   *acc
    //               }))
    //               .collect::<Vec<_>>());

    // [1,3,4,24]
    let mut a = [111u8, 222, 244, 96];
    a.iter().for_each(|b| print!("{:08b} ", b));
    // 01101111 11011110 11110100 01100000
    // 1: 0b0
    // 3: 0b110
    // 4: 1111
    // 24: 11011110 11110100 01100000

    // 取第一个字段：
    // 1. 找 u8s：累加的 bits、自身长度
    // 2. 找开头：累加的 bits 减去 8 的倍数
    // 3. 找结尾：8 的倍数 - (累加的 bits + 自身长度)
    println!("\n{:08b}", 0b11010u8.reverse_bits());

    println!("{:0b}", a[0] >> (8 - 1) & 0b1);
    println!("{:0b}", a[0] >> (8 - 4) & 0b111);
    println!("{:0b}", a[0] >> (8 - 8) & 0b1111);
    // let b = u16::from_le_bytes([0b101, 0b01]);
    a.reverse();
    let b = u32::from_ne_bytes(a);
    println!("{:032b}", b);
    let p = u32::MAX >> (32 - 24); // 固定的
    println!("{:032b}", p);
    println!("{:032b}", b & p);

    let c = 0u8;
    println!("   原数 {:08b}", c);
    println!("1: 清零 {:08b}", c & !0b1);
    println!("3: 清零 {:08b}", c & !(0b111 << 1));
    println!("   设置 {:08b}", c & !(0b111 << 1) | (0b110 << 1));
    println!("4: 清零 {:08b}", c & !(0b1111 << 4));
    println!("   设置 {:08b}", c & !(0b1111 << 4) | (0b1111 << 4));

    let c = 0u32;
    println!("   原数 {:032b}", c);
    println!("1: 清零 {:032b}", c & !0b1);
    println!("3: 清零 {:032b}", c & !(0b111 << 1));
    println!("   设置 {:032b}", c & !(0b111 << 1) | (0b110 << 1));
    println!("4: 清零 {:032b}", c & !(0b1111 << 4));
    println!("   设置 {:032b}", c & !(0b1111 << 4) | (0b1111 << 4));
    println!("24:清零 {:032b}", c & !(0b1111 << 8));
    println!("   设置 {:032b}", c & !(u32::MAX >> (32 - 24) << 8) | (0b11011110_11110100_01100000 << 8));

    let mut c = 1u64;
    println!("{:064b}\nne: {:?}\nle: {:?}\nbe: {:?}\n[{}]",
             c,
             c.to_ne_bytes(),
             c.to_le_bytes(),
             c.to_be_bytes(),
             c.to_be_bytes()
              .iter()
              .map(|b| format!("{:08b} ", b))
              .collect::<Vec<_>>()
              .join(" "));

    c = c & !0b1;
    c = c & !(0b111 << 1) | (0b110 << 1);
    c = c & !(0b1111 << 4) | (0b1111 << 4);
    c = c & !(u64::MAX >> (32 - 24) << 8) | (0b11011110_11110100_01100000 << 8);
    println!("\n\n\n   原数 {:032b}", c);
    let a = c.to_ne_bytes();
    let a = a.iter().map(|b| format!("{:08b} ", b)).collect::<Vec<_>>().join(" ");
    println!("       [{}]", a);
    let a = c.to_be_bytes();
    let a = a.iter().map(|b| format!("{:08b} ", b)).collect::<Vec<_>>().join(" ");
    println!("       [{}]", a);
    let r = c & 0b1;
    println!("\n1: 取数 {:032b}\t{}", r, r as u8);
    let r = c & (0b111 << 1);
    println!("3: 取数 {:032b}\t{}", r, r as u8);
    let r = c & (0b1111 << 4);
    println!("4: 取数 {:032b}\t{}", r, r as u8);
    let r = c & (u64::MAX >> (32 - 24) << 8);
    println!("24:取数 {:032b}\t{}", r, r as u32);

    // let mut c = [0; 8]; // u64

    // c = c & !0b1;
    // c = c & !(0b111 << 1) | (0b110 << 1);
    // c = c & !(0b1111 << 4) | (0b1111 << 4);
    // c = c & !(u64::MAX >> (32 - 24) << 8) | (0b11011110_11110100_01100000 << 8);
    // println!("\n\n\n   原数 {:032b}", c);
    // let a = c.to_ne_bytes();
    // let a = a.iter().map(|b| format!("{:08b} ", b)).collect::<Vec<_>>().join(" ");
    // println!("       [{}]", a);
    // let a = c.to_be_bytes();
    // let a = a.iter().map(|b| format!("{:08b} ", b)).collect::<Vec<_>>().join(" ");
    // println!("       [{}]", a);
    // let r = c & 0b1;
    // println!("\n1: 取数 {:032b}\t{}", r, r as u8);
    // let r = c & (0b111 << 1);
    // println!("3: 取数 {:032b}\t{}", r, r as u8);
    // let r = c & (0b1111 << 4);
    // println!("4: 取数 {:032b}\t{}", r, r as u8);
    // let r = c & (u64::MAX >> (32 - 24) << 8);
    // println!("24:取数 {:032b}\t{}", r, r as u32);
}

struct Bits {
    width: u8,
    acc:   u8,
}

impl Bits {
    fn arr_pos(&self) -> std::ops::RangeInclusive<usize> {
        let lhs = self.acc / 8;
        let rhs = (self.width + self.acc - 1) / 8;
        (lhs as usize)..=(rhs as usize)
    }

    fn offset(&self) -> usize { (self.acc % 8) as usize }

    fn get_u8(&self, arr: &[u8]) -> u8 {
        let num = u8::from_ne_bytes(self.fetch(arr));
        let offset = self.offset();
        (num & (u8::MAX >> (8 - self.width) << offset)) >> offset
    }

    fn get_u16(&self, arr: &[u8]) -> u16 {
        let num = u16::from_ne_bytes(self.fetch(arr));
        let offset = self.offset();
        (num & (u16::MAX >> (16 - self.width) << offset)) >> offset
    }

    fn get_u32(&self, arr: &[u8]) -> u32 {
        let num = u32::from_ne_bytes(self.fetch(arr));
        let offset = self.offset();
        (num & (u32::MAX >> (32 - self.width) << offset)) >> offset
    }

    fn get_u64(&self, arr: &[u8]) -> u64 {
        let num = u64::from_ne_bytes(self.fetch(arr));
        let offset = self.offset();
        (num & (u64::MAX >> (64 - self.width) << offset)) >> offset
    }

    fn fetch<'a, T: TryFrom<&'a [u8]>>(&self, arr: &'a [u8]) -> T
        where <T as TryFrom<&'a [u8]>>::Error: std::fmt::Debug {
        T::try_from(&arr[self.arr_pos()]).expect("slice 转化 array 时失败")
    }

    // fn fetch2<'a, T: TryFrom<&'a [u8]>>(&self, arr: &'a mut [u8]) -> (T, &'a mut [u8])
    //     where <T as TryFrom<&'a [u8]>>::Error: std::fmt::Debug {
    //     let range = self.arr_pos();
    //     let t = T::try_from(&arr[range.clone]).expect("slice 转化 array 时失败");
    //     (t, &mut arr[range])
    // }

    fn set_u8(&self, arr: &mut [u8], num: u8) {
        let p = &mut arr[self.arr_pos()];
        let num_old = u8::from_ne_bytes(p.try_into().expect("slice 转化 array 时失败"));
        let num_new = num_old & !((u8::MAX >> (8 - self.width)) << self.offset()) | (num << self.offset());
        p.copy_from_slice(&num_new.to_ne_bytes());
    }

    fn set_u16(&self, arr: &mut [u8], num: u16) {
        let p = &mut arr[self.arr_pos()];
        let num_old = u16::from_ne_bytes(p.try_into().expect("slice 转化 array 时失败"));
        let num_new = num_old & !((u16::MAX >> (16 - self.width)) << self.offset()) | (num << self.offset());
        p.copy_from_slice(&num_new.to_ne_bytes());
    }

    fn set_u32(&self, arr: &mut [u8], num: u32) {
        let p = &mut arr[self.arr_pos()];
        let num_old = u32::from_ne_bytes(p.try_into().expect("slice 转化 array 时失败"));
        let num_new = num_old & !((u32::MAX >> (32 - self.width)) << self.offset()) | (num << self.offset());
        p.copy_from_slice(&num_new.to_ne_bytes());
    }

    fn set_u64(&self, arr: &mut [u8], num: u64) {
        let p = &mut arr[self.arr_pos()];
        let num_old = u64::from_ne_bytes(p.try_into().expect("slice 转化 array 时失败"));
        let num_new = num_old & !((u64::MAX >> (64 - self.width)) << self.offset()) | (num << self.offset());
        p.copy_from_slice(&num_new.to_ne_bytes());
    }
}

#[test]
fn test_const_struct_range() {
    fn dbg(arr: &[u8]) {
        println!("{}", arr.iter().map(|a| format!("{:08b}", a)).collect::<Vec<_>>().join("_"))
    }

    struct BitsPos<const WIDTH: usize, const ACC: usize>;

    impl<const WIDTH: usize, const ACC: usize> BitsPos<WIDTH, ACC> {
        const ACROSS_U08: bool = Self::RANGE_BITS > u8::BITS;
        const ACROSS_U16: bool = Self::RANGE_BITS > u16::BITS;
        const ACROSS_U32: bool = Self::RANGE_BITS > u32::BITS;
        const ACROSS_U64: bool = Self::RANGE_BITS > u64::BITS;
        const LIMIT_U08: u8 = (u8::MAX >> (8 - WIDTH)) << Self::OFFSET;
        const LIMIT_U16: u16 = (u16::MAX >> (16 - WIDTH)) << Self::OFFSET;
        const LIMIT_U32: u32 = (u32::MAX >> (32 - WIDTH)) << Self::OFFSET;
        const LIMIT_U64: u64 = (u64::MAX >> (64 - WIDTH)) << Self::OFFSET;
        const OFFSET: usize = ACC % 8;
        const OFFSET_END: usize = (ACC + WIDTH) % 8;
        const RANGE: std::ops::RangeInclusive<usize> = Self::RANGE_LHS..=Self::RANGE_RHS;
        const RANGE_BITS: u32 = (Self::RANGE_LEN * 8) as u32;
        const RANGE_LEN: usize = Self::RANGE_RHS - Self::RANGE_LHS + 1;
        const RANGE_LHS: usize = ACC / 8;
        const RANGE_RHS: usize = (WIDTH + ACC - 1) / 8;

        fn get_u8(arr: &[u8]) -> u8 {
            if Self::ACROSS_U08 {
                let r = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
                let num_start = u8::from_ne_bytes(arr[r].try_into().unwrap());
                let num = (num_start & Self::LIMIT_U08) >> Self::OFFSET;

                let num_end = u8::from_ne_bytes(arr[Self::RANGE_RHS..=Self::RANGE_RHS].try_into().unwrap());
                let num_end = (num_end & (u8::MAX >> (8 - Self::OFFSET_END))) << (WIDTH - Self::OFFSET_END);
                num_end | num
            } else {
                let num = u8::from_ne_bytes(Self::fetch(arr));
                (num & Self::LIMIT_U08) >> Self::OFFSET
            }
        }

        fn get_u16(arr: &[u8]) -> u16 {
            if Self::ACROSS_U16 {
                let r = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
                let num_start = u16::from_ne_bytes(arr[r].try_into().unwrap());
                let num = (num_start & Self::LIMIT_U16) >> Self::OFFSET;

                let num_end =
                    u8::from_ne_bytes(arr[Self::RANGE_RHS..=Self::RANGE_RHS].try_into().unwrap()) as u16;
                let num_end = (num_end & (u16::MAX >> (16 - Self::OFFSET_END))) << (WIDTH - Self::OFFSET_END);
                num_end | num
            } else {
                let num = u16::from_ne_bytes(Self::fetch(arr));
                (num & Self::LIMIT_U16) >> Self::OFFSET
            }
        }

        fn get_u32(arr: &[u8]) -> u32 {
            if Self::ACROSS_U32 {
                let r = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
                let num_start = u32::from_ne_bytes(arr[r].try_into().unwrap());
                let num = (num_start & Self::LIMIT_U32) >> Self::OFFSET;

                let num_end =
                    u8::from_ne_bytes(arr[Self::RANGE_RHS..=Self::RANGE_RHS].try_into().unwrap()) as u32;
                let num_end = (num_end & (u32::MAX >> (32 - Self::OFFSET_END))) << (WIDTH - Self::OFFSET_END);
                num_end | num
            } else {
                let r =
                    if Self::RANGE_LEN == 4 { Self::RANGE } else { Self::RANGE_LHS..=Self::RANGE_LHS + 3 };
                let num = u32::from_ne_bytes(arr[r].try_into().expect("slice 转化 array 时失败"));
                (num & Self::LIMIT_U32) >> Self::OFFSET
            }
        }

        fn get_u64(arr: &[u8]) -> u64 {
            if Self::ACROSS_U64 {
                let r = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
                let num_start = u64::from_ne_bytes(arr[r].try_into().unwrap());
                let num = (num_start & Self::LIMIT_U64) >> Self::OFFSET;

                let num_end =
                    u8::from_ne_bytes(arr[Self::RANGE_RHS..=Self::RANGE_RHS].try_into().unwrap()) as u64;
                let num_end = (num_end & (u64::MAX >> (64 - Self::OFFSET_END))) << (WIDTH - Self::OFFSET_END);
                num_end | num
            } else {
                let r =
                    if Self::RANGE_LEN == 8 { Self::RANGE } else { Self::RANGE_LHS..=Self::RANGE_LHS + 7 };
                let num = u64::from_ne_bytes(arr[r].try_into().expect("slice 转化 array 时失败"));
                (num & Self::LIMIT_U64) >> Self::OFFSET
            }
        }

        fn fetch<'a, T: TryFrom<&'a [u8]>>(arr: &'a [u8]) -> T
            where <T as TryFrom<&'a [u8]>>::Error: std::fmt::Debug {
            T::try_from(&arr[Self::RANGE]).expect("slice 转化 array 时失败")
        }

        fn set_u8(arr: &mut [u8], num: u8) {
            if Self::ACROSS_U08 {
                let r = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
                let p = &mut arr[r];
                let num_old = u8::from_ne_bytes(p.try_into().unwrap());
                let num_new = num_old & !Self::LIMIT_U08 | (num << Self::OFFSET);
                p.copy_from_slice(&num_new.to_ne_bytes());

                let num_end = (num >> (8 - Self::OFFSET)) as u8;
                let p = &mut arr[Self::RANGE_RHS..=Self::RANGE_RHS];
                let num_old = u8::from_ne_bytes(p.try_into().unwrap());
                let num_new = num_old & !(u8::MAX >> (8 - Self::OFFSET_END)) | num_end;
                p.copy_from_slice(&num_new.to_ne_bytes());
            } else {
                let p = &mut arr[Self::RANGE];
                let num_old = u8::from_ne_bytes(p.try_into().expect("slice 转化 array 时失败"));
                let num_new = num_old & !Self::LIMIT_U08 | (num << Self::OFFSET);
                p.copy_from_slice(&num_new.to_ne_bytes());
            }
        }

        fn set_u16(arr: &mut [u8], num: u16) {
            if Self::ACROSS_U16 {
                let r = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
                let p = &mut arr[r];
                let num_old = u16::from_ne_bytes(p.try_into().unwrap());
                let num_new = num_old & !Self::LIMIT_U16 | (num << Self::OFFSET);
                p.copy_from_slice(&num_new.to_ne_bytes());

                let num_end = (num >> (16 - Self::OFFSET)) as u8;
                let p = &mut arr[Self::RANGE_RHS..=Self::RANGE_RHS];
                let num_old = u8::from_ne_bytes(p.try_into().unwrap());
                let num_new = num_old & !(u8::MAX >> (8 - Self::OFFSET_END)) | num_end;
                p.copy_from_slice(&num_new.to_ne_bytes());
            } else {
                let p = &mut arr[Self::RANGE];
                let num_old = u16::from_ne_bytes(p.try_into().expect("slice 转化 array 时失败"));
                let num_new = num_old & !Self::LIMIT_U16 | (num << Self::OFFSET);
                p.copy_from_slice(&num_new.to_ne_bytes());
            }
        }

        fn set_u32(arr: &mut [u8], num: u32) {
            if Self::ACROSS_U32 {
                let r = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
                let p = &mut arr[r];
                let num_old = u32::from_ne_bytes(p.try_into().unwrap());
                let num_new = num_old & !Self::LIMIT_U32 | (num << Self::OFFSET);
                p.copy_from_slice(&num_new.to_ne_bytes());

                let num_end = (num >> (32 - Self::OFFSET)) as u8;
                let p = &mut arr[Self::RANGE_RHS..=Self::RANGE_RHS];
                let num_old = u8::from_ne_bytes(p.try_into().unwrap());
                let num_new = num_old & !(u8::MAX >> (8 - Self::OFFSET_END)) | num_end;
                p.copy_from_slice(&num_new.to_ne_bytes());
            } else {
                let r =
                    if Self::RANGE_LEN == 4 { Self::RANGE } else { Self::RANGE_LHS..=Self::RANGE_LHS + 3 };
                let p = &mut arr[r];
                let num_old = u32::from_ne_bytes(p.try_into().expect("slice 转化 array 时失败"));
                let num_new = num_old & !Self::LIMIT_U32 | (num << Self::OFFSET);
                p.copy_from_slice(&num_new.to_ne_bytes());
            }
        }

        fn set_u64(arr: &mut [u8], num: u64) {
            if Self::ACROSS_U64 {
                let r = Self::RANGE_LHS..=(Self::RANGE_RHS - 1);
                let p = &mut arr[r];
                let num_old = u64::from_ne_bytes(p.try_into().unwrap());
                let num_new = num_old & !Self::LIMIT_U64 | (num << Self::OFFSET);
                p.copy_from_slice(&num_new.to_ne_bytes());

                let num_end = (num >> (64 - Self::OFFSET)) as u8;
                let p = &mut arr[Self::RANGE_RHS..=Self::RANGE_RHS];
                let num_old = u8::from_ne_bytes(p.try_into().unwrap());
                let num_new = num_old & !(u8::MAX >> (8 - Self::OFFSET_END)) | num_end;
                p.copy_from_slice(&num_new.to_ne_bytes());
            } else {
                let r =
                    if Self::RANGE_LEN == 8 { Self::RANGE } else { Self::RANGE_LHS..=Self::RANGE_LHS + 7 };
                let p = &mut arr[r];
                let num_old = u64::from_ne_bytes(p.try_into().expect("slice 转化 array 时失败"));
                let num_new = num_old & !Self::LIMIT_U64 | (num << Self::OFFSET);
                p.copy_from_slice(&num_new.to_ne_bytes());
            }
        }
    }

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

    let mut arr = [0; (56 + 18 + 26 + 14 + 8) / 8 + (56 + 18 + 26 + 14 + 8) % 8 as usize];

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
fn test_1_3_4_24() {
    let bits: Bits = Bits { width: 1, acc: 0 };
    assert_eq!(bits.arr_pos(), 0..=0);
    assert_eq!(bits.offset(), 0);

    let bits: Bits = Bits { width: 3, acc: 1 };
    assert_eq!(bits.arr_pos(), 0..=0);
    assert_eq!(bits.offset(), 1);

    let bits: Bits = Bits { width: 4, acc: 4 };
    assert_eq!(bits.arr_pos(), 0..=0);
    assert_eq!(bits.offset(), 4);

    let bits: Bits = Bits { width: 24, acc: 8 };
    assert_eq!(bits.arr_pos(), 1..=3);
    assert_eq!(bits.offset(), 0);
}

#[test]
fn test_8_6_9_25() {
    let bits_a: Bits = Bits { width: 8, acc: 0 };
    assert_eq!(bits_a.arr_pos(), 0..=0);
    assert_eq!(bits_a.offset(), 0);

    let bits_b: Bits = Bits { width: 6, acc: 8 };
    assert_eq!(bits_b.arr_pos(), 1..=1);
    assert_eq!(bits_b.offset(), 0);

    let bits_c: Bits = Bits { width: 9, acc: 14 };
    assert_eq!(bits_c.arr_pos(), 1..=2);
    assert_eq!(bits_c.offset(), 6);

    let bits_d: Bits = Bits { width: 25, acc: 23 };
    assert_eq!(bits_d.arr_pos(), 2..=5);
    assert_eq!(bits_d.offset(), 7);

    let mut arr = [0u8; 6];
    // let arr_idx = |(a, b): (usize, usize)| a..=b;
    let fmt = |s: &[u8]| s.iter().map(|d| format!("{:08b}", d)).collect::<Vec<_>>().join("_");

    // ******************** const ********************
    const A: u8 = 0b1101_0011;
    const B: u8 = 0b10_0010;
    const C: u16 = 0b1_0010_0001;
    const D: u32 = 0b1_1010_0101_1100_0011_0000_1111;

    // ******************** set ********************
    let a = &mut arr[bits_a.arr_pos()];
    let num_a = u8::from_ne_bytes(a.try_into().unwrap());
    // 8: 设置 0b1101_0011
    let num_a = num_a | A;
    a.copy_from_slice(&num_a.to_ne_bytes());
    assert_eq!(fmt(a), "11010011");
    assert_eq!(format!("{:?}", arr), "[211, 0, 0, 0, 0, 0]");
    assert_eq!(fmt(&arr), "11010011_00000000_00000000_00000000_00000000_00000000");

    let b = &mut arr[bits_b.arr_pos()];
    let num_b = u8::from_ne_bytes(b.try_into().unwrap());
    // 6: 设置 0b10_0010
    let num_b = num_b | B;
    b.copy_from_slice(&num_b.to_ne_bytes());
    assert_eq!(fmt(b), "00100010");
    assert_eq!(format!("{:?}", arr), "[211, 34, 0, 0, 0, 0]");
    assert_eq!(fmt(&arr), "11010011_00100010_00000000_00000000_00000000_00000000");

    let c = &mut arr[bits_c.arr_pos()];
    let num_c = u16::from_ne_bytes(c.try_into().unwrap());
    // 9: 设置 0b1_0010_0001
    let num_c = num_c | (C << 6);
    c.copy_from_slice(&num_c.to_ne_bytes());
    assert_eq!(fmt(c), "01100010_01001000");
    assert_eq!(format!("{:?}", arr), "[211, 98, 72, 0, 0, 0]");
    assert_eq!(fmt(&arr), "11010011_01100010_01001000_00000000_00000000_00000000");

    let d = &mut arr[bits_d.arr_pos()];
    let num_d = u32::from_ne_bytes(d.try_into().unwrap());
    // 25: 设置 0b1_1010_0101_1100_0011_0000_1111
    let num_d = num_d | (D << 7);
    d.copy_from_slice(&num_d.to_ne_bytes());
    assert_eq!(fmt(d), "11001000_10000111_11100001_11010010");
    assert_eq!(format!("{:?}", arr), "[211, 98, 200, 135, 225, 210]");
    assert_eq!(fmt(&arr), "11010011_01100010_11001000_10000111_11100001_11010010");

    // ******************** get ********************
    let a = &mut arr[bits_a.arr_pos()];
    let num_a = u8::from_ne_bytes(a.try_into().unwrap());
    assert_eq!(num_a & u8::MAX, A);

    let b = &mut arr[bits_b.arr_pos()];
    let num_b = u8::from_ne_bytes(b.try_into().unwrap());
    assert_eq!(num_b & (u8::MAX >> (8 - 6)), B);

    // ① : u16::MAX >> (当前类型位数 - 宽度)：该字段的最大数值
    // ② : ①  << 前几位        ：这个“前几位”是当前类型的前几位，直接由 acc % 8 得到
    // 15 13 2 => u16 u16 u8 30bits [8+7, 1+8+4, 2, ..]
    // ③ : num & ②             ：取对应的位
    // ④ : ③  >> 前几位        ：消除前几位，从而得到结果
    let c = &mut arr[bits_c.arr_pos()];
    let num_c = u16::from_ne_bytes(c.try_into().unwrap());
    // 11111111_11111111
    //        1_11111111
    //  1_11111111
    assert_eq!((num_c & (u16::MAX >> (16 - 9) << 6)) >> 6, C);
    // println!("{:08b} {:08b} {:08b} {:08b}",
    //          (u8::MAX >> (8 - 3) << 1),
    //          0b1000_1110,
    //          0b1000_1110 & (u8::MAX >> (8 - 3) << 1),
    //          (0b1000_1110 & (u8::MAX >> (8 - 3) << 1)) >> 1);
    // assert_eq!(0b111, (0b1000_1110 & (u8::MAX >> (8 - 3) << 1)) >> 1);

    let d = &mut arr[bits_d.arr_pos()];
    let num_d = u32::from_ne_bytes(d.try_into().unwrap());
    assert_eq!((num_d & (u32::MAX >> (32 - 25) << 7)) >> 7, D);

    // ******************** get ********************
    const AA: u8 = 0b1101_0011;
    const BB: u8 = 0b10_0010;
    const CC: u16 = 0b1_0010_0001;
    const DD: u32 = 0b1_1010_0101_1100_0011_0000_1111;

    bits_a.set_u8(&mut arr, AA);
    bits_b.set_u8(&mut arr, BB);
    bits_c.set_u16(&mut arr, CC);
    bits_d.set_u32(&mut arr, DD);

    // ******************** get ********************
    assert_eq!(bits_a.get_u8(&arr), AA);
    assert_eq!(bits_b.get_u8(&arr), BB);
    assert_eq!(bits_c.get_u16(&arr), CC);
    assert_eq!(bits_d.get_u32(&arr), DD);

    bits_a.set_u8(&mut arr, 0);
    bits_b.set_u8(&mut arr, 0);
    bits_c.set_u16(&mut arr, 0);
    bits_d.set_u32(&mut arr, 0);
    assert_eq!(arr, [0; 6]);
}
