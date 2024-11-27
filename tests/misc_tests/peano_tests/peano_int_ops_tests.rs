use nlist::{Peano, peano};
use nlist::boolean::{Bool, Boolean};
use nlist::peano::{IntoPeano, PeanoInt, PeanoWit, FromUsize, IntoUsize, PlusOne, Usize, Zero};
use nlist::typewit::{CallFn, TypeFn};

use crate::misc_tests::test_utils::assert_type_eq;

macro_rules! test_op {
    (
        $assoc_ty:ident<$($args:ident),*> 
        $type_alias:ident 
        $typefn:ident
        $arg_bound:ident 
        $ret_bound:ident 
        =>
        $(( $first_arg:ty $(,$rem_args:ty)* => $returned:ty ))*
    ) => (
        fn assert_bound<T: $ret_bound>(){}

        #[allow(unused_parens)]
        fn inner<Expected, This: $arg_bound, $($args: $arg_bound),*>() {
            assert_bound::<<This as $arg_bound>::$assoc_ty<$($args)*>>();

            assert_type_eq::<<This as $arg_bound>::$assoc_ty<$($args)*>, Expected>();
            
            assert_type_eq::<peano::$type_alias<This $(,$args)*>, Expected>();
            
            assert_type_eq::<CallFn<peano::type_fns::$typefn, (This $(,$args)*)>, Expected>();
        }

        $(
            inner::<$returned, $first_arg $(,$rem_args)*>();
        )*
    )
}


#[test]
fn sub_one_sat_test() {
    test_op! {
        SubOneSat<> SubOneSat SubOneSatFn PeanoInt PeanoInt => 
        (Peano!(0) => Peano!(0))
        (Peano!(1) => Peano!(0))
        (Peano!(2) => Peano!(1))
        (Peano!(3) => Peano!(2))
    }
}

#[test]
fn is_zero_test() {
    test_op! {
        IsZero<> IsZero IsZeroFn PeanoInt Boolean => 
        (Peano!(0) => Bool<true>)
        (Peano!(1) => Bool<false>)
        (Peano!(2) => Bool<false>)
        (Peano!(3) => Bool<false>)
    }
}

#[test]
fn sub_sat_test() {
    test_op! {
        SubSat<Rhs> SubSat SubSatFn PeanoInt PeanoInt => 
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
        Add<Rhs> Add AddFn PeanoInt PeanoInt => 
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
        Mul<Rhs> Mul MulFn PeanoInt PeanoInt => 
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
        Min<Rhs> Min MinFn PeanoInt PeanoInt => 
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
        Max<Rhs> Max MaxFn PeanoInt PeanoInt => 
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
        IsLt<Rhs> IsLt IsLtFn PeanoInt Boolean => 
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
        IsLe<Rhs> IsLe IsLeFn PeanoInt Boolean => 
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



    // IfZero<Then, Else>
    // IfZeroPI<Then, Else>