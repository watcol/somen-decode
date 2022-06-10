//! A byte string decoder for somen.
#![doc(test(attr(warn(warnings))))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod ascii;
