//! Type-level integers which use a unary representation
//! 
//! The operators on this type-level integer representation are
//! implemented as associated types on the [`Int`] trait,
//! and don't require bounds other than `Int` to use them.
//! 
//! # Example
//! 
//! Constructing an NList that's `A * B + C` long.
//! 
//! ```rust
//! use nlist::{NList, Int, peano, nlist};
//! 
//! // returned list length: 3 * 0 + 1 == 1
//! assert_eq!(make_nlist::<Int!(3), Int!(0), Int!(1)>(), nlist![0]);
//! 
//! // returned list length: 2 * 1 + 3 == 5
//! assert_eq!(make_nlist::<Int!(2), Int!(1), Int!(3)>(), nlist![0, 1, 4, 9, 16]);
//! 
//! // returned list length: 3 * 2 + 1 == 7
//! assert_eq!(make_nlist::<Int!(3), Int!(2), Int!(1)>(), nlist![0, 1, 4, 9, 16, 25, 36]);
//! 
//! // makes an NList<u64, A * B + C>
//! const fn make_nlist<A, B, C>() -> NList<u64, peano::Add<peano::Mul<A, B>, C>>
//! where
//!     A: Int,
//!     B: Int,
//!     C: Int,
//! {
//!     // the recursive function that constructs the list
//!     const fn inner<L: Int>(index: u64) -> NList<u64, L> {
//!         nlist::rec_from_fn!(|| (index.pow(2), inner(index + 1)))
//!     }
//! 
//!     inner(0)
//! }
//! 
//! ```


use core::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    fmt,
    hash::Hash,
    marker::PhantomData,
};


use typewit::{TypeCmp, TypeEq, TypeNe};


use crate::bit::{self, Bit, B0};
use crate::tordering::{self, TOrdering};


///////////////////////////////////////////////////////////////////////////////

mod int_wit;

pub use int_wit::IntWit;

mod from_const;

/// [`typewit::TypeFn`] equivalents of peano type aliases
pub mod type_fns;

#[doc(no_inline)]
pub use self::type_fns::*;


#[doc(no_inline)]
pub use typewit::const_marker::Usize;

pub use self::from_const::{FromInt, FromUsize, IntoInt, IntoUsize};

///////////////////////////////////////////////////////////////////////////////


// The impls of std traits for Zeros and PlusOne are all here
mod int_types;

pub use self::int_types::{Zeros, Nat};


///////////////////////////////////////////////////////////////////////////////


/// Type-level equivalent of `N >> 1`
pub type ShrOne<N> = <N as Int>::ShrOne;

/// Type alias form of [`Int::IsZeros`]
pub type IsZeros<Lhs> = <Lhs as Int>::IsZeros;

/// Type alias form of [`Int::SubOneSat`]
pub type SubOneSat<Lhs> = <Lhs as Int>::SubOneSat;

/// Type-level equivalent of `N << 1`
pub type ShlOne<N> = IfZeroI<N, N, Nat<N, B0>>;

/// Type alias form of [`Int::IsZeros`] chained with [`Bit::IfTrue`]
pub type IfZero<L, Then, Else> = <IsZeros<L> as Bit>::IfTrue<Then, Else>;

/// Type alias form of [`Int::IsZeros`] chained with [`Bit::IfTrueI`]
pub type IfZeroI<L, Then, Else> = <IsZeros<L> as Bit>::IfTrueI<Then, Else>;

/// Type alias form of [`Int::SubSat`]
pub type SubSat<Lhs, Rhs> = <Lhs as Int>::SubSat<Rhs>;

/// Type alias form of [`Int::Add`]
pub type Add<Lhs, Rhs> = <Lhs as Int>::Add<Rhs>;

/// Type alias form of [`Int::Mul`]
pub type Mul<Lhs, Rhs> = <Lhs as Int>::Mul<Rhs>;

/// Type alias form of [`Int::Min`]
pub type Min<Lhs, Rhs> = <Lhs as Int>::Min<Rhs>;

/// Type alias form of [`Int::Max`]
pub type Max<Lhs, Rhs> = <Lhs as Int>::Max<Rhs>;

/// Type alias form of [`Int::IsLe`]
pub type IsLe<Lhs, Rhs> = <Lhs as Int>::IsLe<Rhs>;

/// Type alias form of [`Int::IsLt`]
pub type IsLt<Lhs, Rhs> = tordering::IsLess<Cmp<Lhs, Rhs>>;

/// Type alias form of [`Int::Cmp`]
pub type Cmp<Lhs, Rhs> = <Lhs as Int>::Cmp<Rhs>;



