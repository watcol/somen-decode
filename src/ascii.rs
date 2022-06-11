use somen::prelude::*;

/// An ASCII encoded [`u8`] decoder.
///
/// # Examples
/// ```
/// # futures::executor::block_on(async {
/// # use somen_decode::ascii;
/// use somen::prelude::*;
///
/// let mut parser = ascii();
/// let mut stream = stream::from_slice(b"A$\n\x00\xA2");
///
/// assert_eq!(parser.parse(&mut stream).await, Ok('A'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('$'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('\n'));
/// assert_eq!(parser.parse(&mut stream).await, Ok('\x00'));
/// assert!(parser.parse(&mut stream).await.is_err());
/// # });
/// ```
pub fn ascii<'a, I>() -> impl Parser<I, Output = char>
where
    I: Positioned<Ok = u8> + ?Sized + 'a,
{
    is_some(|c| if c <= 0x7F { Some(c as char) } else { None }).expect("ascii character")
}
