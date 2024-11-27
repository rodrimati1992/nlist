use nlist::{NList, PeanoInt, Peano, nlist, peano};

use konst::option;

use core::mem::ManuallyDrop as MD;

#[test]
fn all_test() {
    // vacuous truth
    assert_eq!(NList::nil::<()>().all(|_| false), true);
    assert_eq!(NList::nil::<()>().all(|_| true), true);
    
    assert_eq!(nlist![3u8, 5, 7].all(|x: u8| x % 2 == 1), true);
    assert_eq!(nlist![3u8, 5, 8].all(|x: u8| x % 2 == 1), false);
}

#[test]
fn rec_all_test() {    
    macro_rules! test_case {
        ($list:ident ($($reffness:tt)*) => $($invocation:tt)*) => ({
            const fn all_odd<L>($list: $($reffness)* NList<u128, L>) -> bool
            where
                L: PeanoInt
            {
                $($invocation)*
            }

            // vacuous truth
            assert_eq!(all_odd($($reffness)* NList::nil()), true);
            
            assert_eq!(all_odd($($reffness)* nlist![3, 5, 7]), true);
            assert_eq!(all_odd($($reffness)* nlist![3, 5, 8]), false);            
        })
    }

    test_case!{list (&)=>
        nlist::rec_all!{list, |elem: &u128, next| *elem % 2 == 1 && all_odd(next)}
    }
    test_case!{list (&mut)=>
        nlist::rec_all!{list, |elem: &mut u128, next: &mut NList<u128, L::SubOneSat>| -> bool {
            *elem % 2 == 1 && all_odd(next)
        }}
    }
    test_case!{list () =>
        nlist::rec_all!{list, |elem, next| { 
            let next = next.assert_copy();

            elem % 2 == 1 && all_odd(MD::into_inner(next))
        }}
    }
    test_case!{list ()=>
        const fn inner<L>(elem: u128, next: NList<u128, L>) -> bool
        where
            L: PeanoInt
        {
            let next = next.assert_copy();
            
            elem % 2 == 1 && all_odd(MD::into_inner(next))
        }

        nlist::rec_all!{list, inner}
    }
}

#[test]
fn any_test() {
    assert_eq!(NList::nil::<()>().any(|_| false), false);
    assert_eq!(NList::nil::<()>().any(|_| true), false);
    
    // returns true if it finds it at any index
    assert_eq!(nlist![3u8, 4, 8].any(|x: u8| x == 3), true);
    assert_eq!(nlist![4, 3u8, 8].any(|x: u8| x == 3), true);
    assert_eq!(nlist![4, 8, 3u8].any(|x: u8| x == 3), true);

    assert_eq!(nlist![2, 4, 8].any(|x: u8| x == 3), false);
}

#[test]
fn rec_any_test() {    
    macro_rules! test_case {
        ($list:ident ($($reffness:tt)*) => $($invocation:tt)*) => ({
            const fn any_odd<L>($list: $($reffness)* NList<u128, L>) -> bool
            where
                L: PeanoInt
            {
                $($invocation)*
            }

            assert_eq!(any_odd($($reffness)* NList::nil()), false);
            
            assert_eq!(any_odd($($reffness)* nlist![3, 5, 6]), true);
            assert_eq!(any_odd($($reffness)* nlist![2, 4, 8]), false);            
        })
    }


    test_case!{list (&)=>
        nlist::rec_any!{list, |elem: &u128, next| *elem % 2 == 1 || any_odd(next)}
    }
    test_case!{list (&mut)=>
        nlist::rec_any!{list, |elem: &mut u128, next: &mut NList<u128, L::SubOneSat>| -> bool {
            *elem % 2 == 1 || any_odd(next)
        }}
    }
    test_case!{list () =>
        nlist::rec_any!{list, |elem, next| { 
            let next = next.assert_copy();

            elem % 2 == 1 || any_odd(MD::into_inner(next))
        }}
    }
    test_case!{list ()=>
        const fn inner<L>(elem: u128, next: NList<u128, L>) -> bool
        where
            L: PeanoInt
        {
            let next = next.assert_copy();
            
            elem % 2 == 1 || any_odd(MD::into_inner(next))
        }

        nlist::rec_any!{list, inner}
    }
}


