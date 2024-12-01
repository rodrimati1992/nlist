use nlist::{Peano, PeanoInt, peano};
use nlist::boolean::{Bool, Boolean};
use nlist::typewit::{CallFn, Identity};

use crate::misc_tests::test_utils::{assert_type_eq, test_op, test_nonassoc_op};




#[test]
fn sub_one_sat_test() {
    test_op! {
        PeanoInt::SubOneSat<> SubOneSat SubOneSatFn, PeanoInt -> PeanoInt, peano =>
        (Peano!(0) => Peano!(0))
        (Peano!(1) => Peano!(0))
        (Peano!(2) => Peano!(1))
        (Peano!(3) => Peano!(2))
    }
}

#[test]
fn is_zero_test() {
    test_op! {
        PeanoInt::IsZero<> IsZero IsZeroFn, PeanoInt -> Boolean, peano => 
        (Peano!(0) => Bool<true>)
        (Peano!(1) => Bool<false>)
        (Peano!(2) => Bool<false>)
        (Peano!(3) => Bool<false>)
    }
}

#[test]
fn sub_sat_test() {
    test_op! {
        PeanoInt::SubSat<Rhs> SubSat SubSatFn, PeanoInt -> PeanoInt, peano => 
        (Peano!(0), Peano!(0) => Peano!(0))
        (Peano!(0), Peano!(1) => Peano!(0))
        (Peano!(0), Peano!(2) => Peano!(0))
        (Peano!(0), Peano!(3) => Peano!(0))

        (Peano!(1), Peano!(0) => Peano!(1))
        (Peano!(1), Peano!(1) => Peano!(0))
        (Peano!(1), Peano!(2) => Peano!(0))
        (Peano!(1), Peano!(3) => Peano!(0))

        (Peano!(2), Peano!(0) => Peano!(2))
        (Peano!(2), Peano!(1) => Peano!(1))
        (Peano!(2), Peano!(2) => Peano!(0))
        (Peano!(2), Peano!(3) => Peano!(0))

        (Peano!(3), Peano!(0) => Peano!(3))
        (Peano!(3), Peano!(1) => Peano!(2))
        (Peano!(3), Peano!(2) => Peano!(1))
        (Peano!(3), Peano!(3) => Peano!(0))
    }
}

#[test]
fn add_test() {
    test_op! {
        PeanoInt::Add<Rhs> Add AddFn, PeanoInt -> PeanoInt, peano => 
        (Peano!(0), Peano!(0) => Peano!(0))
        (Peano!(0), Peano!(1) => Peano!(1))
        (Peano!(0), Peano!(2) => Peano!(2))

        (Peano!(1), Peano!(0) => Peano!(1))
        (Peano!(1), Peano!(1) => Peano!(2))
        (Peano!(1), Peano!(2) => Peano!(3))

        (Peano!(2), Peano!(0) => Peano!(2))
        (Peano!(2), Peano!(1) => Peano!(3))
        (Peano!(2), Peano!(2) => Peano!(4))
    }
}

#[test]
fn mul_test() {
    test_op! {
        PeanoInt::Mul<Rhs> Mul MulFn, PeanoInt -> PeanoInt, peano => 
        (Peano!(0), Peano!(0) => Peano!(0))
        (Peano!(0), Peano!(1) => Peano!(0))
        (Peano!(0), Peano!(2) => Peano!(0))
        (Peano!(0), Peano!(3) => Peano!(0))

        (Peano!(1), Peano!(0) => Peano!(0))
        (Peano!(1), Peano!(1) => Peano!(1))
        (Peano!(1), Peano!(2) => Peano!(2))
        (Peano!(1), Peano!(3) => Peano!(3))

        (Peano!(2), Peano!(0) => Peano!(0))
        (Peano!(2), Peano!(1) => Peano!(2))
        (Peano!(2), Peano!(2) => Peano!(4))
        (Peano!(2), Peano!(3) => Peano!(6))

        (Peano!(3), Peano!(0) => Peano!(0))
        (Peano!(3), Peano!(1) => Peano!(3))
        (Peano!(3), Peano!(2) => Peano!(6))
        (Peano!(3), Peano!(3) => Peano!(9))
    }
}

#[test]
fn min_test() {
    test_op! {
        PeanoInt::Min<Rhs> Min MinFn, PeanoInt -> PeanoInt, peano => 
        (Peano!(0), Peano!(0) => Peano!(0))
        (Peano!(0), Peano!(1) => Peano!(0))
        (Peano!(0), Peano!(2) => Peano!(0))
        (Peano!(0), Peano!(3) => Peano!(0))

        (Peano!(1), Peano!(0) => Peano!(0))
        (Peano!(1), Peano!(1) => Peano!(1))
        (Peano!(1), Peano!(2) => Peano!(1))
        (Peano!(1), Peano!(3) => Peano!(1))

        (Peano!(2), Peano!(0) => Peano!(0))
        (Peano!(2), Peano!(1) => Peano!(1))
        (Peano!(2), Peano!(2) => Peano!(2))
        (Peano!(2), Peano!(3) => Peano!(2))

        (Peano!(3), Peano!(0) => Peano!(0))
        (Peano!(3), Peano!(1) => Peano!(1))
        (Peano!(3), Peano!(2) => Peano!(2))
        (Peano!(3), Peano!(3) => Peano!(3))
    }
}

