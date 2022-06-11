//! A byte string decoder for [somen](https://docs.rs/somen).
#![no_std]
#![doc(test(attr(warn(warnings))))]

mod ascii;
mod utf16;
mod utf32;

pub use ascii::ascii;
pub use utf16::{utf16, utf16be, utf16le};
pub use utf32::{utf32, utf32be, utf32le};
