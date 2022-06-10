//! 7-bit ASCII decoder
use alloc::string::String;
use somen::prelude::*;

/// An ASCII encoded character.
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::ascii::ascii_char;
/// use somen::prelude::*;
///
/// let mut parser = ascii_char();
/// let mut stream = stream::from_slice(b"A$\n\x00\xA2");
///
/// assert_eq!(parser.parse(&mut stream).await, Ok('A'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('$'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('\n'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('\x00'));
/// assert!(ascii_char().parse(&mut stream).await.is_err());
/// # });
/// ```
pub fn ascii_char<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u8> + ?Sized + 'a,
{
    is_some(|c| if c <= 0x7F { Some(c as char) } else { None }).expect("ascii character")
}

/// An ASCII encoded string.
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::ascii::ascii_string;
/// use somen::prelude::*;
///
/// let mut parser = ascii_string();
/// let mut stream = stream::from_slice(b"A$\n\x00\xA2");
///
/// assert_eq!(parser.parse(&mut stream).await, Ok(String::from("A$\n\x00")));
///
/// // Invalid inputs are remained.
/// assert_eq!(any().parse(&mut stream).await, Ok(b'\xA2'));
/// # });
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn ascii_string<'a, I>() -> impl Parser<I, Output = String>
where
    I: Input<Ok = u8> + ?Sized + 'a,
{
    ascii_char().repeat(..).collect().expect("ascii string")
}
