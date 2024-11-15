//! Contains proofs of arithmetic properties of [`PeanoInt`]s

use super::*;

#[doc(hidden)]
pub trait PeanoCmpProofs: PeanoCmpWit {
    const COMM_ADD: TypeEq<Add<Self::L, Self::R>, Add<Self::R, Self::L>>;

    const COMM_ADD1: TypeEq<Add<PlusOne<Self::L>, Self::R>, Add<Self::L, PlusOne<Self::R>>>;
}

#[doc(hidden)]
pub type PairOfPeanos_<L, R> = <L as PeanoInt>::__PairOfPeanosProofs<R>;


impl<R: PeanoInt> PeanoCmpProofs for PairOfPeanos<Zero, R> {
    const COMM_ADD: TypeEq<Add<Zero, R>, Add<R, Zero>> = match R::PEANO_WIT {
        PeanoWit::Zero(te) => te.flip().zip(te).map(AddFn),
        PeanoWit::PlusOne(te) => {
            let te_sub = PairOfPeanos_::<Zero, R::SubOneSat>::COMM_ADD.map(PlusOneFn);

            let te_sub = PairOfPeanos_::<Zero, R::SubOneSat>::COMM_ADD1.flip().join(te_sub);

            te.map(AddProofFn::NEW).to_left(te_sub)
        }
    };

    const COMM_ADD1: TypeEq<Add<PlusOne<Zero>, Self::R>, Add<Zero, PlusOne<Self::R>>> = 
        TypeEq::NEW;
}

impl<L: PeanoInt, R: PeanoInt> PeanoCmpProofs for PairOfPeanos<PlusOne<L>, R> {
    const COMM_ADD: TypeEq<Add<PlusOne<L>, R>, Add<R, PlusOne<L>>> = match R::PEANO_WIT {
        PeanoWit::Zero(te) => 
            te.map(AddProofFn::NEW).to_left(
                PairOfPeanos_::<L, Zero>::COMM_ADD.map(PlusOneFn)
            ),
        PeanoWit::PlusOne(te) => {
            let te_sub = PairOfPeanos_::<PlusOne<L>, R::SubOneSat>::COMM_ADD;
            let te_sub = PairOfPeanos_::<L, R::SubOneSat>::COMM_ADD1
                .flip()
                .join(te_sub)
                .map(PlusOneFn);

            te.map(AddProofFn::NEW).to_left(te_sub)
        }

    };

    const COMM_ADD1: TypeEq<Add<PlusOne<PlusOne<L>>, Self::R>, Add<PlusOne<L>, PlusOne<Self::R>>> = 
        match R::PEANO_WIT {
            PeanoWit::Zero(te) => {
                te.map(AddOneProofFn::<PlusOne<L>>::NEW).to_left(
                    PairOfPeanos_::<L, Zero>::COMM_ADD1
                        .map(PlusOneFn)
                )
            }
            PeanoWit::PlusOne(te) => {
                te.map(AddOneProofFn::<PlusOne<L>>::NEW).to_left(
                    PairOfPeanos_::<L, PlusOne<R::SubOneSat>>::COMM_ADD1
                        .map(PlusOneFn)
                )
            }
        };
}



typewit::type_fn!{
    struct AddProofFn<L: PeanoInt>;

    impl<R: PeanoInt> R => TypeEq<Add<L, R>, Add<R, L>>
}

typewit::type_fn!{
    struct AddOneProofFn<L: PeanoInt>;

    impl<R: PeanoInt> R => TypeEq<Add<PlusOne<L>, R>, Add<L, PlusOne<R>>>
        
}

/// Proof that `L + R` == `R + L`
pub const fn commutative_add<L, R>() -> TypeEq<Add<L, R>, Add<R, L>>
where
    L: PeanoInt,
    R: PeanoInt,
{
    PairOfPeanos_::<L, R>::COMM_ADD
}









