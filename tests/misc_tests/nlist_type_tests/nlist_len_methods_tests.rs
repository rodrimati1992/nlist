use nlist::{Peano, Int, IntWit, NList, nlist};
use nlist::typewit::TypeEq;

use crate::misc_tests::test_utils::{assertm, assert_type};



#[test]
fn len_test() {
    const fn callit<T, L: Int>(list: &NList<T, L>) -> usize {
        list.len()
    }

    assert_eq!(callit(&nlist![0u8; 0]), 0);
    assert_eq!(callit(&nlist![1u16]), 1);
    assert_eq!(callit(&nlist![2u32, 3]), 2);
    assert_eq!(callit(&nlist![5u64, 8, 13]), 3);

}

#[test]
fn is_empty_test() {
    const fn callit<T, L: Int>(list: &NList<T, L>) -> bool {
        list.is_empty()
    }

    assert_eq!(callit(&nlist![0u8; 0]), true);
    assert_eq!(callit(&nlist![1u16]), false);
    assert_eq!(callit(&nlist![2u32, 3]), false);
    assert_eq!(callit(&nlist![5u64, 8, 13]), false);
}

#[test]
fn len_proof_test() {
    const fn callit<T, L: Int>(list: &NList<T, L>) -> IntWit<L> {
        list.len_proof()
    }

    {
        let proof = callit(&nlist![0u8; 0]);
        assert_type::<IntWit<Peano!(0)>>(proof);
        assertm!(proof, IntWit::Zeros{..});
    }
    {
        let proof = callit(&nlist![1u16]);
        assert_type::<IntWit<Peano!(1)>>(proof);
        assertm!(proof, IntWit::Nat{..});
    }
    {
        let proof = callit(&nlist![2u32, 3]);
        assert_type::<IntWit<Peano!(2)>>(proof);
        assertm!(proof, IntWit::Nat{..});
    }
    {
        let proof = callit(&nlist![5u64, 8, 13]);
        assert_type::<IntWit<Peano!(3)>>(proof);
        assertm!(proof, IntWit::Nat{..});
    }
}

macro_rules! coerce_len_test_impl {
    (|$list:ident, $proof:ident| $method:expr, ($($r:tt)*)) => ({
        const fn callit<T, L, L2>(
            $list: $($r)* NList<T, L>, 
            $proof: TypeEq<L, L2>,
        ) -> $($r)* NList<T, L2> 
        where
            L: Int,
            L2: Int,
        {
            $method
        }

        
        assert_type::<$($r)* NList<u8, Peano!(0)>>(
            callit($($r)* nlist![0u8; 0], TypeEq::NEW)
        );
        assert_eq!(callit($($r)* nlist![0u8; 0], TypeEq::NEW), $($r)* nlist![0u8; 0]);
        
        assert_type::<$($r)* NList<u32, Peano!(1)>>(
            callit($($r)* nlist![3u32], TypeEq::NEW)
        );
        assert_eq!(callit($($r)* nlist![3], TypeEq::NEW), $($r)* nlist![3]);
        
        assert_type::<$($r)* NList<u64, Peano!(2)>>(
            callit($($r)* nlist![3u64, 5], TypeEq::NEW)
        );
        assert_eq!(callit($($r)* nlist![3, 5], TypeEq::NEW), $($r)* nlist![3, 5]);
        
        assert_type::<$($r)* NList<u128, Peano!(3)>>(
            callit($($r)* nlist![3u128, 5, 8], TypeEq::NEW)
        );
        assert_eq!(callit($($r)* nlist![3, 5, 8], TypeEq::NEW), $($r)* nlist![3, 5, 8]);
    })
}
#[test]
fn coerce_len_test() {
    coerce_len_test_impl!{|list, proof| list.coerce_len(proof), ()}
}

#[test]
fn as_coerce_len_test() {
    coerce_len_test_impl!{|list, proof| list.as_coerce_len(proof), (&)}
}

#[test]
fn as_mut_coerce_len_test() {
    coerce_len_test_impl!{|list, proof| list.as_mut_coerce_len(proof), (&mut )}
}

#[test]
fn coerce_len_poly_test() {
    coerce_len_test_impl!{|list, proof| NList::coerce_len_poly(list, proof), ()}
    coerce_len_test_impl!{|list, proof| NList::coerce_len_poly(list, proof), (&)}
    coerce_len_test_impl!{|list, proof| NList::coerce_len_poly(list, proof), (&mut )}
}

