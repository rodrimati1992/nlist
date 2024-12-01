use nlist::{Peano, PeanoInt};
use nlist::boolean::{self, Bool, Boolean};
use nlist::typewit::Identity;

use crate::misc_tests::test_utils::{assert_type_eq, test_op};



#[test]
fn not_test() {
    test_op! {
        Boolean::Not<> Not NotFn, Boolean -> Boolean, boolean =>
        (Bool<false> => Bool<true>)
        (Bool<true> => Bool<false>)
    }
}

#[test]
fn and_test() {
    test_op! {
        Boolean::And<Rhs> And AndFn, Boolean -> Boolean, boolean =>
        (Bool<false>, Bool<false> => Bool<false>)
        (Bool<false>, Bool<true> => Bool<false>)
        (Bool<true>, Bool<false> => Bool<false>)
        (Bool<true>, Bool<true> => Bool<true>)
    }
}

#[test]
fn or_test() {
    test_op! {
        Boolean::Or<Rhs> Or OrFn, Boolean -> Boolean, boolean =>
        (Bool<false>, Bool<false> => Bool<false>)
        (Bool<false>, Bool<true> => Bool<true>)
        (Bool<true>, Bool<false> => Bool<true>)
        (Bool<true>, Bool<true> => Bool<true>)
    }
}

#[test]
fn xor_test() {
    test_op! {
        Boolean::Xor<Rhs> Xor XorFn, Boolean -> Boolean, boolean =>
        (Bool<false>, Bool<false> => Bool<false>)
        (Bool<false>, Bool<true> => Bool<true>)
        (Bool<true>, Bool<false> => Bool<true>)
        (Bool<true>, Bool<true> => Bool<false>)
    }
}

#[test]
fn iftrue_test() {
    fn _alt_fn_is_equivalent<B, Then, Else>() 
    where
        B: Boolean,
    {
        let _: typewit::TypeEq<
            typewit::CallFn<boolean::IfTrueAltFn<Then, Else>, B>,
            typewit::CallFn<boolean::IfTrueFn, (B, Then, Else)>,
        > = typewit::TypeEq::NEW;
    }

    test_op! {
        Boolean::IfTrue<Then, Else> IfTrue IfTrueFn, Identity -> Identity, boolean =>
        (Bool<false>, u8, u16 => u16)
        (Bool<false>, u16, u8 => u8)
        (Bool<true>, u8, u16 => u8)
        (Bool<true>, u16, u8 => u16)
    }
}

#[test]
fn iftrueb_test() {
    fn _alt_fn_is_equivalent<B, Then, Else>() 
    where
        B: Boolean,
        Then: Boolean,
        Else: Boolean,
    {
        let _: typewit::TypeEq<
            typewit::CallFn<boolean::IfTrueBAltFn<Then, Else>, B>,
            typewit::CallFn<boolean::IfTrueBFn, (B, Then, Else)>,
        > = typewit::TypeEq::NEW;
    }

    test_op! {
        Boolean::IfTrueB<Then, Else> IfTrueB IfTrueBFn, Boolean -> Boolean, boolean =>
        (Bool<false>, Bool<false>, Bool<true> => Bool<true>)
        (Bool<false>, Bool<true>, Bool<false> => Bool<false>)
        (Bool<true>, Bool<false>, Bool<true> => Bool<false>)
        (Bool<true>, Bool<true>, Bool<false> => Bool<true>)
    }
}

#[test]
fn iftruepi_test() {
    fn _alt_fn_is_equivalent<B, Then, Else>() 
    where
        B: Boolean,
        Then: PeanoInt,
        Else: PeanoInt,
    {
        let _: typewit::TypeEq<
            typewit::CallFn<boolean::IfTruePIAltFn<Then, Else>, B>,
            typewit::CallFn<boolean::IfTruePIFn, (B, Then, Else)>,
        > = typewit::TypeEq::NEW;
    }

    test_op! {
        Boolean::IfTruePI<Then, Else> IfTruePI IfTruePIFn, PeanoInt -> PeanoInt, boolean =>
        (Bool<false>, Peano!(3), Peano!(5) => Peano!(5))
        (Bool<false>, Peano!(8), Peano!(13) => Peano!(13))
        (Bool<true>, Peano!(3), Peano!(5) => Peano!(3))
        (Bool<true>, Peano!(8), Peano!(13) => Peano!(8))
    }
}
