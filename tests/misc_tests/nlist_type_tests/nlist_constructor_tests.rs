use nlist::{Cons, Peano, PeanoInt, Nil, NList, nlist};

use crate::misc_tests::test_utils::{assertm, assert_type};


#[test]
fn nil_test() {
    assert_type::<NList<(), Peano!(0)>>(NList::nil::<()>());

    {
        const NIL: NList<(), Peano!(0)> = NList::nil::<()>();
        assertm!(NIL, NList { node: Nil {..} })
    }
}

#[test]
fn cons_test() {
    assert_type::<NList<u32, Peano!(1)>>(NList::cons(3u32, NList::nil()));    
    {
        const LIST: NList<u32, Peano!(1)> = NList::cons(3u32, NList::nil());
        assertm!(
            LIST, 
            NList { 
                node: Cons {
                    elem: 3,
                    next: NList { 
                        node: Nil { .. } 
                    },
                    ..
                } 
            }
        )
    }

    assert_type::<NList<u32, Peano!(2)>>(
        NList::cons(3, NList::cons(5u32, NList::nil()))
    );
    {
        const LIST: NList<u32, Peano!(2)> = NList::cons(3u32, NList::cons(5, NList::nil()));
        assertm!(
            LIST, 
            NList { 
                node: Cons {
                    elem: 3u32,
                    next: NList { 
                        node: Cons {
                            elem: 5,
                            next: NList { 
                                node: Nil { .. } 
                            },
                            ..
                        } 
                    },
                    ..
                } 
            }
        )
    }
}

#[test]
fn nlist_macro_list_test() {
    {
        // nlist![] needs the element type to be specified somewhere,
        // and it can be anything.
        const _: NList<u8, Peano!(0)> = nlist![];
        const _: NList<u16, Peano!(0)> = nlist![];
        const _: NList<u32, Peano!(0)> = nlist![];

        const LIST: NList<u32, Peano!(0)> = nlist![];
        assert_eq!(LIST, NList::nil::<u32>());
    }
    
    {
        type List = NList<u8, Peano!(1)>;
        assert_type::<List>(nlist![13u8]);
        
        const LIST: List = nlist![13];
        assert_eq!(LIST, NList::cons(13u8, NList::nil()));
    }
    
    {
        type List = NList<u16, Peano!(2)>;
        assert_type::<List>(nlist![13u16, 21]);
        
        const LIST: List = nlist![13, 21];
        assert_eq!(LIST, NList::cons(13u16, NList::cons(21, NList::nil())));
    }

    {
        type List = NList<u32, Peano!(3)>;
        assert_type::<List>(nlist![13u32, 21, 34]);
        
        const LIST: List = nlist![13, 21, 34];
        assert_eq!(
            LIST, 
            NList::cons(
                13u32, 
                NList::cons(
                    21, 
                    NList::cons(
                        34, 
                        NList::nil()
                    )
                )
            )
        );
    }
}

#[test]
fn nlist_macro_repeat_test() {
    const fn repat<T, const N: usize>(val: T) -> NList<T, Peano!(N)>
    where
        T: Copy,
        typewit::const_marker::Usize<N>: nlist::peano::IntoPeano,
    {
        nlist![val; N]
    }

    assert_type::<NList<u64, Peano!(3)>>(nlist![3u64; 3]);

    const REPEATED: NList<&str, Peano!(7)> = repat::<_, 7>("h");
    assert_eq!(REPEATED, nlist!["h", "h", "h", "h", "h", "h", "h"]);
}

#[test]
fn nlist_macro_repeat_infer_test() {
    const fn repat<T, L>(val: T) -> NList<T, L>
    where
        T: Copy,
        L: PeanoInt,
    {
        nlist![val; _]
    }

    const REPEATED: NList<&str, Peano!(7)> = repat("h");
    assert_eq!(REPEATED, nlist!["h", "h", "h", "h", "h", "h", "h"]);
}





#[test]
fn repeat_copy_test() {
    const fn repeat_from_any_copy_type<T, L>(val: T) -> NList<T, L>
    where
        T: Copy,
        L: PeanoInt,
    {
        NList::repeat_copy(val)
    }

    assert_type::<NList<u64, Peano!(3)>>(repeat_from_any_copy_type::<_, Peano!(3)>(5u64));

    const REPEATED: NList<&str, Peano!(7)> = repeat_from_any_copy_type("h");
    assert_eq!(REPEATED, nlist!["h", "h", "h", "h", "h", "h", "h"]);

}

#[test]
fn from_fn_test() {
    assert_type::<NList<usize, Peano!(4)>>(NList::<_, Peano!(4)>::from_fn(|i| i.pow(2)));

    let list = NList::<_, Peano!(5)>::from_fn(|i| i.pow(2));
    assert_eq!(list, nlist![0, 1, 4, 9, 16]);
}

#[test]
fn rec_from_fn_test() {
    macro_rules! test_case {
        ($($macro_arg:tt)*) => ({
            const fn ctor_pair<LNext>() -> (u128, NList<u128, LNext>)
            where
                LNext: PeanoInt
            {
                ((LNext::USIZE * 3) as u128, ctor())
            }

            const fn ctor<L>() -> NList<u128, L>
            where
                L: PeanoInt
            {
                nlist::rec_from_fn!($($macro_arg)*)
            }

            assert_type::<NList<u128, Peano!(4)>>(ctor::<Peano!(4)>());

            const LIST: NList<u128, Peano!(5)> = ctor();
            assert_eq!(LIST, nlist![12, 9, 6, 3, 0]);

        })
    }

    test_case!{|| ctor_pair()}
    
    test_case!{|| -> (u128, _) { ctor_pair() }}

    test_case!{|| -> (u128, NList<u128, L::SubOneSat>) { ctor_pair() }}
    
    test_case!{ctor_pair}

}


#[test]
fn from_array_test() {
    {
        type List = NList<u8, Peano!(0)>;

        // infer NList generic args from array
        assert_type::<List>(NList::from_array([0u8; 0]));

        // infer array generic args from NList
        const LIST: List = NList::from_array(konst::array::from_fn_!(|i| (i + 1) as _));
        assert_eq!(LIST, nlist![]);
    }

    {
        type List = NList<u16, Peano!(1)>;
        
        // infer NList generic args from array
        assert_type::<List>(NList::from_array([0u16; 1]));

        // infer array generic args from NList
        const LIST: List = NList::from_array(konst::array::from_fn_!(|i| (i + 1) as _));
        assert_eq!(LIST, nlist![1]);
    }

    {
        type List = NList<u32, Peano!(2)>;
        
        // infer NList generic args from array
        assert_type::<List>(NList::from_array([0u32; 2]));

        // infer array generic args from NList
        const LIST: List = NList::from_array(konst::array::from_fn_!(|i| (i + 1) as _));
        assert_eq!(LIST, nlist![1, 2]);
    }
}





