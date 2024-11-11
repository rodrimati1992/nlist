use core::marker::PhantomData;

use typewit::{TypeCmp, TypeEq, TypeNe};



/// Type-level encoding of `0`
#[derive(Copy, Clone)]
pub struct Zero;

/// Type-level encoding of `+1`
pub struct PlusOne<T>(PhantomData<T>);

impl<T> Copy for PlusOne<T> {}

impl<T> Clone for PlusOne<T> {
    fn clone(&self) -> Self {
        *self
    }
}


/// Type alias form of [`PeanoInt::SubOneSat`]
pub type SubOneSat<Lhs> = <Lhs as PeanoInt>::SubOneSat;

/// Type alias form of [`PeanoInt::IfZero`]
pub type IfZero<L, Then, Else> = <L as PeanoInt>::IfZero<Then, Else>;

/// Type alias form of [`PeanoInt::IfZeroPI`]
pub type IfZeroPI<L, Then, Else> = <L as PeanoInt>::IfZeroPI<Then, Else>;

/// Type alias form of [`PeanoInt::Add`]
pub type Add<Lhs, Rhs> = <Lhs as PeanoInt>::Add<Rhs>;

/// Type alias form of [`PeanoInt::Min`]
pub type Min<Lhs, Rhs> = <Lhs as PeanoInt>::Min<Rhs>;

/// Type alias form of [`PeanoInt::Max`]
pub type Max<Lhs, Rhs> = <Lhs as PeanoInt>::Max<Rhs>;


/// Trait for peano numbers, a type-level encoding of unsigned integers.
pub trait PeanoInt: Copy + 'static {
    /// Type level equivalent of `.saturating_sub(1)`
    type SubOneSat: PeanoInt;

    #[doc(hidden)]
    type __PairOfPeanos<R: PeanoInt>: PeanoCmpWit<L = Self, R = R>;

    /// Evaluates to `Then` if `Self == Zero`, evaluates to `Else` if `Self == PlusOne<_>`
    type IfZero<Then, Else>;

    /// Variant of `IfZero` which takes and evaluates to types that impl `PeanoInt`
    type IfZeroPI<Then: PeanoInt, Else: PeanoInt>: PeanoInt;

    /// Computes the addition of `Self` and `Rhs`
    type Add<Rhs: PeanoInt>: PeanoInt;

    /// Computes the minimum of `Self` and `Rhs`
    type Min<Rhs: PeanoInt>: PeanoInt;

    /// Computes the maximum of `Self` and `Rhs`
    type Max<Rhs: PeanoInt>: PeanoInt;

    /// Constructs this type
    const NEW: Self;

    /// What integer value `Self` represents.
    const USIZE: usize;

    /// A type witness for whether `Self` is `Zero` or `PlusOne`
    const PEANO_WIT: PeanoWit<Self>;
}

impl PeanoInt for Zero {
    type SubOneSat = Zero;

    #[doc(hidden)]
    type __PairOfPeanos<R: PeanoInt> = PairOfPeanos<Self, R>;

    type IfZero<Then, Else> = Then;

    type IfZeroPI<Then: PeanoInt, Else: PeanoInt> = Then;

    type Add<Rhs: PeanoInt> = Rhs;

    type Min<Rhs: PeanoInt> = Zero;

    type Max<Rhs: PeanoInt> = Rhs;

    const NEW: Self = Zero;

    const USIZE: usize = 0;

    const PEANO_WIT: PeanoWit<Self> = PeanoWit::Zero(TypeEq::NEW);
}