#[test]
fn copy_test() {
    const fn inner<T, L>(list: &NList<T, L>) -> NList<T, L>
    where
        T: Copy,
        L: PeanoInt,
    {
        list.copy()
    }

    assert_eq!(inner(&nlist![0u8; 0]), nlist![0u8; 0]);
    assert_eq!(inner(&nlist![3]), nlist![3]);
    assert_eq!(inner(&nlist![3, 5]), nlist![3, 5]);
    assert_eq!(inner(&nlist![3, 5, 8]), nlist![3, 5, 8]);
}

#[test]
fn concat_test() {
    const fn inner<T, LA, LB>(lhs: NList<T, LA>, rhs: NList<T, LB>) -> NList<T, peano::Add<LA, LB>>
    where
        T: Copy,
        LA: PeanoInt,
        LB: PeanoInt,
    {
        lhs.concat(rhs)
    }

    macro_rules! test_case {
        ([$($elem_a:literal)*] [$($elem_b:literal)*]) => ({
            assert_eq!(
                inner::<u8, _, _>(nlist![$($elem_a,)*], nlist![$($elem_b,)*]).into_array(),
                [$($elem_a,)* $($elem_b,)*]
            );
        })
    }

    test_case!{[] []}
    test_case!{[] [1]}
    test_case!{[] [1 2]}
    
    test_case!{[1] []}
    test_case!{[1] [2]}
    test_case!{[1] [2 3]}

    test_case!{[1 2] []}
    test_case!{[1 2] [3]}
    test_case!{[1 2] [3 4]}
}

#[test]
fn find_test() {
    assert_eq!(NList::nil::<()>().find(|_| true), None::<()>);
    assert_eq!(NList::nil::<()>().find(|_| true), None::<()>);
    
    assert_eq!(nlist![3u8, 4, 8].find(|x: &u8| *x == 3), Some(3u8));
    assert_eq!(nlist![4, 3u8, 8].find(|x: &u8| *x == 3), Some(3u8));
    assert_eq!(nlist![4, 8, 3u8].find(|x: &u8| *x == 3), Some(3u8));

    assert_eq!(nlist![2, 4, 8].find(|x: &u8| *x == 3), None::<u8>);
}

#[test]
fn find_map_test() {
    assert_eq!(NList::nil::<()>().find_map(|_| Some(())), None::<()>);
    assert_eq!(NList::nil::<()>().find_map(|_| Some(())), None::<()>);
    
    assert_eq!(nlist![3u8, 5, 8].find_map(|x: u8| 4u8.checked_sub(x)), Some(1u8));
    assert_eq!(nlist![5, 2u8, 8].find_map(|x: u8| 4u8.checked_sub(x)), Some(2u8));
    assert_eq!(nlist![5, 8, 1u8].find_map(|x: u8| 4u8.checked_sub(x)), Some(3u8));

    assert_eq!(nlist![5, 8, 13].find_map(|x: u8| 4u8.checked_sub(x)), None::<u8>);
}

#[test]
fn rec_find_map_test() {
    macro_rules! test_case {
        ($list:ident ($($reffness:tt)*) => $($invocation:tt)*) => ({
            const fn four_sub<L>($list: $($reffness)* NList<u8, L>) -> Option<u128>
            where
                L: PeanoInt
            {
                $($invocation)*
            }

            assert_eq!(four_sub($($reffness)* NList::nil()), None);
            
            assert_eq!(four_sub($($reffness)* nlist![3, 5, 6]), Some(1));
            assert_eq!(four_sub($($reffness)* nlist![5, 8, 13]), None);
        })
    }


    test_case!{list (&)=>
        nlist::rec_find_map!{
            list, 
            |elem: &u8, next| 
                option::or_else!(
                    4u128.checked_sub(*elem as u128),
                    || four_sub(next)
                )
        }
    }
    test_case!{list (&mut)=>
        nlist::rec_find_map!{
            list, 
            |elem: &mut u8, next: &mut NList<u8, L::SubOneSat>| -> Option<u128> {
                let elem = *elem as u128;

                option::or_else!(4u128.checked_sub(elem), || four_sub(next))
            }
        }
    }
    test_case!{list () =>
        nlist::rec_find_map!{list, |elem, next| { 
            let next = next.assert_copy();
            let elem = elem as u128;

            option::or_else!(4u128.checked_sub(elem), || four_sub(MD::into_inner(next)))
        }}
    }
    test_case!{list ()=>
        const fn inner<L>(elem: u8, next: NList<u8, L>) -> Option<u128>
        where
            L: PeanoInt
        {
            let next = next.assert_copy();
            let elem = elem as u128;

            option::or_else!(4u128.checked_sub(elem), || four_sub(MD::into_inner(next)))
        }

        nlist::rec_find_map!{list, inner}
    }
}

