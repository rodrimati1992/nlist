use nlist::{NList, Peano, Int, nlist, peano, nlist_pat};

// ensures that fixed-length `nlist_pat`s infer the length of the NList
#[test]
fn nlist_pat_fixed_len_inference_test() {
    let nlist_pat![a, b, c] = NList::from_fn(|i| i.pow(2));

    assert_eq!(a, 0);
    assert_eq!(b, 1);
    assert_eq!(c, 4);
}

#[test]
fn nlist_pat_rem_by_value_test() {
    let nlist_pat![a, b, c, rem @ ..] = NList::<_, Peano!(7)>::from_fn(|i| i.pow(2));

    assert_eq!(a, 0);
    assert_eq!(b, 1);
    assert_eq!(c, 4);
    assert_eq!(rem, nlist![9, 16, 25, 36]);
}

#[test]
fn nlist_pat_rem_by_ref_test() {
    let nlist_pat![a, b, c, rem @ ..] = &NList::<_, Peano!(7)>::from_fn(|i| i.pow(2));

    assert_eq!(a, &0);
    assert_eq!(b, &1);
    assert_eq!(c, &4);
    assert_eq!(rem, &nlist![9, 16, 25, 36]);
}

#[test]
fn nlist_pat_rem_by_mut_ref_test() {
    let nlist_pat![a, b, c, rem @ ..] = &mut NList::<_, Peano!(7)>::from_fn(|i| i.pow(2));

    assert_eq!(a, &mut 0);
    assert_eq!(b, &mut 1);
    assert_eq!(c, &mut 4);
    assert_eq!(rem, &mut nlist![9, 16, 25, 36]);
}

#[test]
fn nlist_pat_rem_ref_patterns_test() {
    let nlist_pat![a, ref b, c, ref mut rem @ ..] = NList::<_, Peano!(7)>::from_fn(|i| i.pow(2));

    assert_eq!(a, 0);
    assert_eq!(b, &1);
    assert_eq!(c, 4);
    assert_eq!(rem, &mut nlist![9, 16, 25, 36]);
}

#[test]
fn nlist_pat_rem_ignore_test() {
    {
        let nlist_pat![a, b, _ @ ..] = NList::<_, Peano!(7)>::from_fn(|i| (i + 2).pow(2));

        assert_eq!(a, 4);
        assert_eq!(b, 9);
    }
    {
        let nlist_pat![a, b, _ @ ..,] = NList::<_, Peano!(7)>::from_fn(|i| (i + 2).pow(2));

        assert_eq!(a, 4);
        assert_eq!(b, 9);
    }
    {
        let nlist_pat![a, b, ..] = NList::<_, Peano!(7)>::from_fn(|i| (i + 3).pow(2));

        assert_eq!(a, 9);
        assert_eq!(b, 16);
    }
    {
        let nlist_pat![a, b, ..,] = NList::<_, Peano!(7)>::from_fn(|i| (i + 3).pow(2));

        assert_eq!(a, 9);
        assert_eq!(b, 16);
    }
}

#[test]
fn nlist_pat_pattern_matching() {
    let res = const {
        match nlist![3, 5, 8, 13] {
            nlist_pat![3 | 4, a @ 5, ..] => a,
            _ => unreachable!()
        }
    };

    assert_eq!(res, 5);
}

///////////////////////////////////////////////////////////////////////////////

#[test]
fn unlist_empty() {
    macro_rules! test_case {
        ($($rest:tt)*) => ({
            const fn inner<T>(list: NList<T, Peano!(0)>) {
                nlist::unlist!{[$($rest)*] = list}
            }

            inner(nlist![0u8; 0]);
        })
    }

    test_case!{}
    test_case!{,}
    test_case!{..}
    test_case!{..,}
    test_case!{_ @ ..}
    test_case!{_ @ ..,}
}

#[test]
fn unlist_one() {
    macro_rules! test_case {
        ($($rest:tt)*) => ({
            const fn inner<T>(list: NList<T, Peano!(1)>) -> T {
                nlist::unlist!{[a $($rest)*] = list}
                
                a
            }

            assert_eq!(inner(nlist![3]), 3);
        })
    }

    test_case!{}
    test_case!{,}
    test_case!{, ..}
    test_case!{, ..,}
    test_case!{, _ @ ..}
    test_case!{, _ @ ..,}
}

#[test]
fn unlist_pair() {
    macro_rules! test_case {
        ($($rest:tt)*) => ({
            const fn inner<T>(list: NList<T, Peano!(2)>) -> (T, T) {
                nlist::unlist!{[a, b $($rest)*] = list}
                
                (a, b)
            }

            assert_eq!(inner(nlist![3, 5]), (3, 5));
        })
    }

    test_case!{}
    test_case!{,}
    test_case!{, ..}
    test_case!{, ..,}
    test_case!{, _ @ ..}
    test_case!{, _ @ ..,}
}

#[test]
fn unlist_tuple_pattern() {
    macro_rules! test_case {
        ($($rest:tt)*) => ({
            const fn inner<T: Copy>(list: NList<(T, T), Peano!(2)>) -> [T; 4] {
                nlist::unlist!{[(a, b), (c, d) $($rest)*] = list}
                
                [a, b, c, d]
            }

            assert_eq!(inner(nlist![(3, 5), (8, 13)]), [3, 5, 8, 13]);
        })
    }

    test_case!{}
    test_case!{,}
    test_case!{, ..}
    test_case!{, ..,}
    test_case!{, _ @ ..}
    test_case!{, _ @ ..,}
}

#[test]
fn unlist_array_pattern() {
    macro_rules! test_case {
        ($($rest:tt)*) => ({
            const fn inner<T: Copy>(list: NList<[T; 2], Peano!(2)>) -> [T; 4] {
                nlist::unlist!{[[a, b], [c, d] $($rest)*] = list}
                
                [a, b, c, d]
            }

            assert_eq!(inner(nlist![[3, 5], [8, 13]]), [3, 5, 8, 13]);
        })
    }

    test_case!{}
    test_case!{,}
    test_case!{, ..}
    test_case!{, ..,}
    test_case!{, _ @ ..}
    test_case!{, _ @ ..,}
}

#[test]
fn unlist_split1() {
    const fn inner<T, L>(list: NList<T, peano::Add<Peano!(1), L>>) -> (T, NList<T, L>)
    where
        L: Int
    {
        nlist::unlist!{[a, b @ ..] = list}
        
        (a, b)
    }

    assert_eq!(inner(nlist![3, 5, 8, 13]), (3, nlist![5, 8, 13]));
}

#[test]
fn unlist_split2() {
    const fn inner<T, L>(list: NList<T, peano::Add<Peano!(2), L>>) -> (T, T, NList<T, L>)
    where
        L: Int
    {
        nlist::unlist!{[a, b, c @ ..] = list}
        
        (a, b, c)
    }

    assert_eq!(inner(nlist![3, 5, 8, 13]), (3, 5, nlist![8, 13]));
}



