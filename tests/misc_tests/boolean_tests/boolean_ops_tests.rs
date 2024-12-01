use nlist::{Peano, PeanoInt};
use nlist::boolean::{self, Bool, Boolean};
use nlist::typewit::Identity;

use crate::misc_tests::test_utils::{assert_type_eq, test_op};




#[test]
fn not_test() {
    test_op! {
        Boolean::Not<> Not, Boolean -> Boolean, boolean =>
        (Bool<false> => Bool<true>)
        (Bool<true> => Bool<false>)
    }
}

#[test]
fn and_test() {
    test_op! {
        Boolean::And<Rhs> And, Boolean -> Boolean, boolean =>
        (Bool<false>, Bool<false> => Bool<false>)
        (Bool<false>, Bool<true> => Bool<false>)
        (Bool<true>, Bool<false> => Bool<false>)
        (Bool<true>, Bool<true> => Bool<true>)
    }
}

#[test]
fn or_test() {
    test_op! {
        Boolean::Or<Rhs> Or, Boolean -> Boolean, boolean =>
        (Bool<false>, Bool<false> => Bool<false>)
        (Bool<false>, Bool<true> => Bool<true>)
        (Bool<true>, Bool<false> => Bool<true>)
        (Bool<true>, Bool<true> => Bool<true>)
    }
}

#[test]
fn iftrue_test() {
    test_op! {
        Boolean::IfTrue<Then, Else> IfTrue, Identity -> Identity, boolean =>
        (Bool<false>, u8, u16 => u16)
        (Bool<false>, u16, u8 => u8)
        (Bool<true>, u8, u16 => u8)
        (Bool<true>, u16, u8 => u16)
    }
}

#[test]
fn iftrueb_test() {
    test_op! {
        Boolean::IfTrueB<Then, Else> IfTrueB, Boolean -> Boolean, boolean =>
        (Bool<false>, Bool<false>, Bool<true> => Bool<true>)
        (Bool<false>, Bool<true>, Bool<false> => Bool<false>)
        (Bool<true>, Bool<false>, Bool<true> => Bool<false>)
        (Bool<true>, Bool<true>, Bool<false> => Bool<true>)
    }
}

#[test]
fn iftruepi_test() {
    test_op! {
        Boolean::IfTruePI<Then, Else> IfTruePI, PeanoInt -> PeanoInt, boolean =>
        (Bool<false>, Peano!(3), Peano!(5) => Peano!(5))
        (Bool<false>, Peano!(8), Peano!(13) => Peano!(13))
        (Bool<true>, Peano!(3), Peano!(5) => Peano!(3))
        (Bool<true>, Peano!(8), Peano!(13) => Peano!(8))
    }
}
