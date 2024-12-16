use core::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use typewit::TypeEq;

use super::{
    pair_of_ints::PairOfInts,
    Int, IntWit,
    Add, IfZeroI, Mul, ShlOne,
};

use crate::bit::{self, And, Bit, B0, B1, IfTrueI, Not, Xor};

use crate::tordering::{self, OrdThen, TLess, TEqual};




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
pub struct Zeros;

impl Zeros {
    integer_methods!{}
}


/// Type-level encoding of binary integers greater than or equal to 1
pub struct Nat<NextBits, B> {
    next_bits: PhantomData<fn() -> NextBits>,
    bit: PhantomData<fn() -> B>,
}

impl<NextBits: Int, B: Bit> Nat<NextBits, B> {
    integer_methods!{}
}


impl Int for Zeros {
    type BitArg = B0;
    type ShrOne = Zeros;

    #[doc(hidden)]
    type __PairOfInts<Rhs: Int> = PairOfInts<Self, Rhs>;

    type IsZeros = B1;

    type SubOneSat = Zeros;

    type SubSat<Rhs: Int> = Zeros;

    #[doc(hidden)]
    type __SubSat<Rhs: Int, Overflow: Bit> = Zeros;

    #[doc(hidden)]
    type __SubSatOverflow<Overflow: Bit> = Zeros;

    type AddOne = Nat<Zeros, B1>;

    type Add<Rhs: Int> = Rhs;

    #[doc(hidden)]
    type __Add<Rhs: Int, Overflow: Bit> = Rhs::__AddOverflow<Overflow>;

    #[doc(hidden)]
    type __AddOverflow<Overflow: Bit> = 
        bit::IfTrueI<Overflow, Nat<Zeros, Overflow>, Zeros>;

    type Mul<Rhs: Int> = Zeros;

    type Min<Rhs: Int> = Zeros;

    type Max<Rhs: Int> = Rhs;

    type IsLe<Rhs: Int> = B1;

    type Cmp<Rhs: Int> = bit::IfTrueTO<Rhs::IsZeros, TEqual, TLess>;

    const NEW: Self = Zeros;

    const USIZE: usize = 0;

    const INT_WIT: IntWit<Self> = IntWit::Zeros(TypeEq::NEW);
}

impl<NextBits: Int, B: Bit> Int for Nat<NextBits, B> {
    type BitArg = B;

    type ShrOne = NextBits;

    #[doc(hidden)]
    type __PairOfInts<Rhs: Int> = PairOfInts<Self, Rhs>;

    type IsZeros = B0;

    type SubOneSat = __Normalize<
        <NextBits as Int>::__SubSatOverflow<B>,
        B::Not,
    >;

    type SubSat<Rhs: Int> = Self::__SubSat<Rhs, B0>;

    #[doc(hidden)]
    type __SubSat<Rhs: Int, Overflow: Bit> = __Normalize<
        <NextBits as Int>::__SubSat<Rhs::ShrOne, bit::SubOverflows<B, Overflow, Rhs::BitArg>>,
        Xor<B, Xor<Overflow, Rhs::BitArg>>,
    >;

    #[doc(hidden)]
    type __SubSatOverflow<Overflow: Bit> = __Normalize<
        <NextBits as Int>::__SubSatOverflow<bit::And<Overflow, B>>,
        Xor<Overflow, B>,
    >;

    type AddOne = Nat<<NextBits as Int>::__AddOverflow<B>, B::Not>;

    type Add<Rhs: Int> = Self::__Add<Rhs, B0>;

    #[doc(hidden)]
    type __Add<Rhs: Int, Overflow: Bit> = Nat<
        <NextBits as Int>::__Add<Rhs::ShrOne, bit::AtLeastTwoB1s<B, Overflow, Rhs::BitArg>>,
        Xor<B, Xor<Overflow, Rhs::BitArg>>,
    >;

    #[doc(hidden)]
    type __AddOverflow<Overflow: Bit> = 
        Nat<<NextBits as Int>::__AddOverflow<bit::And<Overflow, B>>, Xor<Overflow, B>>;

    type Mul<Rhs: Int> = Add<
        Mul<NextBits, ShlOne<Rhs>>, 
        IfTrueI<B, Rhs, Zeros>
    >;

    type Min<Rhs: Int> = IfZeroI<
        Rhs, 
        Zeros, 
        __Naturalize<IfTrueI<Self::IsLe<Rhs>, Self, Rhs>>,
    >;

    type Max<Rhs: Int> = __Naturalize<IfTrueI<Self::IsLe<Rhs>, Rhs, Self>>;

    type IsLe<Rhs: Int> = tordering::IsLess<Self::Cmp<Rhs>>;

    type Cmp<Rhs: Int> = OrdThen<
        super::Cmp<NextBits, Rhs::ShrOne>,
        bit::BitCmp<B, Rhs::BitArg>,
    >;

