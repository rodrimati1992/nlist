//! Type-level integers which use a unary representation
//! 
//! 

use core::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    fmt,
    hash::Hash,
    marker::PhantomData,
};


use typewit::{TypeCmp, TypeEq, TypeNe};


use crate::boolean::{Bool, BoolWitG, Boolean, And, Not};

///////////////////////////////////////////////////////////////////////////////

#[doc(no_inline)]
pub use crate::{Peano, peano};

mod peano_wit;

pub use peano_wit::PeanoWit;

mod from_const;

/// [`typewit::TypeFn`] equivalents of peano type aliases
pub mod type_fns;

#[doc(no_inline)]
pub use self::type_fns::*;


#[doc(no_inline)]
pub use typewit::const_marker::Usize;

pub use self::from_const::{FromPeano, FromUsize, IntoPeano, IntoUsize};

///////////////////////////////////////////////////////////////////////////////

macro_rules! integer_methods {
    () => (
        /// The usize value of this integer
        pub const fn usize(self) -> usize {
            Self::USIZE
        }
    )
}



/// Type-level encoding of `0`
#[derive(Copy, Clone)]
pub struct Zero;

impl Zero {
    integer_methods!{}
}


/// Type-level encoding of `T + 1`
pub struct PlusOne<T> {
    /// `Self - 1`
    pub sub_one: T,
}

impl<T: PeanoInt> PlusOne<T> {
    integer_methods!{}
}

// The impls of std traits for Zero and PlusOne are all here
mod std_impls;

///////////////////////////////////////////////////////////////////////////////

/// Type alias form of [`PeanoInt::SubOneSat`]
pub type SubOneSat<Lhs> = <Lhs as PeanoInt>::SubOneSat;

/// Type alias form of [`PeanoInt::IsZero`] chained with [`Boolean::IfTrue`]
pub type IfZero<L, Then, Else> = <IsZero<L> as Boolean>::IfTrue<Then, Else>;

/// Type alias form of [`PeanoInt::IsZero`] chained with [`Boolean::IfTruePI`]
pub type IfZeroPI<L, Then, Else> = <IsZero<L> as Boolean>::IfTruePI<Then, Else>;

/// Type alias form of [`PeanoInt::IsZero`]
pub type IsZero<Lhs> = <Lhs as PeanoInt>::IsZero;

/// Type alias form of [`PeanoInt::SubSat`]
pub type SubSat<Lhs, Rhs> = <Lhs as PeanoInt>::SubSat<Rhs>;

/// Type alias form of [`PeanoInt::Add`]
pub type Add<Lhs, Rhs> = <Lhs as PeanoInt>::Add<Rhs>;

/// Type alias form of [`PeanoInt::Mul`]
pub type Mul<Lhs, Rhs> = <Lhs as PeanoInt>::Mul<Rhs>;

/// Type alias form of [`PeanoInt::Min`]
pub type Min<Lhs, Rhs> = <Lhs as PeanoInt>::Min<Rhs>;

/// Type alias form of [`PeanoInt::Max`]
pub type Max<Lhs, Rhs> = <Lhs as PeanoInt>::Max<Rhs>;

/// Type alias form of [`PeanoInt::IsLe`]
pub type IsLe<Lhs, Rhs> = <Lhs as PeanoInt>::IsLe<Rhs>;

/// Type alias form of [`PeanoInt::IsLt`]
pub type IsLt<Lhs, Rhs> = <Lhs as PeanoInt>::IsLt<Rhs>;



