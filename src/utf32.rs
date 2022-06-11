use somen::prelude::*;

// A base decoder
#[inline]
fn decode(c: u32) -> Option<char> {
    char::from_u32(c)
}

/// A UTF-32 encoded [`u32`] decoder.
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf32;
/// use somen::prelude::*;
///
/// let mut parser = utf32();
/// let mut stream = stream::from_slice(&[0x41, 0xC5, 0x3042, 0x1F4AF, 0x110000]);
///
/// assert_eq!(parser.parse(&mut stream).await, Ok('A'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('Å'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('あ'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('💯'));
/// assert!(parser.parse(&mut stream).await.is_err());
/// # });
/// ```
pub fn utf32<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u32> + ?Sized + 'a,
{
    is_some(decode).expect("UTF-32 character")
}

/// A UTF-32 encoded [`u8`] decoder. (big-endian)
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf32_be;
/// use somen::prelude::*;
///
/// let mut parser = utf32_be();
/// let mut stream = stream::from_slice(
///     b"\x00\x00\x00\x41\x00\x00\x00\xC5\x00\x00\x30\x42\x00\x01\xF4\xAF\x00\x11\x00\x00",
/// );
///
/// assert_eq!(parser.parse(&mut stream).await, Ok('A'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('Å'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('あ'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('💯'));
/// assert!(parser.parse(&mut stream).await.is_err());
/// # });
/// ```
pub fn utf32_be<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u8> + ?Sized + 'a,
{
    (any(), any(), any(), any())
        .try_map(|(b1, b2, b3, b4)| {
            decode(u32::from_be_bytes([b1, b2, b3, b4])).ok_or("UTF-32LE character")
        })
        .rewindable()
}

/// A UTF-32 encoded [`u8`] decoder. (little-endian)
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf32_le;
/// use somen::prelude::*;
///
/// let mut parser = utf32_le();
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
pub fn utf32_le<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u8> + ?Sized + 'a,
{
    (any(), any(), any(), any())
        .try_map(|(b1, b2, b3, b4)| {
            decode(u32::from_le_bytes([b1, b2, b3, b4])).ok_or("UTF-32LE character")
        })
        .rewindable()
}
