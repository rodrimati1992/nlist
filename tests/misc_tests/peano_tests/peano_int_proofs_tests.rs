use nlist::boolean::{Bool, Boolean, BoolWitG};
use nlist::peano::{self, Peano, PeanoInt, Zero, proofs};
use nlist::typewit::TypeEq;

use crate::misc_tests::test_utils::assert_type;


macro_rules! __call_with_callback {
    ($callback:ident $(($($int:tt)*))*) => {
        $(let _ = $callback::<$(Peano!($int)),*>();)*
    }
}
macro_rules! call_with_unary {
    ($callback:ident) => (
        __call_with_callback!{$callback (0) (1) (2) (3) (4)}
    )
}

macro_rules! call_with_binary {
    ($callback:ident) => (
        __call_with_callback!{$callback
            (0 0) (0 1) (0 2) (0 3) (0 4) 
            (1 0) (1 1) (1 2) (1 3) (1 4) 
            (2 0) (2 1) (2 2) (2 3) (2 4) 
            (3 0) (3 1) (3 2) (3 3) (3 4) 
            (4 0) (4 1) (4 2) (4 3) (4 4) 
        }
    )
}

macro_rules! call_with_ternary {
    ($callback:ident) => (
        __call_with_callback!{$callback
            (0 0 0) (0 0 1) (0 0 2) (0 0 3) (0 0 4) 
            (0 1 0) (0 1 1) (0 1 2) (0 1 3) (0 1 4) 
            (0 2 0) (0 2 1) (0 2 2) (0 2 3) (0 2 4) 
            (0 3 0) (0 3 1) (0 3 2) (0 3 3) (0 3 4) 
            (0 4 0) (0 4 1) (0 4 2) (0 4 3) (0 4 4) 
            //
            (1 0 0) (1 0 1) (1 0 2) (1 0 3) (1 0 4) 
            (1 1 0) (1 1 1) (1 1 2) (1 1 3) (1 1 4) 
            (1 2 0) (1 2 1) (1 2 2) (1 2 3) (1 2 4) 
            (1 3 0) (1 3 1) (1 3 2) (1 3 3) (1 3 4) 
            (1 4 0) (1 4 1) (1 4 2) (1 4 3) (1 4 4) 
            //
            (2 0 0) (2 0 1) (2 0 2) (2 0 3) (2 0 4) 
            (2 1 0) (2 1 1) (2 1 2) (2 1 3) (2 1 4) 
            (2 2 0) (2 2 1) (2 2 2) (2 2 3) (2 2 4) 
            (2 3 0) (2 3 1) (2 3 2) (2 3 3) (2 3 4) 
            (2 4 0) (2 4 1) (2 4 2) (2 4 3) (2 4 4) 
            //
            (3 0 0) (3 0 1) (3 0 2) (3 0 3) (3 0 4) 
            (3 1 0) (3 1 1) (3 1 2) (3 1 3) (3 1 4) 
            (3 2 0) (3 2 1) (3 2 2) (3 2 3) (3 2 4) 
            (3 3 0) (3 3 1) (3 3 2) (3 3 3) (3 3 4) 
            (3 4 0) (3 4 1) (3 4 2) (3 4 3) (3 4 4) 
            //
            (4 0 0) (4 0 1) (4 0 2) (4 0 3) (4 0 4) 
            (4 1 0) (4 1 1) (4 1 2) (4 1 3) (4 1 4) 
            (4 2 0) (4 2 1) (4 2 2) (4 2 3) (4 2 4) 
            (4 3 0) (4 3 1) (4 3 2) (4 3 3) (4 3 4) 
            (4 4 0) (4 4 1) (4 4 2) (4 4 3) (4 4 4) 
        }
    )
}

#[test]
fn commutative_add_test() {
    fn inner<A, B>() 
    where
        A: PeanoInt,
        B: PeanoInt,
    {
        let ret = const { proofs::commutative_add::<A, B>() };
        assert_type::<TypeEq<peano::Add<A, B>, peano::Add<B, A>>>(ret);
    }

    call_with_binary!{inner}
}
#[test]
fn commutative_mul_test() {
    fn inner<A, B>() 
    where
        A: PeanoInt,
        B: PeanoInt,
    {
        let ret = const { proofs::commutative_mul::<A, B>() };
        assert_type::<TypeEq<peano::Mul<A, B>, peano::Mul<B, A>>>(ret);
    }

    call_with_binary!{inner}
}


#[test]
fn add_identity_test() {
    fn inner<A>() 
    where
        A: PeanoInt,
    {
        let ret = const { proofs::add_identity::<A>() };
        assert_type::<TypeEq<peano::Add<A, Zero>, A>>(ret);
    }

    call_with_unary!{inner}
}
#[test]
fn sub_identity_test() {
    fn inner<A>() 
    where
        A: PeanoInt,
    {
        let ret = const { proofs::sub_identity::<A>() };
        assert_type::<TypeEq<peano::SubSat<A, Zero>, A>>(ret);
    }

    call_with_unary!{inner}
}
#[test]
fn compose_sub_lt_test() {
    fn inner<A, B, C>() 
    where
        A: PeanoInt,
        B: PeanoInt,
        C: PeanoInt,
    {
        let ret = const { 
            match peano::IsLt::<A, B>::BOOL_WIT {
                BoolWitG::True(te) => Some(proofs::compose_sub_lt::<A, B, C>(te)),
                BoolWitG::False(_) => None,
            }
        };

        assert_eq!(A::USIZE < B::USIZE, ret.is_some());

        assert_type::<Option<TypeEq<peano::IsLt<peano::SubSat<A, C>, B>, Bool<true>>>>(ret);
    }

    call_with_ternary!{inner}
}