#[test]
fn flatten_test() {
    const fn inner<T, L, L2>(list: NList<NList<T, L2>, L>) -> NList<T, peano::Mul<L, L2>>
    where
        L: PeanoInt,
        L2: PeanoInt,
    {
        list.flatten()
    }

    macro_rules! test_case {
        (
            $L:literal $L2:literal 
            [$([$($elem:literal)*])*]
        ) => {
            let nested: NList<NList<u32, Peano!($L2)>, Peano!($L)> = nlist![
                $(nlist![ $($elem,)* ],)*
            ];
            
            let expected: NList<u32, _> = nlist![
                $($($elem,)*)*
            ];

            assert_eq!(inner(nested), expected)
        }
    }

    test_case! {0 0 []}
    test_case! {0 1 []}
    test_case! {0 2 []}
    test_case! {0 3 []}

    test_case! {1 0 [[]]}
    test_case! {1 1 [[3]]}
    test_case! {1 2 [[3 5]]}
    test_case! {1 3 [[3 5 8]]}

    test_case! {2 0 [[] []]}
    test_case! {2 1 [[3] [5]]}
    test_case! {2 2 [[3 5] [8 13]]}
    test_case! {2 3 [[3 5 8] [13 21 34]]}
}

#[test]
fn fold_test() {
    assert_eq!(
        nlist![].fold(0u128, |accum, elem: u8| accum + u128::from(elem)), 
        0,
    );

    assert_eq!(
        nlist![3u8].fold(0u128, |accum, elem: u8| accum + u128::from(elem)), 
        3,
    );

    assert_eq!(
        nlist![3u8, 5].fold(0u128, |accum, elem| accum + u128::from(elem)), 
        8,
    );

    assert_eq!(
        nlist![3u8, 5, 8].fold(0, |accum: u128, elem| accum + u128::from(elem)), 
        16u128,
    );
}

#[test]
fn fold_rec_test() {
    macro_rules! test_case {
        ($list:ident ($($reffness:tt)*) => $($invocation:tt)*) => ({
            const fn add_up<L>($list: $($reffness)* NList<u8, L>) -> u128
            where
                L: PeanoInt
            {
                $($invocation)*
            }

            assert_eq!(add_up($($reffness)* NList::nil()), 100);
            
            assert_eq!(add_up($($reffness)* nlist![3, 5, 6]), 114);
            assert_eq!(add_up($($reffness)* nlist![5, 8, 13]), 126);
        })
    }

    test_case!{list (&)=>
        nlist::rec_fold!{list, 100, |elem: &u8, next| *elem as u128 + add_up(next)}
    }
    test_case!{list (&mut)=>
        nlist::rec_fold!{
            list, 
            100,
            |elem: &mut u8, next: &mut NList<u8, L::SubOneSat>| -> u128 {
                *elem as u128 + add_up(next)
            }
        }
    }
    test_case!{list () =>
        nlist::rec_fold!{list, 100, |elem, next| {
            elem as u128 + add_up(next)
        }}
    }
    test_case!{list ()=>
        const fn inner<L>(elem: u8, next: NList<u8, L>) -> u128
        where
            L: PeanoInt
        {
            elem as u128 + add_up(next)
        }

        nlist::rec_fold!{list, 100, inner}
    }
}

