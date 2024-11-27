use nlist::{Peano, peano};
use nlist::peano::{IntoPeano, PeanoInt, PeanoWit, FromUsize, IntoUsize, PlusOne, Usize, Zero};

use crate::misc_tests::test_utils::{assertm, assert_type, assert_type_eq};


#[test]
fn peano_repr_test() {
    macro_rules! test_cases {
        ($(($n:literal $ty:ty))*) => ($({
            assert_type::<$ty>(peano!($n));
            assert_type_eq::<Peano!($n), $ty>();
        })*)
    }

    test_cases! {
        (0 Zero)
        (1 PlusOne<Zero>)
        (2 PlusOne<PlusOne<Zero>>)
        (3 PlusOne<PlusOne<PlusOne<Zero>>>)
        (4 PlusOne<PlusOne<PlusOne<PlusOne<Zero>>>>)
    }
}

#[test]
fn peano_wit_test() {
    assertm!(<Peano!(0)>::PEANO_WIT, PeanoWit::Zero{..});
    assertm!(<Peano!(1)>::PEANO_WIT, PeanoWit::PlusOne{..});
    assertm!(<Peano!(2)>::PEANO_WIT, PeanoWit::PlusOne{..});
    assertm!(<Peano!(3)>::PEANO_WIT, PeanoWit::PlusOne{..});
    assertm!(<Peano!(4)>::PEANO_WIT, PeanoWit::PlusOne{..});
}


#[test]
fn peano_value_test() {
    const fn to_usize<L, const U: usize>(_peano: L) -> usize
    where
        L: PeanoInt + IntoUsize<Usize = Usize<U>>
    {
        U
    }

    const fn to_peano<const U: usize>() -> FromUsize<U>
    where
        Usize<U>: IntoPeano
    {
        PeanoInt::NEW
    }


    macro_rules! test_cases {
        ($($n:literal)*) => ($({
            assert_eq!(to_usize(peano!($n)), $n);
            assert_eq!(<Peano!($n) as PeanoInt>::USIZE, $n);
            assert_eq!(to_peano::<$n>(), peano!($n));
        })*)
    }

    test_cases!{
        0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 
        16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 
        32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 
        48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 
        64 
    }
}