/// Trait for a type-level unary encoding of unsigned integers.
/// 
/// Only [`Zeros`] and [`PlusOne`] implement this trait,
/// no other type can implement it.
/// 
/// # Example
/// 
/// Constructing a tuple `L`-levels deep with a recursive function.
/// 
/// ```rust
/// use nlist::{Int, peano};
/// use nlist::peano::{Int, IntWit, PlusOne, Zeros};
/// use nlist::typewit::{CallFn, type_fn};
/// 
/// 
/// assert_eq!(recursive::<Int!(0)>(), ());
/// assert_eq!(recursive::<Int!(1)>(), (1, ()));
/// assert_eq!(recursive::<Int!(2)>(), (2, (1, ())));
/// assert_eq!(recursive::<Int!(3)>(), (3, (2, (1, ()))));
/// 
/// 
/// // The `-> CallFn<IntToTupleFn<usize>, L>` return type 
/// // calls the `IntToTupleFn<usize>` type-level function with `L` as an argument.
/// const fn recursive<L: IntToTuple>() -> CallFn<IntToTupleFn<usize>, L> {
///     match L::PEANO_WIT {
///         IntWit::PlusOne(len_te) => {
///             len_te.project::<IntToTupleFn<usize>>()
///                 .to_left((L::USIZE, recursive::<L::SubOneSat>()))
///         }
///         IntWit::Zeros(len_te) => {
///             len_te.project::<IntToTupleFn<usize>>().to_left(())
///         }
///     }
/// }
/// 
/// type_fn! {
///     // `IntToTupleFn<T>` is a type-level function (`typewit::TypeFn` implementor) 
///     // from `L` to <L as IntToTuple>::Output::<T>
///     struct IntToTupleFn<T>;
/// 
///     impl<L: IntToTuple> L => L::Output::<T>;
/// }
/// 
/// trait IntToTuple: Int<SubOneSat = Self::SubOneSat_> {
///     type SubOneSat_: IntToTuple;
///     type Output<T>;
/// }
/// 
/// impl IntToTuple for Zeros {
///     type SubOneSat_ = Zeros;
///     type Output<T> = ();
/// }
/// 
/// impl<L: IntToTuple> IntToTuple for PlusOne<L> {
///     type SubOneSat_ = L;
///     type Output<T> = (T, L::Output<T>);
/// }
/// ```
/// 
pub trait Int: 
    Sized + Copy + Default + Hash + Sync + Send +
    Eq + Ord + PartialEq + PartialEq<usize> + PartialOrd + PartialOrd<usize> +
    fmt::Binary + fmt::Debug + fmt::Display + fmt::LowerHex + fmt::Octal + fmt::UpperHex +
    'static 
{
    /// The lowest [`Bit`] of this integer
    type BitArg: Bit;

    /// Type-level equivalent of `N >> 1`
    type ShrOne: Int;

    #[doc(hidden)]
    type __PairOfInts<R: Int>: IntCmpHelper<L = Self, R = R>;

    /// Whether `Self` is Zeros
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{Int, peano};
    /// use nlist::boolean::{B0, B1};
    /// 
    /// let _: peano::IsZeros<Int!(0)> = B1;
    /// let _: peano::IsZeros<Int!(1)> = B0;
    /// let _: peano::IsZeros<Int!(2)> = B0;
    /// let _: peano::IsZeros<Int!(3)> = B0;
    /// 
    /// ```
    type IsZeros: Bit;

    /// Type level equivalent of `.saturating_sub(1)`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{Int, peano};
    /// 
    /// assert_eq!(peano::SubOneSat::<Int!(0)>::NEW, 0);
    /// assert_eq!(peano::SubOneSat::<Int!(1)>::NEW, 0);
    /// assert_eq!(peano::SubOneSat::<Int!(2)>::NEW, 1);
    /// assert_eq!(peano::SubOneSat::<Int!(3)>::NEW, 2);
    /// 
    /// ```
    type SubOneSat: Int;

    /// Type level equivalent of `.saturating_sub(R)`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{Int, peano};
    /// 
    /// assert_eq!(peano::SubSat::<Int!(3), Int!(0)>::NEW, 3);
    /// assert_eq!(peano::SubSat::<Int!(3), Int!(1)>::NEW, 2);
    /// assert_eq!(peano::SubSat::<Int!(3), Int!(2)>::NEW, 1);
    /// assert_eq!(peano::SubSat::<Int!(3), Int!(3)>::NEW, 0);
    /// assert_eq!(peano::SubSat::<Int!(3), Int!(4)>::NEW, 0);
    /// ```
    type SubSat<R: Int>: Int;

    #[doc(hidden)]
    type __SubSat<Rhs: Int, Overflow: Bit>: Int;

    /// Computes the addition of `Self` and `Rhs`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{Int, peano};
    /// 
    /// assert_eq!(peano::Add::<Int!(0), Int!(0)>::NEW, 0);
    /// assert_eq!(peano::Add::<Int!(0), Int!(1)>::NEW, 1);
    /// assert_eq!(peano::Add::<Int!(0), Int!(2)>::NEW, 2);
    /// 
    /// assert_eq!(peano::Add::<Int!(1), Int!(0)>::NEW, 1);
    /// assert_eq!(peano::Add::<Int!(1), Int!(1)>::NEW, 2);
    /// assert_eq!(peano::Add::<Int!(1), Int!(2)>::NEW, 3);
    /// 
    /// assert_eq!(peano::Add::<Int!(2), Int!(0)>::NEW, 2);
    /// assert_eq!(peano::Add::<Int!(2), Int!(1)>::NEW, 3);
    /// assert_eq!(peano::Add::<Int!(2), Int!(2)>::NEW, 4);
    /// 
    /// ```
    type Add<Rhs: Int>: Int;

    #[doc(hidden)]
    type __Add<Rhs: Int, Overflow: Bit>: Int;

    /// Computes `Self` multiplied by `Rhs`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{Int, peano};
    /// 
    /// assert_eq!(peano::Mul::<Int!(0), Int!(0)>::NEW, 0);
    /// assert_eq!(peano::Mul::<Int!(0), Int!(1)>::NEW, 0);
    /// 
    /// assert_eq!(peano::Mul::<Int!(1), Int!(0)>::NEW, 0);
    /// assert_eq!(peano::Mul::<Int!(1), Int!(1)>::NEW, 1);
    /// assert_eq!(peano::Mul::<Int!(1), Int!(2)>::NEW, 2);
    /// 
    /// assert_eq!(peano::Mul::<Int!(2), Int!(1)>::NEW, 2);
    /// assert_eq!(peano::Mul::<Int!(2), Int!(2)>::NEW, 4);
    /// assert_eq!(peano::Mul::<Int!(2), Int!(3)>::NEW, 6);
    /// 
    /// ```
    type Mul<Rhs: Int>: Int;

    /// Computes the minimum of `Self` and `Rhs`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{Int, peano};
    /// 
    /// assert_eq!(peano::Min::<Int!(0), Int!(0)>::NEW, 0);
    /// assert_eq!(peano::Min::<Int!(0), Int!(1)>::NEW, 0);
    /// assert_eq!(peano::Min::<Int!(0), Int!(2)>::NEW, 0);
    /// 
    /// assert_eq!(peano::Min::<Int!(1), Int!(0)>::NEW, 0);
    /// assert_eq!(peano::Min::<Int!(1), Int!(1)>::NEW, 1);
    /// assert_eq!(peano::Min::<Int!(1), Int!(2)>::NEW, 1);
    /// 
    /// assert_eq!(peano::Min::<Int!(2), Int!(0)>::NEW, 0);
    /// assert_eq!(peano::Min::<Int!(2), Int!(1)>::NEW, 1);
    /// assert_eq!(peano::Min::<Int!(2), Int!(2)>::NEW, 2);
    /// 
    /// ```
    type Min<Rhs: Int>: Int;

    /// Computes the maximum of `Self` and `Rhs`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{Int, peano};
    /// 
    /// assert_eq!(peano::Max::<Int!(0), Int!(0)>::NEW, 0);
    /// assert_eq!(peano::Max::<Int!(0), Int!(1)>::NEW, 1);
    /// assert_eq!(peano::Max::<Int!(0), Int!(2)>::NEW, 2);
    /// 
    /// assert_eq!(peano::Max::<Int!(1), Int!(0)>::NEW, 1);
    /// assert_eq!(peano::Max::<Int!(1), Int!(1)>::NEW, 1);
    /// assert_eq!(peano::Max::<Int!(1), Int!(2)>::NEW, 2);
    /// 
    /// assert_eq!(peano::Max::<Int!(2), Int!(0)>::NEW, 2);
    /// assert_eq!(peano::Max::<Int!(2), Int!(1)>::NEW, 2);
    /// assert_eq!(peano::Max::<Int!(2), Int!(2)>::NEW, 2);
    /// 
    /// ```
    type Max<Rhs: Int>: Int;

    /// Whether `Self <= Rhs`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{Int, peano};
    /// use nlist::boolean::{B0, B1};
    /// 
    /// let _: peano::IsLe<Int!(0), Int!(0)> = B1;
    /// let _: peano::IsLe<Int!(0), Int!(1)> = B1;
    /// let _: peano::IsLe<Int!(0), Int!(2)> = B1;
    /// 
    /// let _: peano::IsLe<Int!(1), Int!(0)> = B0;
    /// let _: peano::IsLe<Int!(1), Int!(1)> = B1;
    /// let _: peano::IsLe<Int!(1), Int!(2)> = B1;
    /// 
    /// let _: peano::IsLe<Int!(2), Int!(0)> = B0;
    /// let _: peano::IsLe<Int!(2), Int!(1)> = B0;
    /// let _: peano::IsLe<Int!(2), Int!(2)> = B1;
    /// 
    /// ```
    type IsLe<Rhs: Int>: Bit;

    /// A type level version of [`core::cmp::Ord::cmp`] for `Int`s.
    type Cmp<Rhs: Int>: TOrdering;

    /// Constructs this type
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{Int};
    /// 
    /// let int = <Int!(2)>::NEW;
    /// 
    /// assert_eq!(int, 2);
    /// ```
    const NEW: Self;

    /// What integer value `Self` represents.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{Int};
    /// 
    /// assert_eq!(<Int!(3)>::USIZE, 3);
    /// assert_eq!(<Int!(5)>::USIZE, 5);
    /// ```
    const USIZE: usize;

    /// A type witness for whether `Self` is `Zeros` or `Nat`
    /// 
    /// For an example, you can look at the docs of [`IntWit`] itself
    /// 
    const INT_WIT: IntWit<Self>;
}


