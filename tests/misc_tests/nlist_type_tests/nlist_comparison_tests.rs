use core::cmp::{Ordering, Ord, PartialOrd};

use nlist::{PeanoInt, NList, nlist};

#[test]
fn total_eq_test() {
    // making sure that NLists of different lengths can be compared
    fn inner<T, L, L2>(lhs: &NList<T, L>, rhs: &NList<T, L2>) -> bool 
    where
        T: core::cmp::Eq,
        L: PeanoInt,
        L2: PeanoInt,
    {
        lhs.total_eq(rhs)
    }

    macro_rules! test_case {
        ($lhs:expr, $rhs:expr, $is_equal:expr) => ({
            let lhs = $lhs;
            let rhs = $rhs;

            assert_eq!(inner(&lhs, &rhs), $is_equal);
            assert_eq!(inner(&rhs, &lhs), $is_equal);
        })
    }

    test_case!(nlist![0u8; 0], nlist![], true);
    test_case!(nlist![0u8; 0], nlist![3], false);
    test_case!(nlist![0u8; 0], nlist![3, 5], false);
    test_case!(nlist![0u8; 0], nlist![3, 5, 8], false);

    test_case!(nlist![3], nlist![], false);
    test_case!(nlist![3], nlist![3], true);
    test_case!(nlist![3], nlist![7], false);
    test_case!(nlist![3], nlist![3, 5], false);
    test_case!(nlist![3], nlist![3, 5, 8], false);

    test_case!(nlist![3, 5], nlist![], false);
    test_case!(nlist![3, 5], nlist![3], false);
    test_case!(nlist![3, 5], nlist![3, 5], true);
    test_case!(nlist![3, 5], nlist![7, 5], false);
    test_case!(nlist![3, 5], nlist![3, 7], false);
    test_case!(nlist![3, 5], nlist![3, 5, 8], false);

    test_case!(nlist![3, 5, 8], nlist![], false);    
    test_case!(nlist![3, 5, 8], nlist![3], false);    
    test_case!(nlist![3, 5, 8], nlist![3, 5], false);    
    test_case!(nlist![3, 5, 8], nlist![3, 5, 8], true);    
    test_case!(nlist![3, 5, 8], nlist![0, 5, 8], false);
    test_case!(nlist![3, 5, 8], nlist![3, 0, 8], false);
    test_case!(nlist![3, 5, 8], nlist![3, 5, 0], false);
}

#[test]
fn eq_trait_test() {
    fn assert_impls_eq<T: core::cmp::Eq>() {}

    fn inner<T, L>(lhs: &NList<T, L>, rhs: &NList<T, L>) -> bool 
    where
        T: core::cmp::Eq,
        L: PeanoInt,
    {
        _ = assert_impls_eq::<NList<T, L>>;

        lhs == rhs
    }

    macro_rules! test_case {
        ($lhs:expr, $rhs:expr, $is_equal:expr) => ({
            let lhs = $lhs;
            let rhs = $rhs;

            assert_eq!(inner(&lhs, &rhs), $is_equal);
            assert_eq!(inner(&rhs, &lhs), $is_equal);
        })
    }

    test_case!(nlist![0u8; 0], nlist![], true);

    test_case!(nlist![3], nlist![3], true);
    test_case!(nlist![3], nlist![7], false);

    test_case!(nlist![3, 5], nlist![3, 5], true);
    test_case!(nlist![3, 5], nlist![7, 5], false);
    test_case!(nlist![3, 5], nlist![3, 7], false);

    test_case!(nlist![3, 5, 8], nlist![3, 5, 8], true);    
    test_case!(nlist![3, 5, 8], nlist![0, 5, 8], false);
    test_case!(nlist![3, 5, 8], nlist![3, 0, 8], false);
    test_case!(nlist![3, 5, 8], nlist![3, 5, 0], false);
}

