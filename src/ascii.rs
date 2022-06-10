//! 7-bit ASCII decoder
use somen::prelude::*;

/// An ASCII encoded character.
pub fn ascii_char<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u8> + ?Sized + 'a,
{
    is_some(|c| if c <= 0x7F { Some(c as char) } else { None }).expect("ascii character")
}

/// An ASCII encoded string.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn ascii_string<'a, I>() -> impl Parser<I, Output = String>
where
    I: Input<Ok = u8> + ?Sized + 'a,
{
    ascii_char().repeat(..).collect().expect("ascii string")
}
