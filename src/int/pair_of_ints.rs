use super::*;

pub struct PairOfInts<L: Int, R: Int>(PhantomData<(fn() -> L, fn() -> R)>);

pub trait IntCmpHelper {
    type L: Int;
    type R: Int;

    const EQ_WIT: TypeCmp<Self::L, Self::R>;
}

pub type PairOfInts_<L, R> = <L as Int>::__PairOfInts<R>;


impl<R: Int> IntCmpHelper for PairOfInts<Zeros, R> {
    type L = Zeros;
    type R = R;

    const EQ_WIT: TypeCmp<Zeros, R> = match R::INT_WIT {
        IntWit::Zeros(r_te) => TypeCmp::Eq(r_te.flip()),
        IntWit::Nat(r_te) => TypeCmp::Ne(zero_nat_inequality().join_right(r_te.flip())),
    };
}

impl<NextBits: Int, B: Bit, R: Int> IntCmpHelper for PairOfInts<Nat<NextBits, B>, R> {
    type L = Nat<NextBits, B>;
    type R = R;

    const EQ_WIT: TypeCmp<Nat<NextBits, B>, R> = match R::INT_WIT {
        IntWit::Zeros(r_te) => {
            TypeCmp::Ne(zero_nat_inequality().flip().join_right(r_te.flip()))
        }
        IntWit::Nat(r_te) => {
            PairOfInts_::<NextBits, R::ShrOne>::EQ_WIT
            .zip(bit::eq::<B, R::BitArg>())
            .map(NatFn)
            .join_right(r_te.flip())
        }
    };
}


typewit::inj_type_fn! {
    struct NatFn;

    impl<NextBits: Int, B: Bit> (NextBits, B) => Nat<NextBits, B>;
}


