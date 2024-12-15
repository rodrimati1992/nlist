//! Traits and operations for type-level bits
//! 
//! The operators on this type-level bit representation are
//! implemented as associated types on the [`Bit`] trait,
//! and don't require bounds other than `Bit` to use them.
//! 
//! # Example
//! 
//! Returning different types depending on a type-level condition.
//! 
//! ```rust
//! use nlist::{Int, Peano, peano};
//! use nlist::peano::IsLt;
//! use nlist::bit::{self, Bool, Bit, BitWit};
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
//! // The `-> CallFn<StrOrUsize, IsLt<L, Peano!(3)>>` return type calls the `StrOrUsize` 
//! // type-level function with `IsLt<L, Peano!(3)>` as an argument.
//! // 
//! // `IsLt<L, Peano!(3)>` effectively evaluates to `Bool<{L::USIZE < 3}>`
//! const fn make<L: Int>(_: L) -> CallFn<StrOrUsize, IsLt<L, Peano!(3)>> {
//!     // Making a `BitWit<IsLt<L, Peano!(3)>>` with `Bit::BIT_WIT`
//!     match IsLt::<L, Peano!(3)>::BIT_WIT {
//!         // lt_te is a proof that `IsLt<L, Peano!(3)> == B1`, i.e.: L < 3 == true
//!         // lt_te: TypeEq<IsLt<L, Peano!(3)>, B1>
//!         BitWit::B1(lt_te) => {
//!             // te is a proof that `CallFn<StrOrUsize, IsLt<L, Peano!(3)>> == &'static str`
//!             let te: TypeEq<CallFn<StrOrUsize, IsLt<L, Peano!(3)>>, &'static str> = 
//!                 lt_te.project::<StrOrUsize>();
//!
//!             // using the type equality proof to coerce `&'static str` to the return type.
//!             te.to_left(match L::USIZE {
//!                 0 => "zero",
//!                 1 => "one",
//!                 2 => "two",
//!                 _ => panic!("unreachable"),
//!             })
//!         }
//! 
//!         // ge_te is a proof that `IsLt<L, Peano!(3)> == B0`, i.e.: L < 3 == false
//!         // ge_te: TypeEq<IsLt<L, Peano!(3)>, B0>
//!         BitWit::B0(ge_te) => {
//!             // te is a proof that `CallFn<StrOrUsize, IsLt<L, Peano!(3)>> == usize`
//!             let te: TypeEq<CallFn<StrOrUsize, IsLt<L, Peano!(3)>>, usize> = 
//!                 ge_te.project::<StrOrUsize>();
//!
//!             // using the type equality proof to coerce `usize` to the return type.
//!             te.to_left(L::USIZE)
//!         }
//!     }
//! }
//! 
//! // StrOrUsize is a type-level function (`typewit::TypeFn` implementor),
//! // which takes a Bit parameter.
//! // 
//! // In pseudocode, this is what it does on the type level:
//! // fn StrOrUsize(B: Bit) -> type {
//! //      if B { &'static str } else { usize }  
//! // }
//! type StrOrUsize = boolean::IfTrueAltFn<&'static str, usize>;
//! 
//! ```
//! 

use crate::Int;
use crate::tordering::{TOrdering, TLess, TEqual, TGreater};

use typewit::{HasTypeWitness, TypeCmp, TypeEq, TypeNe};

//////////////////////////////////////////////////////////////////////////////

/// [`typewit::TypeFn`] equivalents of boolean type aliases
pub mod type_fns;

mod bit_wit;

#[doc(no_inline)]
pub use self::{
    bit_wit::BitWit,
    type_fns::*,
};


//////////////////////////////////////////////////////////////////////////////

/// Type alias form of [`Bit::IfTrueI`]
pub type IfTrueI<B, Then, Else> = <B as Bit>::IfTrueI<Then, Else>;

/// Type alias form of [`Bit::IfTrueB`]
pub type IfTrueB<B, Then, Else> = <B as Bit>::IfTrueB<Then, Else>;

/// Type alias form of [`Bit::IfTrueTO`]
pub type IfTrueTO<B, Then, Else> = <B as Bit>::IfTrueTO<Then, Else>;

/// Type alias form of [`Bit::IfTrue`]
pub type IfTrue<B, Then, Else> = <B as Bit>::IfTrue<Then, Else>;

/// Type alias form of [`Bit::Not`]
pub type Not<B> = <B as Bit>::Not;