#[test]
fn inherent_cmp_and_partial_cmp_trait_test() {
    // making sure that NLists of different lengths can be compared
    fn inner_inherent_cmp<T, L, L2>(lhs: &NList<T, L>, rhs: &NList<T, L2>) -> Ordering 
    where
        T: Ord,
        L: PeanoInt,
        L2: PeanoInt,
    {
        lhs.cmp(rhs)
    }
    // making sure that NLists of different element types and lengths can be compared
    fn inner_trait_partial_cmp<T, U, L, L2>(lhs: &NList<T, L>, rhs: &NList<U, L2>) -> Ordering 
    where
        T: PartialOrd<U>,
        L: PeanoInt,
        L2: PeanoInt,
    {
        PartialOrd::partial_cmp(lhs, rhs).unwrap()
    }

    macro_rules! test_case {
        ($lhs:expr, $rhs:expr, $ordering:expr) => ({
            let lhs = $lhs;
            let rhs = $rhs;

            assert_eq!(inner_inherent_cmp(&lhs, &rhs), $ordering);
            assert_eq!(inner_trait_partial_cmp(&lhs, &rhs), $ordering);

            assert_eq!(inner_inherent_cmp(&rhs, &lhs), $ordering.reverse());
            assert_eq!(inner_trait_partial_cmp(&rhs, &lhs), $ordering.reverse());
        })
    }

    test_case!(nlist![0u8; 0], nlist![], Ordering::Equal);
    test_case!(nlist![0u8; 0], nlist![3], Ordering::Less);
    test_case!(nlist![0u8; 0], nlist![3, 5], Ordering::Less);
    test_case!(nlist![0u8; 0], nlist![3, 5, 8], Ordering::Less);

    test_case!(nlist![3], nlist![], Ordering::Greater);
    test_case!(nlist![3], nlist![3], Ordering::Equal);
    test_case!(nlist![3], nlist![0], Ordering::Greater);
    test_case!(nlist![3], nlist![7], Ordering::Less);
    test_case!(nlist![3], nlist![0, 5], Ordering::Greater);
    test_case!(nlist![3], nlist![3, 5], Ordering::Less);
    test_case!(nlist![3], nlist![3, 5, 8], Ordering::Less);

    test_case!(nlist![3, 5], nlist![], Ordering::Greater);
    test_case!(nlist![3, 5], nlist![0], Ordering::Greater);
    test_case!(nlist![3, 5], nlist![3], Ordering::Greater);
    test_case!(nlist![3, 5], nlist![5], Ordering::Less);
    test_case!(nlist![3, 5], nlist![0, 5], Ordering::Greater);
    test_case!(nlist![3, 5], nlist![3, 5], Ordering::Equal);
    test_case!(nlist![3, 5], nlist![7, 5], Ordering::Less);
    test_case!(nlist![3, 5], nlist![3, 7], Ordering::Less);
    test_case!(nlist![3, 5], nlist![0, 5, 8], Ordering::Greater);
    test_case!(nlist![3, 5], nlist![3, 5, 8], Ordering::Less);
    test_case!(nlist![3, 5], nlist![7, 5, 8], Ordering::Less);

    test_case!(nlist![3, 5, 8], nlist![], Ordering::Greater);
    test_case!(nlist![3, 5, 8], nlist![0], Ordering::Greater);    
    test_case!(nlist![3, 5, 8], nlist![3], Ordering::Greater);    
    test_case!(nlist![3, 5, 8], nlist![7], Ordering::Less);    
    test_case!(nlist![3, 5, 8], nlist![0, 5], Ordering::Greater);
    test_case!(nlist![3, 5, 8], nlist![3, 5], Ordering::Greater);
    test_case!(nlist![3, 5, 8], nlist![7, 5], Ordering::Less);
    test_case!(nlist![3, 5, 8], nlist![3, 5, 8], Ordering::Equal);    
    test_case!(nlist![3, 5, 8], nlist![0, 5, 8], Ordering::Greater);
    test_case!(nlist![3, 5, 8], nlist![4, 5, 8], Ordering::Less);
    test_case!(nlist![3, 5, 8], nlist![3, 0, 8], Ordering::Greater);
    test_case!(nlist![3, 5, 8], nlist![3, 5, 0], Ordering::Greater);
}

#[test]
fn trait_cmp_test() {
    fn inner_trait_cmp<T, L>(lhs: &NList<T, L>, rhs: &NList<T, L>) -> Ordering 
    where
        T: Ord,
        L: PeanoInt,
    {
        Ord::cmp(lhs, rhs)
    }

    macro_rules! test_case {
        ($lhs:expr, $rhs:expr, $ordering:expr) => ({
            let lhs = $lhs;
            let rhs = $rhs;

            assert_eq!(inner_trait_cmp(&lhs, &rhs), $ordering);
            assert_eq!(inner_trait_cmp(&rhs, &lhs), $ordering.reverse());
        })
    }

    test_case!(nlist![0u8; 0], nlist![], Ordering::Equal);

    test_case!(nlist![3], nlist![3], Ordering::Equal);
    test_case!(nlist![3], nlist![0], Ordering::Greater);
    test_case!(nlist![3], nlist![7], Ordering::Less);

    test_case!(nlist![3, 5], nlist![0, 5], Ordering::Greater);
    test_case!(nlist![3, 5], nlist![3, 5], Ordering::Equal);
    test_case!(nlist![3, 5], nlist![7, 5], Ordering::Less);
    test_case!(nlist![3, 5], nlist![3, 7], Ordering::Less);

    test_case!(nlist![3, 5, 8], nlist![3, 5, 8], Ordering::Equal);    
    test_case!(nlist![3, 5, 8], nlist![0, 5, 8], Ordering::Greater);
    test_case!(nlist![3, 5, 8], nlist![4, 5, 8], Ordering::Less);
    test_case!(nlist![3, 5, 8], nlist![3, 0, 8], Ordering::Greater);
    test_case!(nlist![3, 5, 8], nlist![3, 5, 0], Ordering::Greater);
}



