//! UTF-32 decoder
use alloc::string::String;
use somen::prelude::*;

// A base decoder
#[inline]
fn decode_utf32(c: u32) -> Option<char> {
    char::from_u32(c)
}

/// A 32-bit encoded UTF-32 character.
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf32::utf32_char;
/// use somen::prelude::*;
///
/// let mut parser = utf32_char();
/// let mut stream = stream::from_slice(&[0x41, 0xC5, 0x3042, 0x1F4AF, 0x110000]);
///
/// assert_eq!(parser.parse(&mut stream).await, Ok('A'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('Å'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('あ'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('💯'));
/// assert!(parser.parse(&mut stream).await.is_err());
/// # });
/// ```
pub fn utf32_char<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u32> + ?Sized + 'a,
{
    any().try_map(|b| decode_utf32(b).ok_or("UTF-32 character"))
}

/// A 32-bit encoded UTF-32 string.
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf32::utf32_string;
/// use somen::prelude::*;
///
/// let mut parser = utf32_string();
///
/// let mut ok = stream::from_slice(&[0x41, 0xC5, 0x3042, 0x1F4AF]);
/// assert_eq!(parser.parse(&mut ok).await, Ok(String::from("AÅあ💯")));
///
/// let mut err = stream::from_slice(&[0x110000]);
/// assert!(parser.parse(&mut err).await.is_err());
/// # });
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn utf32_string<'a, I>() -> impl Parser<I, Output = String>
where
    I: Input<Ok = u32> + ?Sized + 'a,
{
    utf32_char().repeat(..).collect().expect("UTF-32 string")
}

/// A byte encoded UTF-32 character (big-endian).
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf32::utf32be_char;
/// use somen::prelude::*;
///
/// let mut parser = utf32be_char();
/// let mut stream = stream::from_slice(
///     b"\x00\x00\x00\x41\x00\x00\x00\xC5\x00\x00\x30\x42\x00\x01\xF4\xAF\x00\x11\x00\x00"
/// );
///
/// assert_eq!(parser.parse(&mut stream).await, Ok('A'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('Å'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('あ'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('💯'));
/// assert!(parser.parse(&mut stream).await.is_err());
/// # });
/// ```
pub fn utf32be_char<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u8> + ?Sized + 'a,
{
    any()
        .times(4)
        .fill::<4>(0)
        .try_map(|b| decode_utf32(u32::from_be_bytes(b.unwrap())).ok_or("UTF-32BE character"))
}

/// A byte encoded UTF-32 character (big-endian).
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf32::utf32be_string;
/// use somen::prelude::*;
///
/// let mut parser = utf32be_string();
///
/// let mut ok = stream::from_slice(
///     b"\x00\x00\x00\x41\x00\x00\x00\xC5\x00\x00\x30\x42\x00\x01\xF4\xAF"
/// );
/// assert_eq!(parser.parse(&mut ok).await, Ok(String::from("AÅあ💯")));
///
/// let mut err = stream::from_slice(b"\x00\x11\x00\x00");
/// assert!(parser.parse(&mut err).await.is_err());
/// # });
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn utf32be_string<'a, I>() -> impl Parser<I, Output = String>
where
    I: Input<Ok = u8> + ?Sized + 'a,
{
    utf32be_char()
        .repeat(..)
        .collect()
        .expect("UTF-32BE string")
}

/// A byte encoded UTF-32 character (little-endian).
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf32::utf32le_char;
/// use somen::prelude::*;
///
/// let mut parser = utf32le_char();
/// let mut stream = stream::from_slice(
///     b"\x41\x00\x00\x00\xC5\x00\x00\x00\x42\x30\x00\x00\xAF\xF4\x01\x00\x00\x00\x11\x00"
/// );
///
/// assert_eq!(parser.parse(&mut stream).await, Ok('A'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('Å'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('あ'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('💯'));
/// assert!(parser.parse(&mut stream).await.is_err());
/// # });
/// ```
pub fn utf32le_char<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u8> + ?Sized + 'a,
{
    any()
        .times(4)
        .fill::<4>(0)
        .try_map(|b| decode_utf32(u32::from_le_bytes(b.unwrap())).ok_or("UTF-32LE character"))
}

/// A byte encoded UTF-32 character (little-endian).
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf32::utf32le_string;
/// use somen::prelude::*;
///
/// let mut parser = utf32le_string();
///
/// let mut ok = stream::from_slice(
///     b"\x41\x00\x00\x00\xC5\x00\x00\x00\x42\x30\x00\x00\xAF\xF4\x01\x00"
/// );
/// assert_eq!(parser.parse(&mut ok).await, Ok(String::from("AÅあ💯")));
///
/// let mut err = stream::from_slice(b"\x00\x00\x11\x00");
/// assert!(parser.parse(&mut err).await.is_err());
/// # });
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn utf32le_string<'a, I>() -> impl Parser<I, Output = String>
where
    I: Input<Ok = u8> + ?Sized + 'a,
{
    utf32le_char()
        .repeat(..)
        .collect()
        .expect("UTF-32BE string")
}
