//! Provides an [inline-allocated list] which statically tracks its length.
//!
//! # Example
//!
//! ```rust
//! use nlist::{NList, Peano, PeanoInt, nlist, peano};
//!
//! fn transform<T, L>(list: NList<T, L>) -> NList<i128, peano::Add<L, Peano!(1)>>
//! where
//!     T: Into<i128>,
//!     L: PeanoInt,
//! {
//!     list.reverse()
//!         .map(|x| -> i128 { x.into() })
//!         .map(|x| x * 10)
//!         .concat(nlist![0])
//! }
//!
//! let fibb = transform(nlist![3, 5, 8]);
//! assert_eq!(fibb.into_vec(), vec![80, 50, 30, 0]);
//!
//! let powers = transform(nlist![4u8, 9, 25]);
//! assert_eq!(powers.into_vec(), vec![250, 90, 40, 0]);
//!
//! ```
//!
//! [inline-allocated list]: crate::NList
//! [`NList`]: crate::NList

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(unused_results)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(no_inline)]
pub use typewit;

#[macro_use]
mod macros;

pub mod peano;

mod nlist;

pub use crate::{
    nlist::*,
    peano::{PeanoInt, PeanoWit, PlusOne, Zero},
};
