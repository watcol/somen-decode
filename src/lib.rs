//! A byte string decoder for somen.
#![no_std]
#![doc(test(attr(warn(warnings))))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod ascii;
