use nlist::{Peano, PeanoInt, NList, nlist, peano};
use nlist::boolean::{Bool, Boolean, BoolWitG};

use crate::misc_tests::test_utils::assert_type;


#[test]
fn split_at_test() {
    const fn callit<T, At, L>(
        list: NList<T, L>
    ) -> (NList<T, At>, NList<T, peano::SubSat<L, At>>)
    where
        L: PeanoInt,
        At: peano::PeanoInt<IsLe<L> = Bool<true>>,
    {
        list.split_at::<At>()
    }

    macro_rules! split_at_ {
        ($at:literal) => ({
            const fn split_at_at<T, L>(
                list: NList<T, peano::Add<Peano!($at), L>>
            ) -> (NList<T, Peano!($at)>, NList<T, L>)
            where
                L: PeanoInt,
            {
                konst::destructure!{(before, after) = callit::<_, Peano!($at), _>(list)}
                
                (before, after.coerce_len(peano::proofs::sub_identity()))
            }

            assert_type::<(NList<u8, Peano!($at)>, NList<u8, Peano!(5 - $at)>)>(
                nlist![3u8, 5, 8, 13, 21].split_at::<Peano!($at)>()
            );

            const BOTH: (NList<u64, Peano!($at)>, NList<u64, Peano!(5 - $at)>) = 
                split_at_at(nlist![3, 5, 8, 13, 21]);

            assert_eq!(BOTH.0.into_array().as_slice(), [3, 5, 8, 13, 21].split_at($at).0);
            assert_eq!(BOTH.1.into_array().as_slice(), [3, 5, 8, 13, 21].split_at($at).1);
        })
    }


    split_at_!{0}
    split_at_!{1}
    split_at_!{2}
    split_at_!{3}
    split_at_!{4}
    split_at_!{5}
}


#[test]
fn split_at_alt_test() {
    const fn callit<T, At, L>(
        list: NList<T, L>,
    ) -> Option<(NList<T, At>, NList<T, peano::SubSat<L, At>>)>
    where
        T: Copy,
        L: PeanoInt,
        At: PeanoInt,
    {
        match peano::IsLe::<At, L>::BOOL_WIT {
            BoolWitG::True(is_le) => Some(list.split_at_alt::<At>(is_le)),
            BoolWitG::False(_) => {
                list.assert_copy_drop();
                None
            }
        }
    }

    type Len = Peano!(5);

    macro_rules! split_at_ {
        ($at:literal) => ({
            type At = Peano!($at);
            type TailLen = peano::SubSat<Len, At>;

            const LIST: [u8; Len::USIZE] = [3u8, 5, 8, 13, 21];
            const NLIST: NList<u8, Len> = NList::from_array(LIST);

            assert_type::<Option<(NList<u8, At>, NList<u8, TailLen>)>>(
                match peano::IsLe::<At, Len>::BOOL_WIT {
                    BoolWitG::True(is_le) => Some(NLIST.split_at_alt::<At>(is_le)),
                    BoolWitG::False(_) => None,
                }
            );

            const BOTH: Option<(NList<u8, At>, NList<u8, TailLen>)> = 
                callit(NLIST);

            let at = $at;
            if at > Len::USIZE {
                assert_eq!(BOTH, None);
            } else {
                let both = BOTH.unwrap();
                
                assert_eq!(both.0.into_array().as_slice(), [3, 5, 8, 13, 21].split_at($at).0);
                assert_eq!(both.1.into_array().as_slice(), [3, 5, 8, 13, 21].split_at($at).1);
            }
        })
    }


    split_at_!{0}
    split_at_!{1}
    split_at_!{2}
    split_at_!{3}
    split_at_!{4}
    split_at_!{5}
    split_at_!{6}
    split_at_!{7}
}


#[test]
fn get_test() {
    const fn callit<T, L>(list: &NList<T, L>, index: usize) -> Option<&T>
    where
        L: PeanoInt
    {
        list.get(index)
    }

    macro_rules! with_length {
        ($L:literal) => ({
            static LIST: NList<usize, Peano!($L)> = NList::from_array(konst::array::from_fn_!(
                |i| i.pow(2)
            ));

            let len = $L;
            for n in 0usize..=len + 5 {
                if n < len {
                    assert_eq!(callit(&LIST, n), Some(&n.pow(2)));
                } else {
                    assert_eq!(callit(&LIST, n), None);
                }
            }
        })
    }

    with_length!{0}
    with_length!{1}
    with_length!{2}
    with_length!{3}
    with_length!{4}

}