#[test]
fn max_test() {
    test_op! {
        PeanoInt::Max<Rhs> Max MaxFn, PeanoInt -> PeanoInt, peano => 
        (Peano!(0), Peano!(0) => Peano!(0))
        (Peano!(0), Peano!(1) => Peano!(1))
        (Peano!(0), Peano!(2) => Peano!(2))
        (Peano!(0), Peano!(3) => Peano!(3))

        (Peano!(1), Peano!(0) => Peano!(1))
        (Peano!(1), Peano!(1) => Peano!(1))
        (Peano!(1), Peano!(2) => Peano!(2))
        (Peano!(1), Peano!(3) => Peano!(3))

        (Peano!(2), Peano!(0) => Peano!(2))
        (Peano!(2), Peano!(1) => Peano!(2))
        (Peano!(2), Peano!(2) => Peano!(2))
        (Peano!(2), Peano!(3) => Peano!(3))

        (Peano!(3), Peano!(0) => Peano!(3))
        (Peano!(3), Peano!(1) => Peano!(3))
        (Peano!(3), Peano!(2) => Peano!(3))
        (Peano!(3), Peano!(3) => Peano!(3))
    }
}

#[test]
fn is_lt_test() {
    test_op! {
        PeanoInt::IsLt<Rhs> IsLt IsLtFn, PeanoInt -> Boolean, peano => 
        (Peano!(0), Peano!(0) => Bool<false>)
        (Peano!(0), Peano!(1) => Bool<true>)
        (Peano!(0), Peano!(2) => Bool<true>)
        (Peano!(0), Peano!(3) => Bool<true>)

        (Peano!(1), Peano!(0) => Bool<false>)
        (Peano!(1), Peano!(1) => Bool<false>)
        (Peano!(1), Peano!(2) => Bool<true>)
        (Peano!(1), Peano!(3) => Bool<true>)

        (Peano!(2), Peano!(0) => Bool<false>)
        (Peano!(2), Peano!(1) => Bool<false>)
        (Peano!(2), Peano!(2) => Bool<false>)
        (Peano!(2), Peano!(3) => Bool<true>)

        (Peano!(3), Peano!(0) => Bool<false>)
        (Peano!(3), Peano!(1) => Bool<false>)
        (Peano!(3), Peano!(2) => Bool<false>)
        (Peano!(3), Peano!(3) => Bool<false>)
    }
}

#[test]
fn is_le_test() {
    test_op! {
        PeanoInt::IsLe<Rhs> IsLe IsLeFn, PeanoInt -> Boolean, peano => 
        (Peano!(0), Peano!(0) => Bool<true>)
        (Peano!(0), Peano!(1) => Bool<true>)
        (Peano!(0), Peano!(2) => Bool<true>)
        (Peano!(0), Peano!(3) => Bool<true>)

        (Peano!(1), Peano!(0) => Bool<false>)
        (Peano!(1), Peano!(1) => Bool<true>)
        (Peano!(1), Peano!(2) => Bool<true>)
        (Peano!(1), Peano!(3) => Bool<true>)

        (Peano!(2), Peano!(0) => Bool<false>)
        (Peano!(2), Peano!(1) => Bool<false>)
        (Peano!(2), Peano!(2) => Bool<true>)
        (Peano!(2), Peano!(3) => Bool<true>)

        (Peano!(3), Peano!(0) => Bool<false>)
        (Peano!(3), Peano!(1) => Bool<false>)
        (Peano!(3), Peano!(2) => Bool<false>)
        (Peano!(3), Peano!(3) => Bool<true>)
    }
}



#[test]
fn if_zero_test() {
    fn _alt_fn_is_equivalent<This, Then, Else>() 
    where
        This: PeanoInt,
    {
        let _: typewit::TypeEq<
            typewit::CallFn<peano::IfZeroAltFn<Then, Else>, This>,
            typewit::CallFn<peano::IfZeroFn, (This, Then, Else)>,
        > = typewit::TypeEq::NEW;
    }

    test_nonassoc_op! {
        PeanoInt IfZero<A, B> IfZeroFn, Identity -> Identity, peano =>

        (Peano!(0), u8, u16 => u8)
        (Peano!(1), u8, u16 => u16)
        (Peano!(2), u8, u16 => u16)
        (Peano!(3), u8, u16 => u16)
    }
}

#[test]
fn if_zero_pi_test() {
    fn _alt_fn_is_equivalent<This, Then, Else>() 
    where
        This: PeanoInt,
        Then: PeanoInt,
        Else: PeanoInt,
    {
        let _: typewit::TypeEq<
            typewit::CallFn<peano::IfZeroPIAltFn<Then, Else>, This>,
            typewit::CallFn<peano::IfZeroPIFn, (This, Then, Else)>,
        > = typewit::TypeEq::NEW;
    }

    test_nonassoc_op! {
        PeanoInt IfZeroPI<A, B> IfZeroPIFn, PeanoInt -> PeanoInt, peano =>

        (Peano!(0), Peano!(10), Peano!(20) => Peano!(10))
        (Peano!(1), Peano!(11), Peano!(21) => Peano!(21))
        (Peano!(2), Peano!(12), Peano!(22) => Peano!(22))
        (Peano!(3), Peano!(13), Peano!(23) => Peano!(23))
    }
}