/// Type alias form of [`Bit::BitCmp`]
pub type BitCmp<L, R> = <L as Bit>::BitCmp<R>;

/// Type alias form of [`Bit::And`]
pub type And<L, R> = <L as Bit>::And<R>;

/// Type alias form of [`Bit::Or`]
pub type Or<L, R> = <L as Bit>::Or<R>;

/// Type alias form of [`Bit::Xor`]
pub type Xor<L, R> = <L as Bit>::Xor<R>;

/// Whether `(A, B, C)` contains at least two `B1`s
pub type AtLeastTwoB1s<A, B, C> = <A as Bit>::AtLeastTwoB1s<B, C>;

/// Whether `A - B - C` produces an overflow
pub type SubOverflows<A, B, C> = <A as Bit>::SubOverflows<B, C>;

//////////////////////////////////////////////////////////////////////////////

/// Trait for bounding [type-level bools].
///
/// # Example
///
/// For an example that (indirectly) uses this trait, 
/// you can look at the [module-level example](crate::boolean#example) 
///
/// [type-level bools]: typewit::const_marker::Bool
pub trait Bit: 
    Copy + Clone + core::fmt::Debug + Send + Sync + 'static +
    HasTypeWitness<BitWit<Self>>
{
    /// Logical negation
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::Not<B0> = Bool::<true>;
    /// let _: boolean::Not<B1> = Bool::<false>;
    /// 
    /// ```
    type Not: Bit<Not = Self>;

    /// Logical and
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::And<B0, B0> = Bool::<false>;
    /// let _: boolean::And<B0, B1> = Bool::<false>;
    /// let _: boolean::And<B1, B0> = Bool::<false>;
    /// let _: boolean::And<B1, B1> = Bool::<true>;
    /// 
    /// ```
    type And<Rhs: Bit>: Bit;

    /// Logical or
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::Or<B0, B0> = Bool::<false>;
    /// let _: boolean::Or<B0, B1> = Bool::<true>;
    /// let _: boolean::Or<B1, B0> = Bool::<true>;
    /// let _: boolean::Or<B1, B1> = Bool::<true>;
    /// 
    /// ```
    type Or<Rhs: Bit>: Bit;

    /// Exclusive or
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::Xor<B0, B0> = Bool::<false>;
    /// let _: boolean::Xor<B0, B1> = Bool::<true>;
    /// let _: boolean::Xor<B1, B0> = Bool::<true>;
    /// let _: boolean::Xor<B1, B1> = Bool::<false>;
    /// 
    /// ```
    type Xor<Rhs: Bit>: Bit;
    
    /// A type level version of [`core::cmp::Ord::cmp`] for `Bit`s.
    type BitCmp<Rhs: Bit>: TOrdering;

    /// Evaluates to different types depending on the type of `Self`:
    /// - if `Self == B1`: evaluates to `Then`
    /// - if `Self == B0`: evaluates to `Else`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::IfTrue<B0, u8, u16> = 3u16;
    /// let _: boolean::IfTrue<B0, u32, u64> = 5u64;
    /// let _: boolean::IfTrue<B1, u8, u16> = 8u8;
    /// let _: boolean::IfTrue<B1, u32, u64> = 13u32;
    /// 
    /// ```
    type IfTrue<Then, Else>;

    /// Equivalent to `IfTrue` but only takes and returns [`TOrdering`]s
    type IfTrueTO<Then: TOrdering, Else: TOrdering>: TOrdering;

    /// Equivalent to `IfTrue` but only takes and returns [`Bit`]s
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::IfTrue<B0, B0, B1> = Bool::<true>;
    /// let _: boolean::IfTrue<B0, B1, B0> = Bool::<false>;
    /// let _: boolean::IfTrue<B1, B0, B1> = Bool::<false>;
    /// let _: boolean::IfTrue<B1, B1, B0> = Bool::<true>;
    /// 
    /// ```
    type IfTrueB<Then: Bit, Else: Bit>: Bit;

    /// Equivalent to `IfTrue` but only takes and returns [`Int`]s
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{Peano, peano};
    /// use nlist::boolean::{self, Bool};
    /// 
    /// let _: boolean::IfTrue<B0, Peano!(3), Peano!(5)> = peano!(5);
    /// let _: boolean::IfTrue<B0, Peano!(8), Peano!(13)> = peano!(13);
    /// let _: boolean::IfTrue<B1, Peano!(3), Peano!(5)> = peano!(3);
    /// let _: boolean::IfTrue<B1, Peano!(8), Peano!(13)> = peano!(8);
    /// 
    /// ```
    type IfTrueI<Then: Int, Else: Int>: Int;

    /// Whether (Self, Arg1, Arg2) contains at least two `B1`s
    type AtLeastTwoB1s<Arg1: Bit, Arg2: Bit>: Bit;
    
    /// Whether `A - B - C` produces an overflow
    type SubOverflows<Arg1: Bit, Arg2: Bit>: Bit;

    /// Witness for whether `Self` is `B0` or `B1`
    const BIT_WIT: BitWit<Self>;

    /// Converts this `Bit` to a `bool`.
    /// 
    /// Converts `B0` to `false` and `B1` to `true`.
    const BOOL: bool = matches!(Self::BIT_WIT, BitWit::B1 {..});
}

