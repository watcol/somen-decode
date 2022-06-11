use somen::prelude::*;

/// A UTF-8 encoded [`u8`] decoder.
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::utf8;
/// use somen::prelude::*;
///
/// let mut parser = utf8();
/// let mut stream = stream::from_slice(b"A\xC3\x85\xE3\x81\x82\xF0\x9F\x92\xAF\xC0\xAF");
///
/// assert_eq!(parser.parse(&mut stream).await, Ok('A'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('Å'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('あ'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('💯'));
/// assert!(parser.parse(&mut stream).await.is_err());
/// # });
/// ```
pub fn utf8<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u8> + ?Sized + 'a,
{
    // First byte
    is_some(|b1| match b1 {
        0x00..=0x7F => Some(Ok(b1 as u32)),
        0xC2..=0xF4 => Some(Err(b1)),
        _ => None,
    })
    // Second byte
    .then(|res| match res {
        Ok(c) => value(Ok(c)).left(),
        Err(b1) => is_some(move |b2| {
            if b2 & 0xC0 != 0x80
                || (b1 == 0xE0 && b2 < 0xA0)
                || (b1 == 0xED && b2 >= 0xA0)
                || (b1 == 0xF0 && b2 < 0x90)
                || (b1 == 0xF4 && b2 >= 0x90)
            {
                None
            } else if b1 & 0xE0 == 0xC0 {
                Some(Ok(((b1 & 0x1F) as u32) << 6 | (b2 & 0x3F) as u32))
            } else {
                Some(Err((b1, b2)))
            }
        })
        .right(),
    })
    // Third byte
    .then(|res| match res {
        Ok(c) => value(Ok(c)).left(),
        Err((b1, b2)) => is_some(move |b3| {
            if b3 & 0xC0 != 0x80 {
                None
            } else if b1 & 0xF0 == 0xE0 {
                Some(Ok(((b1 & 0x0F) as u32) << 12
                    | ((b2 & 0x3F) as u32) << 6
                    | (b3 & 0x3F) as u32))
            } else {
                Some(Err((b1, b2, b3)))
            }
        })
        .right(),
    })
    // Last byte
    .then(|res| match res {
        Ok(c) => value(c).left(),
        Err((b1, b2, b3)) => is_some(move |b4| {
            if b4 & 0xC0 != 0x80 {
                None
            } else {
                Some(
                    ((b1 & 0x07) as u32) << 18
                        | ((b2 & 0x3F) as u32) << 12
                        | ((b3 & 0x3F) as u32) << 6
                        | (b4 & 0x3F) as u32,
                )
            }
        })
        .right(),
    })
    .map(|c| unsafe { char::from_u32_unchecked(c) })
    .expect("UTF-8 character")
}
