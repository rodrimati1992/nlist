use nlist::{Peano, Int, peano};
use nlist::boolean::{Bool, Boolean};
use nlist::typewit::{CallFn, Identity};

use crate::misc_tests::test_utils::{assert_type_eq, test_op, test_nonassoc_op};




#[test]
fn sub_one_sat_test() {
    test_op! {
        Int::SubOneSat<> SubOneSat SubOneSatFn, Int -> Int, peano =>
        (Peano!(0) => Peano!(0))
        (Peano!(1) => Peano!(0))
        (Peano!(2) => Peano!(1))
        (Peano!(3) => Peano!(2))
    }
}

#[test]
fn is_zero_test() {
    test_op! {
        Int::IsZero<> IsZero IsZeroFn, Int -> Boolean, peano => 
        (Peano!(0) => Bool<true>)
        (Peano!(1) => Bool<false>)
        (Peano!(2) => Bool<false>)
        (Peano!(3) => Bool<false>)
    }
}

#[test]
fn sub_sat_test() {
    test_op! {
        Int::SubSat<Rhs> SubSat SubSatFn, Int -> Int, peano => 
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
        Int::Add<Rhs> Add AddFn, Int -> Int, peano => 
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
        Int::Mul<Rhs> Mul MulFn, Int -> Int, peano => 
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
        Int::Min<Rhs> Min MinFn, Int -> Int, peano => 
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
        Int::Max<Rhs> Max MaxFn, Int -> Int, peano => 
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
        Int::IsLt<Rhs> IsLt IsLtFn, Int -> Boolean, peano => 
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
        Int::IsLe<Rhs> IsLe IsLeFn, Int -> Boolean, peano => 
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
        This: Int,
    {
        let _: typewit::TypeEq<
            typewit::CallFn<peano::IfZeroAltFn<Then, Else>, This>,
            typewit::CallFn<peano::IfZeroFn, (This, Then, Else)>,
        > = typewit::TypeEq::NEW;
    }

    test_nonassoc_op! {
        Int IfZero<A, B> IfZeroFn, Identity -> Identity, peano =>

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
        This: Int,
        Then: Int,
        Else: Int,
    {
        let _: typewit::TypeEq<
            typewit::CallFn<peano::IfZeroIAltFn<Then, Else>, This>,
            typewit::CallFn<peano::IfZeroIFn, (This, Then, Else)>,
        > = typewit::TypeEq::NEW;
    }

    test_nonassoc_op! {
        Int IfZeroI<A, B> IfZeroIFn, Int -> Int, peano =>

        (Peano!(0), Peano!(10), Peano!(20) => Peano!(10))
        (Peano!(1), Peano!(11), Peano!(21) => Peano!(21))
        (Peano!(2), Peano!(12), Peano!(22) => Peano!(22))
        (Peano!(3), Peano!(13), Peano!(23) => Peano!(23))
    }
}
