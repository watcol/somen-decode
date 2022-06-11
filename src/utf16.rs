use somen::prelude::*;

// A base decoder
#[inline]
fn decode_one(b: u16) -> Option<Result<char, u16>> {
    if b & 0xFC00 == 0xDC00 {
        None
    } else if b & 0xFC00 == 0xD800 {
        Some(Err(b))
    } else {
        Some(Ok(unsafe { char::from_u32_unchecked(b as u32) }))
    }
}

fn decode_two(b1: u16, b2: u16) -> Option<char> {
    if b2 & 0xFC00 == 0xDC00 {
        let cp =
            (((b1 & 0x3C0) + 1) as u32) << 16 | ((b1 & 0x3F) as u32) << 10 | (b2 & 0x3FF) as u32;
        Some(unsafe { char::from_u32_unchecked(cp) })
    } else {
        None
    }
}

/// A UTF-16 encoded [`u16`] decoder.
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf16;
/// use somen::prelude::*;
///
/// let mut parser = utf16();
/// let mut stream = stream::from_slice(&[
///     0xD834, 0xDD1E, 0x004d, 0x0075, 0x0073, 0x0069, 0x0063, 0xD834,
/// ]);
///
/// assert_eq!(parser.parse(&mut stream).await, Ok('𝄞'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('M'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('u'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('s'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('i'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('c'));
/// assert!(parser.parse(&mut stream).await.is_err());
/// # });
/// ```
pub fn utf16<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u16> + ?Sized + 'a,
{
    is_some(decode_one)
        .then(|res| match res {
            Ok(c) => value(c).left(),
            Err(b1) => is_some(move |b2| decode_two(b1, b2)).right(),
        })
        .expect("UTF-16 character")
}

/// A UTF-16 encoded [`u8`] decoder. (big-endian)
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf16be;
/// use somen::prelude::*;
///
/// let mut parser = utf16be();
/// let mut stream = stream::from_slice(
///     b"\xD8\x34\xDD\x1E\x00\x4d\x00\x75\x00\x73\x00\x69\x00\x63\xD8\x34",
/// );
///
/// assert_eq!(parser.parse(&mut stream).await, Ok('𝄞'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('M'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('u'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('s'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('i'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('c'));
/// assert!(parser.parse(&mut stream).await.is_err());
/// # });
/// ```
pub fn utf16be<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u8> + ?Sized + 'a,
{
    any()
        .times(2)
        .fill::<2>(0)
        .try_map(|b| decode_one(u16::from_be_bytes(b.unwrap())).ok_or("UTF-16BE character"))
        .rewindable()
        .then(|res| match res {
            Ok(c) => value(c).left(),
            Err(b1) => any()
                .times(2)
                .fill::<2>(0)
                .try_map(move |b2| {
                    decode_two(b1, u16::from_be_bytes(b2.unwrap())).ok_or("UTF-16BE character")
                })
                .right(),
        })
}

/// A UTF-16 encoded [`u8`] decoder. (little-endian)
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf16le;
/// use somen::prelude::*;
///
/// let mut parser = utf16le();
/// let mut stream = stream::from_slice(
///     b"\x34\xD8\x1E\xDD\x4d\x00\x75\x00\x73\x00\x69\x00\x63\x00\x34\xD8",
/// );
///
/// assert_eq!(parser.parse(&mut stream).await, Ok('𝄞'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('M'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('u'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('s'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('i'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('c'));
/// assert!(parser.parse(&mut stream).await.is_err());
/// # });
/// ```
pub fn utf16le<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u8> + ?Sized + 'a,
{
    any()
        .times(2)
        .fill::<2>(0)
        .try_map(|b| decode_one(u16::from_le_bytes(b.unwrap())).ok_or("UTF-16LE character"))
        .rewindable()
        .then(|res| match res {
            Ok(c) => value(c).left(),
            Err(b1) => any()
                .times(2)
                .fill::<2>(0)
                .try_map(move |b2| {
                    decode_two(b1, u16::from_le_bytes(b2.unwrap())).ok_or("UTF-16LE character")
                })
                .right(),
        })
}