mod pair_of_ints;

use self::pair_of_ints::{IntCmpHelper, PairOfInts_};

pub mod proofs;



////////////////////////////////////////////////////////////////////////////////

/// Converts the peano integer to a usize
/// 
/// # Example
/// 
/// ```rust
/// use nlist::peano;
/// 
/// assert_eq!(peano::to_usize(peano!(0)), 0);
/// assert_eq!(peano::to_usize(peano!(1)), 1);
/// assert_eq!(peano::to_usize(peano!(2)), 2);
/// assert_eq!(peano::to_usize(peano!(3)), 3);
/// 
/// ```
pub const fn to_usize<I: Int>(_: I) -> usize {
    I::USIZE
}

////////////////////////////////////////////////////////////////////////////////


/// Returns a [`TypeCmp<L, R>`], which is a proof of whether `L == R` or `L != R`.
/// 
/// # Example
/// 
/// Coercing an [`NList`](crate::NList) to a specific length
/// 
/// ```rust
/// use nlist::{NList, Int, nlist, peano};
/// 
/// assert_eq!(try_coerce(nlist![0; 0]), None);
/// assert_eq!(try_coerce(nlist![3]), None);
/// assert_eq!(try_coerce(nlist![3, 5]), None);
/// assert_eq!(try_coerce(nlist![3, 5, 8]), Some(nlist![3, 5, 8]));
/// assert_eq!(try_coerce(nlist![3, 5, 8, 13]), None);
/// assert_eq!(try_coerce(nlist![3, 5, 8, 13, 21]), None);
/// 
/// const fn try_coerce<T, L>(list: NList<T, L>) -> Option<NList<T, Int!(3)>>
/// where
///     T: Copy,
///     L: Int
/// {
///     match peano::eq::<L, Int!(3)>().eq() {
///         Some(te) => Some(list.coerce_len(te)),
///         None => {
///             // works around "destructor cannot be evaluated at compile-time" error
///             list.assert_copy_drop();
///
///             None
///         }
///     }
/// }
/// ```
pub const fn eq<L, R>() -> TypeCmp<L, R>
where
    L: Int,
    R: Int,
{
    PairOfInts_::<L, R>::EQ_WIT
}

const fn zero_nat_inequality<NextBits: Int, B: Bit>() -> TypeNe<Zeros, Nat<NextBits, B>> {
    typewit::type_ne!(
        <NextBits: Int, B: Bit> Zeros, Nat<NextBits, B>
    )
}



/// Diverges when given a proof of `Nat<L> == Zeros`
/// (which is a contradiction, because they're different types).
pub const fn contradiction<NextBits, B>(length_te: TypeEq<Nat<NextBits, B>, Zeros>) -> ! 
where
    NextBits: Int,
    B: Bit
{
    typewit::type_fn! {
        struct ZeroEqualsOneFn<T, U>;

        impl<NextBits: Int, B: Bit> Nat<NextBits, B> => T;
        impl Zeros => U;
    }

    length_te.map(ZeroEqualsOneFn::NEW).to_left(())
}