#[test]
fn for_each_test() {
    macro_rules! test_case {
        ($(($i:literal $elems:literal))*) => ({
            let list: NList<u8, _> = nlist![$($elems,)*];

            let mut vect = Vec::new();

            list.for_each(|i: usize, elem: u8| vect.push((i, elem)));

            assert_eq!(vect, vec![$(($i, $elems),)*]);
        })
    }

    test_case!{}
    test_case!{(0 3u8)}
    test_case!{(0 3u8) (1 5)}
    test_case!{(0 3u8) (1 5) (2 8)}
}

#[test]
fn for_each_rec_test() {
    macro_rules! test_case {
        ($list:ident $accum:ident ($($const:ident)?) ($($reffness:tt)*) => $($invocation:tt)*) => ({
            #[allow(unused_parens)]
            $($const)? fn add_up<L>($list: $($reffness)* NList<u8, L>, $accum: &mut u128)
            where
                L: PeanoInt
            {
                $($invocation)*
            }

            let mut accum;

            accum = 0;
            add_up($($reffness)* NList::nil(), &mut accum);
            assert_eq!(accum, 0);
            
            accum = 0;
            add_up($($reffness)* nlist![3, 5, 6], &mut accum);
            assert_eq!(accum, 14);
            
            accum = 0;
            add_up($($reffness)* nlist![5, 8, 13], &mut accum);
            assert_eq!(accum, 26);
        })
    }

    test_case!{list accum (const) (&)=>
        nlist::rec_for_each!{list, |elem: &u8, next| ({
            *accum += *elem as u128;
            add_up(next, accum);
        })}
    }
    test_case!{list accum (const) (&mut)=>
        nlist::rec_for_each!{
            list, 
            |elem: &mut u8, next: &mut NList<u8, L::SubOneSat>| -> () {
                *accum += *elem as u128;
                add_up(next, accum);
            }
        }
    }
    test_case!{list accum (const) () =>
        nlist::rec_for_each!{list, |elem, next| {
            *accum += elem as u128;
            add_up(next, accum);
        }}
    }
    // this macro needs passing closures this way to work, even if it only works at runtime
    test_case!{list accum (/* non-const */) ()=>
        let inner = |elem: u8, next: NList<u8, L::SubOneSat>| -> () {
            *accum += elem as u128;
            add_up(next, accum);
        };

        nlist::rec_for_each!{list, inner}
    }
}

#[test]
fn map_test() {


    macro_rules! test_case {
        ($list:ident ($($reffness:tt)*) => $($invocation:tt)*) => ({
            const fn double<L>($list: $($reffness)* NList<u8, L>) -> NList<u16, L>
            where
                L: PeanoInt
            {
                $($invocation)*
            }

            assert_eq!(double($($reffness)*nlist![]), nlist![0u16; 0]);

            assert_eq!(double($($reffness)*nlist![3u8]), nlist![6u16]);

            assert_eq!(double($($reffness)*nlist![3u8, 5]), nlist![6u16, 10]);

            assert_eq!(double($($reffness)*nlist![3u8, 5, 8]), nlist![6u16, 10, 16]);
        })
    }


    test_case!{list (&)=>
        nlist::rec_map!{
            list, 
            |elem: &u8, next| (*elem as u16 * 2, double(next))
        }
    }
    test_case!{list (&mut)=>
        nlist::rec_map!{
            list, 
            |elem: &mut u8, next: &mut NList<u8, L::SubOneSat>| 
            -> (u16, NList<u16, L::SubOneSat>)
            {
                (*elem as u16 * 2, double(next))
            }
        }
    }
    test_case!{list () =>
        nlist::rec_map!{list, |elem, next| { 
            (elem as u16 * 2, double(next))
        }}
    }
    test_case!{list ()=>
        const fn inner<L>(elem: u8, next: NList<u8, L>) -> (u16, NList<u16, L>)
        where
            L: PeanoInt
        {
            (elem as u16 * 2, double(next))
        }

        nlist::rec_map!{list, inner}
    }
}

