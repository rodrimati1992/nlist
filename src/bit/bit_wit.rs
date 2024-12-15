use core::fmt::{self, Debug};

use typewit::{TypeEq, TypeWitnessTypeArg, MakeTypeWitness};

use super::{B0, B1};

/// Type witness that `B` is either [`B1`] or [`B0`].
pub enum BitWit<B> {
    /// Witnesses that `B == B1`
    B1(TypeEq<B, B1>),
    /// Witnesses that `B == B0`
    B0(TypeEq<B, B0>),
}

impl<B> Copy for BitWit<B> {}

impl<B> Clone for BitWit<B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B> Debug for BitWit<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::B1{..} => "B1",
            Self::B0{..} => "B0",
        })
    }
}

impl<B> TypeWitnessTypeArg for BitWit<B> {
    type Arg = B;
}

impl MakeTypeWitness for BitWit<B0> {
    const MAKE: Self = BitWit::B0(TypeEq::NEW);
}

impl MakeTypeWitness for BitWit<B1> {
    const MAKE: Self = BitWit::B1(TypeEq::NEW);
}



