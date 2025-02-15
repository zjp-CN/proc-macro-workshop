// Crates that have the "proc-macro" crate type are only allowed to export
// procedural macros. So we cannot have one crate that defines procedural macros
// alongside other types of public APIs like traits and structs.
//
// For this project we are going to need a #[bitfield] macro but also a trait
// and some structs. We solve this by defining the trait and structs in this
// crate, defining the attribute macro in a separate bitfield-impl crate, and
// then re-exporting the macro from this crate so that users only have one crate
// that they need to import.
//
// From the perspective of a user of this crate, they get all the necessary APIs
// (macro, trait, struct) through the one bitfield crate.
pub use bitfield_impl::bitfield;
pub use bitfield_impl::BitfieldSpecifier;

pub trait Specifier {
    const BITS: usize;

    type T: Sized;

    fn set<const ACC: usize, const SIZE: usize>(arr: &mut [u8], num: <Self as Specifier>::T);
    fn get<const ACC: usize, const SIZE: usize>(arr: &[u8]) -> <Self as Specifier>::T;
}

bitfield_impl::gen! {}

impl Specifier for bool {
    type T = bool;

    const BITS: usize = 1;

    fn set<const ACC: usize, const SIZE: usize>(arr: &mut [u8], num: bool) {
        B1::set::<ACC, SIZE>(arr, num as u8)
    }

    fn get<const ACC: usize, const SIZE: usize>(arr: &[u8]) -> bool {
        B1::get::<ACC, SIZE>(arr).eq(&1)
    }
}

mod pos;
pub use pos::{u16::BitsU16, u32::BitsU32, u64::BitsU64, u8::BitsU8, Basic, SetGet};
