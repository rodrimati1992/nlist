use nlist::{NList, Peano, nlist, nlist_pat};

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