/// Represents a `0` bit and `false`
#[derive(Debug, Copy, Clone)]
pub struct B0;

/// Represents a `1` bit and `true`
#[derive(Debug, Copy, Clone)]
pub struct B1;

impl Bit for B0 {
    type Not = B1;
    
    type And<Rhs: Bit> = B0;
    
    type Or<Rhs: Bit> = Rhs;

    type Xor<Rhs: Bit> = Rhs;

    type BitCmp<Rhs: Bit> = IfTrueTO<Rhs, TLess, TEqual>;

    type IfTrue<Then, Else> = Else;

    type IfTrueTO<Then: TOrdering, Else: TOrdering> = Else;
    
    type IfTrueI<Then: Int, Else: Int> = Else;

    type IfTrueB<Then: Bit, Else: Bit> = Else;

    type AtLeastTwoB1s<Arg1: Bit, Arg2: Bit> = And<Arg1, Arg2>;

    type SubOverflows<Arg1: Bit, Arg2: Bit> = Or<Arg1, Arg2>;

    const BIT_WIT: BitWit<Self> =  BitWit::B0(TypeEq::NEW);
}

impl Bit for B1 {
    type Not = B0;
    
    type And<Rhs: Bit> = Rhs;
    
    type Or<Rhs: Bit> = B1;

    type Xor<Rhs: Bit> = Rhs::Not;

    type BitCmp<Rhs: Bit> = IfTrueTO<Rhs, TEqual, TGreater>;

    type IfTrue<Then, Else> = Then;

    type IfTrueTO<Then: TOrdering, Else: TOrdering> = Then;
    
    type IfTrueI<Then: Int, Else: Int> = Then;

    type IfTrueB<Then: Bit, Else: Bit> = Then;

    type AtLeastTwoB1s<Arg1: Bit, Arg2: Bit> = Or<Arg1, Arg2>;

    type SubOverflows<Arg1: Bit, Arg2: Bit> = And<Arg1, Arg2>;

    const BIT_WIT: BitWit<Self> =  BitWit::B1(TypeEq::NEW);
}


/// Returns a [`TypeCmp<L, R>`], which is a proof of whether `L == R` or `L != R`.
pub const fn eq<L, R>() -> TypeCmp<L, R>
where
    L: Bit,
    R: Bit,
{
    const {
        match (L::BIT_WIT, R::BIT_WIT) {
            (BitWit::B0(l_te), BitWit::B0(r_te)) => {
                TypeCmp::Eq(l_te.join(r_te.flip()))
            }
            (BitWit::B0(l_te), BitWit::B1(r_te)) => {
                TypeCmp::Ne(b0_b1_inequality().join_left(l_te).join_right(r_te.flip()))
            }
            (BitWit::B1(l_te), BitWit::B0(r_te)) => {
                TypeCmp::Ne(b0_b1_inequality().flip().join_left(l_te).join_right(r_te.flip()))
            }
            (BitWit::B1(l_te), BitWit::B1(r_te)) => {
                TypeCmp::Eq(l_te.join(r_te.flip()))
            }
        }
    }
}



const fn b0_b1_inequality() -> TypeNe<B0, B1> {
    typewit::type_ne!( B0, B1 )
}


/// Diverges when given a proof of `B1 == B0`
/// (which is a contradiction, because they're different types).
pub const fn contradiction(length_te: TypeEq<B1, B0>) -> ! {
    typewit::type_fn! {
        struct TrueEqualsFalseFn<T, U>;

        impl B1 => T;
        impl B0 => U;
    }

    length_te.map(TrueEqualsFalseFn::NEW).to_left(())
}
