//! Provides an [inline-allocated list] which statically tracks its length.
//!
//! # Example
//!
//! ### Splitting and recombining
//!
//! This example shows how NLists can be split and recombined in const,
//! even if the length is generic, 
//! so long as the length is known to be greater than the split index.
//!
//! ```rust
//! use nlist::{NList, Peano, PeanoInt, nlist, peano};
//!
//! const LIST: NList<u128, Peano!(7)> = transform(nlist![3, 5, 8, 13, 21, 34, 55]);
//!
//! assert_eq!(LIST, nlist![21, 34, 55, 6, 10, 16, 26]);
//!
//! type SplitIndex = Peano!(4);
//! const fn transform<L>(
//!     list: NList<u128, peano::Add<SplitIndex, L>>
//! ) -> NList<u128, peano::Add<SplitIndex, L>>
//! where
//!     L: PeanoInt,
//! {
//!     // if we use `let` to destructure instead of `konst::destructure`,
//!     // we get a "destructor cannot be evaluated at compile-time" error as of Rust 1.83
//!     konst::destructure!{(mut before, after) = list.split_at::<SplitIndex>()}
//!     
//!     let mut array = before.into_array();
//!     mutate_array(&mut array);
//!     before = NList::from_array(array);
//!     
//!     // math spice: using arithmetic properties to coerce equal generic lengths
//!     // 
//!     // coercing `NList<u128, L - 0>` to `NList<u128, L>`
//!     after.coerce_len(peano::proofs::sub_identity::<L>())
//!         .concat(before)
//!         // coercing `NList<u128, L + SplitIndex>` to `NList<u128, SplitIndex + L>`
//!         .coerce_len(peano::proofs::commutative_add::<L, SplitIndex>())
//! }
//! 
//! const fn mutate_array(array: &mut [u128; SplitIndex::USIZE]) {
//!     *array = konst::array::map_!(*array, |x| x * 2);
//! }
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

pub mod boolean;

pub mod peano;

mod nlist;

mod imply_trait;

pub mod receiver;

pub use crate::{
    nlist::*,
    peano::{PeanoInt, PeanoWit, PlusOne, Zero},
};




#[doc(hidden)]
pub mod __ {
    pub use konst::destructure;

    pub use core::primitive::bool;
}