#[test]
fn position_test() {
    assert_eq!(NList::nil::<()>().position(|_| true), None::<usize>);
    assert_eq!(NList::nil::<()>().position(|_| true), None::<usize>);
    
    assert_eq!(nlist![3u8, 4, 8].position(|x: u8| x == 3), Some(0usize));
    assert_eq!(nlist![4, 3u8, 8].position(|x: u8| x == 3), Some(1usize));
    assert_eq!(nlist![4, 8, 3u8].position(|x: u8| x == 3), Some(2usize));

    assert_eq!(nlist![2, 4, 8].position(|x: u8| x == 3), None::<usize>);
}

#[test]
fn reduce_test() {
    assert_eq!(nlist![3u16].reduce(|l: u16, r| l + r),  3u16);

    assert_eq!(nlist![3u32, 5].reduce(|l, r| l + r),  8u32);

    assert_eq!(nlist![3u64, 5, 8].reduce(|l: u64, r: u64| l + r),  16u64);
}

#[test]
fn reverse_test() {
    assert_eq!(nlist![0u8; 0].reverse(), nlist![0u8; 0]);
    assert_eq!(nlist![3u16].reverse(), nlist![3u16]);
    assert_eq!(nlist![3u32, 5].reverse(), nlist![5, 3u32]);
    assert_eq!(nlist![3u64, 5, 8].reverse(), nlist![8, 5, 3u64]);
}

#[test]
fn rfind_test() {
    assert_eq!(NList::nil::<()>().rfind(|_| true), None::<()>);
    assert_eq!(NList::nil::<()>().rfind(|_| true), None::<()>);
    
    assert_eq!(nlist![1, 2, 5, 7u8, 4, 8].rfind(|x: &u8| *x % 2 == 1), Some(7u8));
    assert_eq!(nlist![1, 2, 3, 5, 3u8, 8].rfind(|x: &u8| *x % 2 == 1), Some(3u8));
    assert_eq!(nlist![1, 3, 0, 4, 8, 5u8].rfind(|x: &u8| *x % 2 == 1), Some(5u8));

    assert_eq!(nlist![2, 4, 8].rfind(|x: &u8| *x % 2 == 1), None::<u8>);
}

#[test]
fn rfind_map_test() {
    assert_eq!(NList::nil::<()>().rfind_map(|_| Some(())), None::<()>);
    assert_eq!(NList::nil::<()>().rfind_map(|_| Some(())), None::<()>);
    
    assert_eq!(nlist![3u8, 5, 8].rfind_map(|x: u8| 4u8.checked_sub(x)), Some(1u8));
    assert_eq!(nlist![0, 2u8, 8].rfind_map(|x: u8| 4u8.checked_sub(x)), Some(2u8));
    assert_eq!(nlist![0, 0, 1u8].rfind_map(|x: u8| 4u8.checked_sub(x)), Some(3u8));

    assert_eq!(nlist![5, 8, 13].rfind_map(|x: u8| 4u8.checked_sub(x)), None::<u8>);
}

#[test]
fn rposition_test() {
    assert_eq!(NList::nil::<()>().rposition(|_| true), None::<usize>);
    assert_eq!(NList::nil::<()>().rposition(|_| true), None::<usize>);
    
    assert_eq!(nlist![1, 2, 5, 7u8, 4, 8].rposition(|x: u8| x % 2 == 1), Some(3usize));
    assert_eq!(nlist![1, 2, 3, 5, 3u8, 8].rposition(|x: u8| x % 2 == 1), Some(4usize));
    assert_eq!(nlist![1, 3, 0, 4, 8, 5u8].rposition(|x: u8| x % 2 == 1), Some(5usize));

    assert_eq!(nlist![2, 4, 8].rposition(|x: u8| x % 2 == 1), None::<usize>);
}

#[test]
fn zip_test() {
    assert_eq!(nlist![(); 0].zip(nlist![0u8; 0]), nlist![((), 0u8); 0]);
    assert_eq!(nlist![1u8].zip(nlist![2u32]), nlist![(1, 2)]);
    assert_eq!(nlist![1u8, 3].zip(nlist![2u64, 4]), nlist![(1, 2), (3, 4)]);
    assert_eq!(nlist![1u8, 3, 5].zip(nlist![2u128, 4, 6]), nlist![(1, 2), (3, 4), (5, 6)]);
}