#[test]
fn get_mut_test() {
    const fn callit<T, L>(list: &mut NList<T, L>, index: usize) -> Option<&mut T>
    where
        L: PeanoInt
    {
        list.get_mut(index)
    }

    macro_rules! with_length {
        ($L:literal) => ({
            let mut list: NList<usize, Peano!($L)> = NList::from_array(konst::array::from_fn_!(
                |i| i.pow(2)
            ));

            let len = $L;
            for n in 0usize..=len + 5 {
                if n < len {
                    assert_eq!(callit(&mut list, n), Some(&mut n.pow(2)));
                } else {
                    assert_eq!(callit(&mut list, n), None);
                }
            }
        })
    }

    with_length!{0}
    with_length!{1}
    with_length!{2}
    with_length!{3}
    with_length!{4}
}

#[test]
fn index_test() {
    const fn callit<T, L, At>(list: &NList<T, L>, _at: At) -> &T
    where
        L: PeanoInt,
        At: PeanoInt<IsLt<L> = Bool<true>>
    {
        list.index::<At>()
    }

    macro_rules! test_case {
        ($L:literal $At:literal) => ({
            type L = Peano!($L);
            static LIST: NList<usize, L> = NList::from_array(konst::array::from_fn_!(
                |i| i.pow(2)
            ));

            let at: usize = $At;
            assert_eq!(callit(&LIST, peano!($At)), &at.pow(2));
        })
    }

    test_case!{5 0}
    test_case!{5 1}
    test_case!{5 2}
    test_case!{5 3}
    test_case!{5 4}

    test_case!{4 0}
    test_case!{4 1}
    test_case!{4 2}
    test_case!{4 3}

    test_case!{3 0}
    test_case!{3 1}
    test_case!{3 2}

    test_case!{2 0}
    test_case!{2 1}

    test_case!{1 0}
}

#[test]
fn index_alt_test() {
    const fn callit<T, L, At>(list: &NList<T, L>, _at: At) -> Option<&T>
    where
        L: PeanoInt,
        At: PeanoInt,
    {
        match peano::IsLt::<At, L>::BOOL_WIT {
            BoolWitG::True(proof) => {
                Some(list.index_alt::<At>(proof))
            }
            BoolWitG::False(_) => None
        }
    }

    macro_rules! test_case {
        ($L:literal [$($indices:literal)*]) => ({
            type L = Peano!($L);
            static LIST: NList<usize, L> = NList::from_array(konst::array::from_fn_!(
                |i| i.pow(2)
            ));

            $(
                let at: usize = $indices;
                let len = $L;
                let ret = callit(&LIST, peano!($indices));

                if at < len {
                    assert_eq!(ret, Some(&at.pow(2)));
                } else {
                    assert_eq!(ret, None);
                }
            )*
        })
    }

    test_case!{0 [0 1 2 3]}
    test_case!{1 [0 1 2 3]}
    test_case!{2 [0 1 2 3]}
}

#[test]
fn index_mut_test() {
    const fn callit<T, L, At>(list: &mut NList<T, L>, _at: At) -> &mut T
    where
        L: PeanoInt,
        At: PeanoInt<IsLt<L> = Bool<true>>
    {
        list.index_mut::<At>()
    }

    macro_rules! test_case {
        ($L:literal $At:literal) => ({
            type L = Peano!($L);
            let mut list: NList<usize, L> = NList::from_array(konst::array::from_fn_!(
                |i| i.pow(2)
            ));

            let at: usize = $At;
            assert_eq!(callit(&mut list, peano!($At)), &mut at.pow(2));
        })
    }

    test_case!{5 0}
    test_case!{5 1}
    test_case!{5 2}
    test_case!{5 3}
    test_case!{5 4}

    test_case!{4 0}
    test_case!{4 1}
    test_case!{4 2}
    test_case!{4 3}

    test_case!{3 0}
    test_case!{3 1}
    test_case!{3 2}

    test_case!{2 0}
    test_case!{2 1}

    test_case!{1 0}
}

#[test]
fn index_mut_alt_test() {
    const fn callit<T, L, At>(list: &mut NList<T, L>, _at: At) -> Option<&mut T>
    where
        L: PeanoInt,
        At: PeanoInt,
    {
        match peano::IsLt::<At, L>::BOOL_WIT {
            BoolWitG::True(proof) => {
                Some(list.index_mut_alt::<At>(proof))
            }
            BoolWitG::False(_) => None
        }
    }

    macro_rules! test_case {
        ($L:literal [$($indices:literal)*]) => ({
            type L = Peano!($L);
            let mut list: NList<usize, L> = NList::from_array(konst::array::from_fn_!(
                |i| i.pow(2)
            ));

            $(
                let at: usize = $indices;
                let len = $L;
                let ret = callit(&mut list, peano!($indices));

                if at < len {
                    assert_eq!(ret, Some(&mut at.pow(2)));
                } else {
                    assert_eq!(ret, None);
                }
            )*
        })
    }

    test_case!{0 [0 1 2 3]}
    test_case!{1 [0 1 2 3]}
    test_case!{2 [0 1 2 3]}

}