/// Trait for peano numbers, a type-level encoding of unsigned integers.
/// 
/// Only [`Zero`] and [`PlusOne`] implement this trait,
/// no other type can implement it.
/// 
/// # Example
/// 
/// Constructing a tuple `L`-levels deep with a recursive function.
/// 
/// ```rust
/// use nlist::{Peano, peano};
/// use nlist::peano::{PeanoInt, PeanoWit, PlusOne, Zero};
/// use nlist::typewit::{CallFn, type_fn};
/// 
/// 
/// assert_eq!(recursive::<Peano!(0)>(), ());
/// assert_eq!(recursive::<Peano!(1)>(), (1, ()));
/// assert_eq!(recursive::<Peano!(2)>(), (2, (1, ())));
/// assert_eq!(recursive::<Peano!(3)>(), (3, (2, (1, ()))));
/// 
/// 
/// // The `-> CallFn<PeanoToTupleFn<usize>, L>` return type 
/// // calls the `PeanoToTupleFn<usize>` type-level function with `L` as an argument.
/// const fn recursive<L: PeanoToTuple>() -> CallFn<PeanoToTupleFn<usize>, L> {
///     match L::PEANO_WIT {
///         PeanoWit::PlusOne(len_te) => {
///             len_te.project::<PeanoToTupleFn<usize>>()
///                 .to_left((L::USIZE, recursive::<L::SubOneSat>()))
///         }
///         PeanoWit::Zero(len_te) => {
///             len_te.project::<PeanoToTupleFn<usize>>().to_left(())
///         }
///     }
/// }
/// 
/// type_fn! {
///     // `PeanoToTupleFn<T>` is a type-level function (`typewit::TypeFn` implementor) 
///     // from `L` to <L as PeanoToTuple>::Output::<T>
///     struct PeanoToTupleFn<T>;
/// 
///     impl<L: PeanoToTuple> L => L::Output::<T>;
/// }
/// 
/// trait PeanoToTuple: PeanoInt<SubOneSat = Self::SubOneSat_> {
///     type SubOneSat_: PeanoToTuple;
///     type Output<T>;
/// }
/// 
/// impl PeanoToTuple for Zero {
///     type SubOneSat_ = Zero;
///     type Output<T> = ();
/// }
/// 
/// impl<L: PeanoToTuple> PeanoToTuple for PlusOne<L> {
///     type SubOneSat_ = L;
///     type Output<T> = (T, L::Output<T>);
/// }
/// ```
/// 
pub trait PeanoInt: 
    Sized + Copy + Default + Hash + Sync + Send +
    Eq + Ord + PartialEq + PartialEq<usize> + PartialOrd + PartialOrd<usize> +
    fmt::Binary + fmt::Debug + fmt::Display + fmt::LowerHex + fmt::Octal + fmt::UpperHex +
    'static 
{
    /// Type level equivalent of `.saturating_sub(1)`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{PeanoInt, Peano, peano};
    /// 
    /// assert_eq!(peano::SubOneSat::<Peano!(0)>::NEW, 0);
    /// assert_eq!(peano::SubOneSat::<Peano!(1)>::NEW, 0);
    /// assert_eq!(peano::SubOneSat::<Peano!(2)>::NEW, 1);
    /// assert_eq!(peano::SubOneSat::<Peano!(3)>::NEW, 2);
    /// 
    /// ```
    type SubOneSat: PeanoInt;

    #[doc(hidden)]
    type __PairOfPeanos<R: PeanoInt>: PeanoCmpWit<L = Self, R = R>;

    /// Whether `Self` is Zero
    type IsZero: Boolean;

    /// Type level equivalent of `.saturating_sub(R)`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{PeanoInt, Peano, peano};
    /// 
    /// const fn subsat<Rhs: PeanoInt>() -> impl PeanoInt {
    ///     peano::SubSat::<Peano!(3), Rhs>::NEW
    /// }
    /// 
    /// assert_eq!(subsat::<Peano!(0)>(), 3);
    /// assert_eq!(subsat::<Peano!(1)>(), 2);
    /// assert_eq!(subsat::<Peano!(2)>(), 1);
    /// assert_eq!(subsat::<Peano!(3)>(), 0);
    /// assert_eq!(subsat::<Peano!(4)>(), 0);
    /// ```
    type SubSat<R: PeanoInt>: PeanoInt;

    /// Computes the addition of `Self` and `Rhs`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{PeanoInt, Peano, peano};
    /// 
    /// assert_eq!(peano::Add::<Peano!(0), Peano!(0)>::NEW, 0);
    /// assert_eq!(peano::Add::<Peano!(0), Peano!(1)>::NEW, 1);
    /// assert_eq!(peano::Add::<Peano!(0), Peano!(2)>::NEW, 2);
    /// 
    /// assert_eq!(peano::Add::<Peano!(1), Peano!(0)>::NEW, 1);
    /// assert_eq!(peano::Add::<Peano!(1), Peano!(1)>::NEW, 2);
    /// assert_eq!(peano::Add::<Peano!(1), Peano!(2)>::NEW, 3);
    /// 
    /// assert_eq!(peano::Add::<Peano!(2), Peano!(0)>::NEW, 2);
    /// assert_eq!(peano::Add::<Peano!(2), Peano!(1)>::NEW, 3);
    /// assert_eq!(peano::Add::<Peano!(2), Peano!(2)>::NEW, 4);
    /// 
    /// ```
    type Add<Rhs: PeanoInt>: PeanoInt;

    /// Computes `Self` multiplied by `Rhs`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{PeanoInt, Peano, peano};
    /// 
    /// assert_eq!(peano::Mul::<Peano!(0), Peano!(0)>::NEW, 0);
    /// assert_eq!(peano::Mul::<Peano!(0), Peano!(1)>::NEW, 0);
    /// 
    /// assert_eq!(peano::Mul::<Peano!(1), Peano!(0)>::NEW, 0);
    /// assert_eq!(peano::Mul::<Peano!(1), Peano!(1)>::NEW, 1);
    /// assert_eq!(peano::Mul::<Peano!(1), Peano!(2)>::NEW, 2);
    /// 
    /// assert_eq!(peano::Mul::<Peano!(2), Peano!(1)>::NEW, 2);
    /// assert_eq!(peano::Mul::<Peano!(2), Peano!(2)>::NEW, 4);
    /// assert_eq!(peano::Mul::<Peano!(2), Peano!(3)>::NEW, 6);
    /// 
    /// ```
    type Mul<Rhs: PeanoInt>: PeanoInt;

    /// Computes the minimum of `Self` and `Rhs`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{PeanoInt, Peano, peano};
    /// 
    /// assert_eq!(peano::Min::<Peano!(0), Peano!(0)>::NEW, 0);
    /// assert_eq!(peano::Min::<Peano!(0), Peano!(1)>::NEW, 0);
    /// assert_eq!(peano::Min::<Peano!(0), Peano!(2)>::NEW, 0);
    /// 
    /// assert_eq!(peano::Min::<Peano!(1), Peano!(0)>::NEW, 0);
    /// assert_eq!(peano::Min::<Peano!(1), Peano!(1)>::NEW, 1);
    /// assert_eq!(peano::Min::<Peano!(1), Peano!(2)>::NEW, 1);
    /// 
    /// assert_eq!(peano::Min::<Peano!(2), Peano!(0)>::NEW, 0);
    /// assert_eq!(peano::Min::<Peano!(2), Peano!(1)>::NEW, 1);
    /// assert_eq!(peano::Min::<Peano!(2), Peano!(2)>::NEW, 2);
    /// 
    /// ```
    type Min<Rhs: PeanoInt>: PeanoInt;

    /// Computes the maximum of `Self` and `Rhs`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{PeanoInt, Peano, peano};
    /// 
    /// assert_eq!(peano::Max::<Peano!(0), Peano!(0)>::NEW, 0);
    /// assert_eq!(peano::Max::<Peano!(0), Peano!(1)>::NEW, 1);
    /// assert_eq!(peano::Max::<Peano!(0), Peano!(2)>::NEW, 2);
    /// 
    /// assert_eq!(peano::Max::<Peano!(1), Peano!(0)>::NEW, 1);
    /// assert_eq!(peano::Max::<Peano!(1), Peano!(1)>::NEW, 1);
    /// assert_eq!(peano::Max::<Peano!(1), Peano!(2)>::NEW, 2);
    /// 
    /// assert_eq!(peano::Max::<Peano!(2), Peano!(0)>::NEW, 2);
    /// assert_eq!(peano::Max::<Peano!(2), Peano!(1)>::NEW, 2);
    /// assert_eq!(peano::Max::<Peano!(2), Peano!(2)>::NEW, 2);
    /// 
    /// ```
    type Max<Rhs: PeanoInt>: PeanoInt;

    /// Whether `Self < Rhs`
    type IsLt<Rhs: PeanoInt>: Boolean;

    /// Whether `Self <= Rhs`
    type IsLe<Rhs: PeanoInt>: Boolean;

    /// Constructs this type
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{PeanoInt, Peano, peano};
    /// 
    /// let int = <Peano!(2)>::NEW;
    /// 
    /// assert_eq!(int, 2);
    /// ```
    const NEW: Self;

    /// What integer value `Self` represents.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{PeanoInt, Peano, peano};
    /// 
    /// assert_eq!(<Peano!(3)>::USIZE, 3);
    /// assert_eq!(<Peano!(5)>::USIZE, 5);
    /// ```
    const USIZE: usize;

    /// A type witness for whether `Self` is `Zero` or `PlusOne`
    /// 
    /// For an example, you can look at the docs of [`PeanoWit`] itself
    /// 
    const PEANO_WIT: PeanoWit<Self>;
}

