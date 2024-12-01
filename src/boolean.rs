//! Traits and operations for type-level booleans
//! 
//! # Example
//! 
//! Returning different types depending on a type-level condition.
//! 
//! ```rust
//! use nlist::{PeanoInt, Peano, peano};
//! use nlist::boolean::{self, Bool, Boolean, BoolWitG};
//! use nlist::typewit::{CallFn, TypeEq};
//! 
//! assert_eq!(make(peano!(0)), "zero");
//! assert_eq!(make(peano!(1)), "one");
//! assert_eq!(make(peano!(2)), "two");
//! assert_eq!(make(peano!(3)), 3);
//! assert_eq!(make(peano!(4)), 4);
//! 
//! // Function which returns different types depending on the value of `L`
//! // 
//! // If L < 3, this returns a &'static str
//! // If L >= 3, this returns a usize
//! // 
//! // The `-> CallFn<StrOrUsize, Lt3<L>>` return type calls the `StrOrUsize` type-level function 
//! // with `Lt3<L>` as an argument.
//! const fn make<L: PeanoInt>(_: L) -> CallFn<StrOrUsize, Lt3<L>> {
//!     match Lt3::<L>::BOOL_WIT {
//!         // lt_te is a proof that `Lt3<L> == Bool<true>`, i.e.: L < 3 == true
//!         // lt_te: TypeEq<Lt3<L>, Bool<true>>
//!         BoolWitG::True(lt_te) => {
//!             // te is a proof that `CallFn<StrOrUsize, Lt3<L>> == &'static str`
//!             let te: TypeEq<CallFn<StrOrUsize, Lt3<L>>, &'static str> = 
//!                 lt_te.project::<StrOrUsize>();
//!
//!             te.to_left(match L::USIZE {
//!                 0 => "zero",
//!                 1 => "one",
//!                 2 => "two",
//!                 _ => panic!("unreachable"),
//!             })
//!         }
//! 
//!         // gt_te is a proof that `Lt3<L> == Bool<false>`, i.e.: L < 3 == false
//!         // gt_te: TypeEq<Lt3<L>, Bool<false>>
//!         BoolWitG::False(gt_te) => {
//!             // te is a proof that `CallFn<StrOrUsize, Lt3<L>> == usize`
//!             let te: TypeEq<CallFn<StrOrUsize, Lt3<L>>, usize> = gt_te.project::<StrOrUsize>();
//!
//!             te.to_left(L::USIZE)
//!         }
//!     }
//! }
//! 
//! // Evaluates to `nlist::boolean::Bool<L < 3>`
//! type Lt3<L> = peano::IsLt<L, Peano!(3)>;
//! 
//! // StrOrUsize is a type-level function (`typewit::TypeFn` implementor),
//! // which takes a Boolean parameter.
//! // 
//! // In pseudocode, this is what it does on the type level:
//! // fn StrOrUsize(B: Boolean) -> type {
//! //      if B { &'static str } else { usize }  
//! // }
//! type StrOrUsize = boolean::IfTrueAltFn<&'static str, usize>;
//! 
//! ```
//! 


use crate::PeanoInt;


#[doc(no_inline)]
pub use typewit::const_marker::{Bool, BoolWit, BoolWitG};

use typewit::{HasTypeWitness, TypeEq};

//////////////////////////////////////////////////////////////////////////////

/// [`typewit::TypeFn`] equivalents of boolean type aliases
pub mod type_fns;

#[doc(no_inline)]
pub use self::type_fns::*;

//////////////////////////////////////////////////////////////////////////////

/// Type alias form of [`Boolean::IfTruePI`]
pub type IfTruePI<B, Then, Else> = <B as Boolean>::IfTruePI<Then, Else>;

/// Type alias form of [`Boolean::IfTrueB`]
pub type IfTrueB<B, Then, Else> = <B as Boolean>::IfTrueB<Then, Else>;

/// Type alias form of [`Boolean::IfTrue`]
pub type IfTrue<B, Then, Else> = <B as Boolean>::IfTrue<Then, Else>;

/// Type alias form of [`Boolean::Not`]
pub type Not<B> = <B as Boolean>::Not;

/// Type alias form of [`Boolean::And`]
pub type And<L, R> = <L as Boolean>::And<R>;

/// Type alias form of [`Boolean::Or`]
pub type Or<L, R> = <L as Boolean>::Or<R>;

//////////////////////////////////////////////////////////////////////////////

