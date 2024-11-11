//! Provides an [inline-allocated list] which statically tracks its length.
//!
//! [inline-allocated list]: crate::NList

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(unused_results)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod macros;

/// Type-level integers which use a unary representation
pub mod peano;

mod nlist;

pub use crate::{
    nlist::*,
    peano::{PeanoInt, PlusOne, Zero},
};
