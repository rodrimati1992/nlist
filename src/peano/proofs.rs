//! Contains proofs of arithmetic properties of [`PeanoInt`]s

use super::*;

#[doc(hidden)]
pub trait PeanoCmpProofs: PeanoCmpWit {
    const COMM_ADD: TypeEq<Add<Self::L, Self::R>, Add<Self::R, Self::L>>;

    const COMM_ADD1: TypeEq<Add<PlusOne<Self::L>, Self::R>, Add<Self::L, PlusOne<Self::R>>>;
    
    const COMM_MUL: TypeEq<Mul<Self::L, Self::R>, Mul<Self::R, Self::L>>;
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


    const COMM_MUL: TypeEq<Mul<Zero, R>, Mul<R, Zero>> = 
        match R::PEANO_WIT {
            PeanoWit::Zero(te) => te.flip().zip(te).map(MulFn),
            PeanoWit::PlusOne(te) => {
                te
                .map(MulProofFn::NEW)
                .to_left(PairOfPeanos_::<Zero, R::SubOneSat>::COMM_MUL
                    .join(add_identity().flip())
                )
            }
        };

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

    const COMM_ADD1: TypeEq<Add<PlusOne<PlusOne<L>>, R>, Add<PlusOne<L>, PlusOne<R>>> = 
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

    const COMM_MUL: TypeEq<Mul<PlusOne<L>, R>, Mul<R, PlusOne<L>>> = match R::PEANO_WIT {
        PeanoWit::Zero(te) => te
            .map(MulProofFn::NEW)
            .to_left(PairOfPeanos_::<Zero, PlusOne<L>>::COMM_MUL.flip()),
        PeanoWit::PlusOne(_) => {
            // while this works, 
            // it'd be way better if it was done by combining arithmetic properties
            super::eq(
                Mul::<PlusOne<L>, R>::NEW,
                Mul::<R, PlusOne<L>>::NEW,
            ).unwrap_eq()
        }
    };
}



typewit::type_fn!{
    struct AddProofFn<L: PeanoInt>;

    impl<R: PeanoInt> R => TypeEq<Add<L, R>, Add<R, L>>
}

typewit::type_fn!{
    struct MulProofFn<L: PeanoInt>;

    impl<R: PeanoInt> R => TypeEq<Mul<L, R>, Mul<R, L>>
}

typewit::type_fn!{
    struct DistMulProofFn<L: PeanoInt>;

    impl<R: PeanoInt> R => TypeEq<Mul<L, PlusOne<R>>, Add<Mul<L, R>, L>>
}

typewit::type_fn!{
    struct AddOneProofFn<L: PeanoInt>;

    impl<R: PeanoInt> R => TypeEq<Add<PlusOne<L>, R>, Add<L, PlusOne<R>>>
}

typewit::type_fn!{
    struct AddCaptureFn<R: PeanoInt>;

    impl<L: PeanoInt> L => Add<L, R>
}

/// Proof that `L + R` == `R + L`
pub const fn commutative_add<L, R>() -> TypeEq<Add<L, R>, Add<R, L>>
where
    L: PeanoInt,
    R: PeanoInt,
{
    PairOfPeanos_::<L, R>::COMM_ADD
}

/// Proof that `L * R` == `R * L`
pub const fn commutative_mul<L, R>() -> TypeEq<Mul<L, R>, Mul<R, L>>
where
    L: PeanoInt,
    R: PeanoInt,
{
    PairOfPeanos_::<L, R>::COMM_MUL
}

/// Proof that `L + 0` == `L`
pub const fn add_identity<L>() -> TypeEq<Add<L, Zero>, L>
where
    L: PeanoInt,
{
    PairOfPeanos_::<Zero, L>::COMM_ADD
        .flip()
}