impl PeanoInt for Zero {
    type SubOneSat = Zero;

    #[doc(hidden)]
    type __PairOfPeanos<R: PeanoInt> = PairOfPeanos<Self, R>;

    type IsZero = Bool<true>;

    type SubSat<R: PeanoInt> = Zero;

    type Add<Rhs: PeanoInt> = Rhs;

    type Mul<Rhs: PeanoInt> = Zero;

    type Min<Rhs: PeanoInt> = Zero;

    type Max<Rhs: PeanoInt> = Rhs;

    type IsLt<Rhs: PeanoInt> = Not<Rhs::IsZero>;
    
    type IsLe<Rhs: PeanoInt> = Bool<true>;

    const NEW: Self = Zero;

    const USIZE: usize = 0;

    const PEANO_WIT: PeanoWit<Self> = PeanoWit::Zero(TypeEq::NEW);
}

impl<T> PeanoInt for PlusOne<T>
where
    T: PeanoInt,
{
    type SubOneSat = T;

    #[doc(hidden)]
    type __PairOfPeanos<R: PeanoInt> = PairOfPeanos<Self, R>;

    type IsZero = Bool<false>;

    type SubSat<R: PeanoInt> = IfZeroPI<R, Self, T::SubSat<R::SubOneSat>>;

    type Add<Rhs: PeanoInt> = PlusOne<T::Add<Rhs>>;

    type Mul<Rhs: PeanoInt> = Add<Mul<T, Rhs>, Rhs>;

    type Min<Rhs: PeanoInt> = IfZeroPI<Rhs, Zero, PlusOne<T::Min<Rhs::SubOneSat>>>;

    type Max<Rhs: PeanoInt> = PlusOne<IfZeroPI<Rhs, T, T::Max<Rhs::SubOneSat>>>;

    type IsLt<Rhs: PeanoInt> = And<Not<Rhs::IsZero>, T::IsLt<Rhs::SubOneSat>>;

    type IsLe<Rhs: PeanoInt> = And<Not<Rhs::IsZero>, T::IsLe<Rhs::SubOneSat>>;

    const NEW: Self = PlusOne { sub_one: T::NEW };

    const USIZE: usize = 1 + T::USIZE;

    const PEANO_WIT: PeanoWit<Self> = PeanoWit::PlusOne(TypeEq::NEW);
}

