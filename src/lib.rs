//! A byte string decoder for somen.
#![doc(test(attr(warn(warnings))))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;
