use super::*;

pub struct PairOfPeanos<L: PeanoInt, R: PeanoInt>(PhantomData<(fn() -> L, fn() -> R)>);

pub trait PeanoCmpWit {
    type L: PeanoInt;
    type R: PeanoInt;

    const EQ_WIT: TypeCmp<Self::L, Self::R>;
}

pub type PairOfPeanos_<L, R> = <L as PeanoInt>::__PairOfPeanos<R>;


impl<R: PeanoInt> PeanoCmpWit for PairOfPeanos<Zero, R> {
    type L = Zero;
    type R = R;

    const EQ_WIT: TypeCmp<Zero, R> = match R::PEANO_WIT {
        PeanoWit::Zero(r_te) => TypeCmp::Eq(r_te.flip()),
        PeanoWit::PlusOne(r_te) => TypeCmp::Ne(zero_one_inequality().join_right(r_te.flip())),
    };
}

impl<L: PeanoInt, R: PeanoInt> PeanoCmpWit for PairOfPeanos<PlusOne<L>, R> {
    type L = PlusOne<L>;
    type R = R;

    const EQ_WIT: TypeCmp<PlusOne<L>, R> = match R::PEANO_WIT {
        PeanoWit::Zero(r_te) => {
            TypeCmp::Ne(zero_one_inequality().flip().join_right(r_te.flip()))
        }
        PeanoWit::PlusOne(r_te) => PairOfPeanos_::<L, R::SubOneSat>::EQ_WIT
            .map(PlusOneFn)
            .join_right(r_te.flip()),
    };
}