    const NEW: Self = {
        assert_normalized::<NextBits, B>();

        Nat {
            next_bits: PhantomData,
            bit: PhantomData,
        }
    };

    const USIZE: usize = {
        assert_normalized::<NextBits, B>();

        (NextBits::USIZE << 1) + B::BOOL as usize
    };

    const INT_WIT: IntWit<Self> = {
        assert_normalized::<NextBits, B>();

        IntWit::Nat(TypeEq::NEW)
    };
}

// normalizes a Nat so that instead of evaluating to
//     Nat<Nat<Zeros, B0>, B1>
// it evaluates to
//     Nat<Zeros, B1>
// (the top bit must always be B1)
type __Normalize<NextBits, B> =
    bit::IfTrueI<
        And<super::IsZeros<NextBits>, Not<B>>,
        Zeros,
        Nat<NextBits, B>,
    >;

// Assumes that N is a Nat
type __Naturalize<N> = Nat<<N as Int>::ShrOne, <N as Int>::BitArg>;


typewit::type_fn! {
    struct ConstIFn<N: Int>;

    impl () => N
}



#[track_caller]
const fn assert_normalized<NextBits: Int, B: Bit>() {
    if <NextBits::IsZeros as Bit>::BOOL && !B::BOOL {
        panic!("denormalized integer detected")
    }
}



//////////////////////////////////////////////////////////////
//          formatting impls
//////////////////////////////////////////////////////////////

macro_rules! delegate_fmt_trait {
    ($trait:ident) => {
        impl fmt::$trait for Zeros {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::$trait::fmt(&0, fmt)
            }
        }

        impl<NextBits: Int, B: Bit> fmt::$trait for Nat<NextBits, B> {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                <_ as fmt::$trait>::fmt(&Self::USIZE, fmt)
            }
        }
    }
}

delegate_fmt_trait!{Binary}
delegate_fmt_trait!{Debug}
delegate_fmt_trait!{Display}
delegate_fmt_trait!{LowerHex}
delegate_fmt_trait!{Octal}
delegate_fmt_trait!{UpperHex}


//////////////////////////////////////////////////////////////
//          comparison impls
//////////////////////////////////////////////////////////////


impl<Rhs: Int> PartialEq<Rhs> for Zeros {
    fn eq(&self, _: &Rhs) -> bool {
        0 == Rhs::USIZE
    }
}

impl PartialEq<usize> for Zeros {
    fn eq(&self, rhs: &usize) -> bool {
        0 == *rhs
    }
}

impl Eq for Zeros {}

impl<Rhs: Int> PartialOrd<Rhs> for Zeros {
    fn partial_cmp(&self, _: &Rhs) -> Option<Ordering> {
        0.partial_cmp(&Rhs::USIZE)
    }
}

impl PartialOrd<usize> for Zeros {
    fn partial_cmp(&self, rhs: &usize) -> Option<Ordering> {
        0.partial_cmp(rhs)
    }
}

impl Ord for Zeros {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

///////////////

impl<NextBits: Int, B: Bit, Rhs: Int> PartialEq<Rhs> for Nat<NextBits, B> {
    fn eq(&self, _: &Rhs) -> bool {
        Self::USIZE == Rhs::USIZE
    }
}

impl<NextBits: Int, B: Bit> PartialEq<usize> for Nat<NextBits, B> {
    fn eq(&self, rhs: &usize) -> bool {
        Self::USIZE == *rhs
    }
}

impl<NextBits: Int, B: Bit> Eq for Nat<NextBits, B> {}

impl<NextBits: Int, B: Bit, Rhs: Int> PartialOrd<Rhs> for Nat<NextBits, B> {
    fn partial_cmp(&self, _: &Rhs) -> Option<Ordering> {
        Self::USIZE.partial_cmp(&Rhs::USIZE)
    }
}

impl<NextBits: Int, B: Bit> PartialOrd<usize> for Nat<NextBits, B> {
    fn partial_cmp(&self, rhs: &usize) -> Option<Ordering> {
        Self::USIZE.partial_cmp(rhs)
    }
}

impl<NextBits: Int, B: Bit> Ord for Nat<NextBits, B> {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

//////////////////////////////////////////////////////////////

impl Hash for Zeros {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        0usize.hash(hasher)
    }
}

impl<NextBits: Int, B: Bit> Hash for Nat<NextBits, B> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        Self::USIZE.hash(hasher)
    }
}


impl Default for Zeros {
    fn default() -> Self {
        Zeros
    }
}

impl<NextBits: Int, B: Bit> Default for Nat<NextBits, B> {
    fn default() -> Self {
        Self::NEW
    }
}


impl<NextBits: Int, B: Bit> Copy for Nat<NextBits, B> {}

impl<NextBits: Int, B: Bit> Clone for Nat<NextBits, B> {
    fn clone(&self) -> Self {
        *self
    }
}