impl<T> PeanoInt for PlusOne<T> 
where
    T: PeanoInt
{
    type SubOneSat = T;
    
    #[doc(hidden)]
    type __PairOfPeanos<R: PeanoInt> = PairOfPeanos<Self, R>;

    type IfZero<Then, Else> = Else;

    type IfZeroPI<Then: PeanoInt, Else: PeanoInt> = Else;

    type Add<Rhs: PeanoInt> = PlusOne<T::Add<Rhs>>;

    type Min<Rhs: PeanoInt> = Rhs::IfZeroPI<
        Zero, 
        PlusOne<T::Min<Rhs::SubOneSat>>
    >;

    type Max<Rhs: PeanoInt> = Rhs::IfZeroPI<
        Self, 
        PlusOne<T::Max<Rhs::SubOneSat>>
    >;

    const NEW: Self = PlusOne(PhantomData);
    
    const USIZE: usize = 1 + T::USIZE;

    const PEANO_WIT: PeanoWit<Self> = PeanoWit::PlusOne(TypeEq::NEW);
}

/// A type witness for whether `L` is `Zero` or `PlusOne`
pub enum PeanoWit<L: PeanoInt> {
    /// Proof that `L == PlusOne<L::SubOneSat>`
    PlusOne(TypeEq<L, PlusOne<L::SubOneSat>>),
    /// Proof that `L == Zero`
    Zero(TypeEq<L, Zero>),
}

mod peano_cmp_wit {
    use super::*;

    pub struct PairOfPeanos<L: PeanoInt, R: PeanoInt>(PhantomData<(fn() -> L, fn() -> R)>);
    
    pub trait PeanoCmpWit {
        type L: PeanoInt;
        type R: PeanoInt;

        const CMP_WIT: TypeCmp<Self::L, Self::R>;
    }

    impl<R: PeanoInt> PeanoCmpWit for PairOfPeanos<Zero, R> {
        type L = Zero;
        type R = R;

        const CMP_WIT: TypeCmp<Zero, R> = match R::PEANO_WIT {
            PeanoWit::Zero(r_te) => TypeCmp::Eq(r_te.flip()),
            PeanoWit::PlusOne(r_te) => TypeCmp::Ne(
                zero_one_inequality()
                    .join_right(r_te.flip())
            ),
        };
    }


    impl<L: PeanoInt, R: PeanoInt> PeanoCmpWit for PairOfPeanos<PlusOne<L>, R> {
        type L = PlusOne<L>;
        type R = R;

        const CMP_WIT: TypeCmp<PlusOne<L>, R> = match R::PEANO_WIT {
            PeanoWit::Zero(r_te) => TypeCmp::Ne(
                zero_one_inequality()
                    .flip()
                    .join_right(r_te.flip())
            ),
            PeanoWit::PlusOne(r_te) => {
                <L as PeanoInt>::__PairOfPeanos::<R::SubOneSat>::CMP_WIT
                    .map(PlusOneFn)
                    .join_right(r_te.flip())
            }
        };
    }
}
use peano_cmp_wit::{PairOfPeanos, PeanoCmpWit};





/// Returns a [`TypeCmp<L, R>`], which is a proof of whether `L == R` or `L != R`.
pub const fn cmp_peanos<L, R>(_: L, _: R) -> TypeCmp<L, R> 
where
    L: PeanoInt,
    R: PeanoInt,
{
    <L as PeanoInt>::__PairOfPeanos::<R>::CMP_WIT
}

const fn zero_one_inequality<L: PeanoInt>() -> TypeNe<Zero, PlusOne<L>> {
    typewit::type_ne!(<L: PeanoInt> Zero, PlusOne<L>)
}

/// Diverges when given a proof of `PlusOne<L> == Zero`
/// (which is a contradiction, because they're different types).
pub const fn contradiction<L>(length_te: TypeEq<PlusOne<L>, Zero>) -> ! {
    typewit::type_fn!{
        struct ZeroEqualsOneFn<T, U>;

        impl<L> PlusOne<L> => T;
        impl Zero => U;
    }

    length_te.map(ZeroEqualsOneFn::NEW).to_left(())
}

typewit::inj_type_fn!{
    struct PlusOneFn;

    impl<L: PeanoInt> L => PlusOne<L>;
}

