//! A byte string decoder for somen.
#![no_std]
#![doc(test(attr(warn(warnings))))]

mod ascii;
mod utf32;

pub use ascii::ascii;
pub use utf32::{utf32, utf32be, utf32le};