/// Trait for bounding [type-level bools].
///
/// [type-level bools]: typewit::const_marker::Bool
pub trait Boolean: 
    Copy + Clone + core::fmt::Debug + Send + Sync +
    HasTypeWitness<BoolWitG<Self>>
{
    /// Logical negation
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::Not<Bool<false>> = Bool::<true>;
    /// let _: boolean::Not<Bool<true>> = Bool::<false>;
    /// 
    /// ```
    type Not: Boolean<Not = Self>;

    /// Logical and
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::And<Bool<false>, Bool<false>> = Bool::<false>;
    /// let _: boolean::And<Bool<false>, Bool<true>> = Bool::<false>;
    /// let _: boolean::And<Bool<true>, Bool<false>> = Bool::<false>;
    /// let _: boolean::And<Bool<true>, Bool<true>> = Bool::<true>;
    /// 
    /// ```
    type And<Rhs: Boolean>: Boolean;
    
    /// Logical or
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::Or<Bool<false>, Bool<false>> = Bool::<false>;
    /// let _: boolean::Or<Bool<false>, Bool<true>> = Bool::<true>;
    /// let _: boolean::Or<Bool<true>, Bool<false>> = Bool::<true>;
    /// let _: boolean::Or<Bool<true>, Bool<true>> = Bool::<true>;
    /// 
    /// ```
    type Or<Rhs: Boolean>: Boolean;

    /// Evaluates to different types depending on the type of `Self`:
    /// - if `Self == Bool<true>`: evaluates to `Then`
    /// - if `Self == Bool<false>`: evaluates to `Else`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::IfTrue<Bool<false>, u8, u16> = 3u16;
    /// let _: boolean::IfTrue<Bool<false>, u32, u64> = 5u64;
    /// let _: boolean::IfTrue<Bool<true>, u8, u16> = 8u8;
    /// let _: boolean::IfTrue<Bool<true>, u32, u64> = 13u32;
    /// 
    /// ```
    type IfTrue<Then, Else>;

    /// Equivalent to `IfTrue` but only takes and returns [`Boolean`]s
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::IfTrue<Bool<false>, Bool<false>, Bool<true>> = Bool::<true>;
    /// let _: boolean::IfTrue<Bool<false>, Bool<true>, Bool<false>> = Bool::<false>;
    /// let _: boolean::IfTrue<Bool<true>, Bool<false>, Bool<true>> = Bool::<false>;
    /// let _: boolean::IfTrue<Bool<true>, Bool<true>, Bool<false>> = Bool::<true>;
    /// 
    /// ```
    type IfTrueB<Then: Boolean, Else: Boolean>: Boolean;

    /// Equivalent to `IfTrue` but only takes and returns [`PeanoInt`]s
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{Peano, peano};
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::IfTrue<Bool<false>, Peano!(3), Peano!(5)> = peano!(5);
    /// let _: boolean::IfTrue<Bool<false>, Peano!(8), Peano!(13)> = peano!(13);
    /// let _: boolean::IfTrue<Bool<true>, Peano!(3), Peano!(5)> = peano!(3);
    /// let _: boolean::IfTrue<Bool<true>, Peano!(8), Peano!(13)> = peano!(8);
    /// 
    /// ```
    type IfTruePI<Then: PeanoInt, Else: PeanoInt>: PeanoInt;

    /// Witness for whether `Self` is `Bool<false>` or `Bool<true>`
    const BOOL_WIT: BoolWitG<Self> = Self::WITNESS;
}

impl Boolean for Bool<false> {
    type Not = Bool<true>;
    
    type And<Rhs: Boolean> = Bool<false>;
    
    type Or<Rhs: Boolean> = Rhs;

    type IfTrue<Then, Else> = Else;

    type IfTruePI<Then: PeanoInt, Else: PeanoInt> = Else;

    type IfTrueB<Then: Boolean, Else: Boolean> = Else;
}

impl Boolean for Bool<true> {
    type Not = Bool<false>;
    
    type And<Rhs: Boolean> = Rhs;
    
    type Or<Rhs: Boolean> = Bool<true>;

    type IfTrue<Then, Else> = Then;

    type IfTruePI<Then: PeanoInt, Else: PeanoInt> = Then;

    type IfTrueB<Then: Boolean, Else: Boolean> = Then;
}


/// Diverges when given a proof of `Bool<true> == Bool<false>`
/// (which is a contradiction, because they're different types).
pub const fn contradiction(length_te: TypeEq<Bool<true>, Bool<false>>) -> ! {
    typewit::type_fn! {
        struct TrueEqualsFalseFn<T, U>;

        impl Bool<true> => T;
        impl Bool<false> => U;
    }

    length_te.map(TrueEqualsFalseFn::NEW).to_left(())
}