mod pair_of_peanos;

use self::pair_of_peanos::{PairOfPeanos, PeanoCmpWit, PairOfPeanos_};

pub mod proofs;



////////////////////////////////////////////////////////////////////////////////

/// Converts the peano integer to a usize
pub const fn to_usize<I: PeanoInt>(_: I) -> usize {
    I::USIZE
}

////////////////////////////////////////////////////////////////////////////////


/// Returns a [`TypeCmp<L, R>`], which is a proof of whether `L == R` or `L != R`.
pub const fn eq<L, R>() -> TypeCmp<L, R>
where
    L: PeanoInt,
    R: PeanoInt,
{
    PairOfPeanos_::<L, R>::EQ_WIT
}

/// Returns a [`BoolWitG`] proof of whether `L <= R` is true.
pub const fn check_le<L, R>() -> BoolWitG<L::IsLe<R>>
where
    L: PeanoInt,
    R: PeanoInt,
{
    Boolean::BOOL_WIT
}



const fn zero_one_inequality<L: PeanoInt>() -> TypeNe<Zero, PlusOne<L>> {
    typewit::type_ne!(<L: PeanoInt> Zero, PlusOne<L>)
}



typewit::inj_type_fn! {
    struct PlusOneFn;

    impl<L: PeanoInt> L => PlusOne<L>;
}



/// Diverges when given a proof of `PlusOne<L> == Zero`
/// (which is a contradiction, because they're different types).
pub const fn contradiction<L>(length_te: TypeEq<PlusOne<L>, Zero>) -> ! {
    typewit::type_fn! {
        struct ZeroEqualsOneFn<T, U>;

        impl<L> PlusOne<L> => T;
        impl Zero => U;
    }

    length_te.map(ZeroEqualsOneFn::NEW).to_left(())
}
