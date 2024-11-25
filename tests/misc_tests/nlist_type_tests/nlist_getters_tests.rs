use nlist::{Cons, Peano, PeanoInt, Nil, Node, NList, nlist};
use nlist::peano::PlusOne;
use nlist::receiver::{HktApply, Receiver};


use crate::misc_tests::test_utils::assert_type;


#[test]
fn into_node_test() {
    {
        assert_type::<Nil<u8, Peano!(0)>>(nlist![3u8; 0].into_node());

        const _: Nil<u8, Peano!(0)> = nlist![3u8; 0].into_node();
    }

    const fn into_cons_node<T, L>(list: NList<T, L>) -> Node<T, L>
    where
        L: PeanoInt
    {
        list.into_node()
    }

    {
        assert_type::<Cons<u16, Peano!(1)>>(into_cons_node(nlist![3u16]));

        const NODE: Cons<u16, Peano!(1)> = into_cons_node(nlist![3u16]);
        assert_eq!(NODE.elem, 3);
        assert_eq!(NODE.next, nlist![]);
    }
    {
        assert_type::<Cons<u64, Peano!(2)>>(into_cons_node(nlist![5u64, 8]));
        
        const NODE: Cons<u64, Peano!(2)> = into_cons_node(nlist![5u64, 8]);
        assert_eq!(NODE.elem, 5);
        assert_eq!(NODE.next, nlist![8]);
    }
    {
        assert_type::<Cons<i8, Peano!(3)>>(into_cons_node(nlist![13i8, 21, 34]));

        const NODE: Cons<i8, Peano!(3)> = into_cons_node(nlist![13i8, 21, 34]);
        assert_eq!(NODE.elem, 13);
        assert_eq!(NODE.next, nlist![21, 34]);
    }
}

#[test]
fn head_test() {
    // asserts usability in const and generic context
    const fn inner<T, L>(list: &NList<T, PlusOne<L>>) -> &T
    where
        L: PeanoInt
    {
        list.head()
    }

    {
        assert_type::<&u16>(inner(&nlist![3u16]));

        const NODE: NList<u16, Peano!(1)> = nlist![3u16];
        const HEAD: &u16 = inner(&NODE);
        assert_eq!(HEAD, &3);
    }
    {
        assert_type::<&u64>(inner(&nlist![5u64, 8]));
        
        const NODE: NList<u64, Peano!(2)> = nlist![5u64, 8];
        const HEAD: &u64 = inner(&NODE);
        assert_eq!(HEAD, &5);
    }
    {
        assert_type::<&i8>(inner(&nlist![13i8, 21, 34]));

        const NODE: NList<i8, Peano!(3)> = nlist![13i8, 21, 34];
        const HEAD: &i8 = inner(&NODE);
        assert_eq!(HEAD, &13);
    }
}

#[test]
fn head_mut_test() {
    // asserts usability in const and generic context
    const fn inner<T, L>(list: &mut NList<T, PlusOne<L>>) -> &mut T
    where
        L: PeanoInt
    {
        list.head_mut()
    }

    {
        assert_type::<&mut u16>(inner(&mut nlist![3u16]));

        const NODE: NList<u16, Peano!(1)> = nlist![3u16];
        const HEAD: u16 = {
            let mut node = NODE;
            let x: &mut u16 = inner(&mut node);
            *x
        };
        assert_eq!(HEAD, 3);
    }
    {
        assert_type::<&mut u64>(inner(&mut nlist![5u64, 8]));
        
        const NODE: NList<u64, Peano!(2)> = nlist![5u64, 8];
        const HEAD: u64 = {
            let mut node = NODE;
            let x: &mut u64 = inner(&mut node);
            *x
        };
        assert_eq!(HEAD, 5);
    }
    {
        assert_type::<&mut i8>(inner(&mut nlist![13i8, 21, 34]));

        const NODE: NList<i8, Peano!(3)> = nlist![13i8, 21, 34];
        const HEAD: i8 = {
            let mut node = NODE;
            let x: &mut i8 = inner(&mut node);
            *x
        };
        assert_eq!(HEAD, 13);
    }
}

#[test]
fn into_head_test() {
    fn inner<T, L>(list: NList<T, PlusOne<L>>) -> T
    where
        L: PeanoInt
    {
        list.into_head()
    }

    {
        assert_type::<u16>(inner(nlist![3u16]));

        const NODE: NList<u16, Peano!(1)> = nlist![3u16];
        let head: u16 = inner(NODE);
        assert_eq!(head, 3);
    }
    {
        assert_type::<u64>(inner(nlist![5u64, 8]));
        
        const NODE: NList<u64, Peano!(2)> = nlist![5u64, 8];
        let head: u64 = inner(NODE);
        assert_eq!(head, 5);
    }
    {
        assert_type::<i8>(inner(nlist![13i8, 21, 34]));

        const NODE: NList<i8, Peano!(3)> = nlist![13i8, 21, 34];
        let head: i8 = inner(NODE);
        assert_eq!(head, 13);
    }
}

#[test]
fn tail_test() {
    // asserts usability in const and generic context
    const fn inner<T, L>(list: &NList<T, PlusOne<L>>) -> &NList<T, L>
    where
        L: PeanoInt
    {
        list.tail()
    }

    {
        assert_type::<&NList<u16, Peano!(0)>>(inner(&nlist![3u16]));

        const NODE: NList<u16, Peano!(1)> = nlist![3u16];
        const TAIL: &NList<u16, Peano!(0)> = inner(&NODE);
        assert_eq!(TAIL, &nlist![0u16; 0]);
    }
    {
        assert_type::<&NList<u64, Peano!(1)>>(inner(&nlist![5u64, 8]));
        
        const NODE: NList<u64, Peano!(2)> = nlist![5u64, 8];
        const TAIL: &NList<u64, Peano!(1)> = inner(&NODE);
        assert_eq!(TAIL, &nlist![8u64]);
    }
    {
        assert_type::<&NList<i8, Peano!(2)>>(inner(&nlist![13i8, 21, 34]));

        const NODE: NList<i8, Peano!(3)> = nlist![13i8, 21, 34];
        const TAIL: &NList<i8, Peano!(2)> = inner(&NODE);
        assert_eq!(TAIL, &nlist![21i8, 34]);
    }
}

#[test]
fn tail_mut_test() {
    // asserts usability in const and generic context
    const fn inner<T, L>(list: &mut NList<T, PlusOne<L>>) -> &mut NList<T, L>
    where
        L: PeanoInt
    {
        list.tail_mut()
    }

    {
        assert_type::<&mut NList<u16, Peano!(0)>>(inner(&mut nlist![3u16]));

        let mut node: NList<u16, Peano!(1)> = nlist![3u16];
        let tail: &mut NList<u16, Peano!(0)> = inner(&mut node);
        assert_eq!(tail, &nlist![0u16; 0]);
    }
    {
        assert_type::<&mut NList<u64, Peano!(1)>>(inner(&mut nlist![5u64, 8]));
        
        let mut node: NList<u64, Peano!(2)> = nlist![5u64, 8];
        let tail: &mut NList<u64, Peano!(1)> = inner(&mut node);
        assert_eq!(tail, &nlist![8u64]);
    }
    {
        assert_type::<&mut NList<i8, Peano!(2)>>(inner(&mut nlist![13i8, 21, 34]));

        let mut node: NList<i8, Peano!(3)> = nlist![13i8, 21, 34];
        let tail: &mut NList<i8, Peano!(2)> = inner(&mut node);
        assert_eq!(tail, &nlist![21i8, 34]);
    }
}

macro_rules! into_tail_test_ {
    ($method:ident, $($const:ident $bound:tt)?, $binding_kind:ident $binding:ident) => {
        // asserts usability in const and generic context
        $($const)? fn inner<T, L>(list: NList<T, PlusOne<L>>) -> NList<T, L>
        where
            $(T: $bound,)?
            L: PeanoInt
        {
            list.$method()
        }

        {
            assert_type::<NList<u16, Peano!(0)>>(inner(nlist![3u16]));

            const NODE: NList<u16, Peano!(1)> = nlist![3u16];
            $binding_kind $binding: NList<u16, Peano!(0)> = inner(NODE);
            assert_eq!($binding, nlist![0u16; 0]);
        }
        {
            assert_type::<NList<u64, Peano!(1)>>(inner(nlist![5u64, 8]));
            
            const NODE: NList<u64, Peano!(2)> = nlist![5u64, 8];
            $binding_kind $binding: NList<u64, Peano!(1)> = inner(NODE);
            assert_eq!($binding, nlist![8u64]);
        }
        {
            assert_type::<NList<i8, Peano!(2)>>(inner(nlist![13i8, 21, 34]));

            const NODE: NList<i8, Peano!(3)> = nlist![13i8, 21, 34];
            $binding_kind $binding: NList<i8, Peano!(2)> = inner(NODE);
            assert_eq!($binding, nlist![21i8, 34]);
        }
    };
}

#[test]
fn into_tail_test() {
    into_tail_test_!{into_tail,, let tail}
}

#[test]
fn into_tail_const_test() {
    into_tail_test_!{into_tail_const, const Copy, const TAIL}
}

#[test]
fn split_head_test() {
    // asserts usability in const and generic context
    const fn inner<T, L>(list: &NList<T, PlusOne<L>>) -> (&T, &NList<T, L>)
    where
        L: PeanoInt
    {
        list.split_head()
    }

    {
        assert_type::<(&u16, &NList<u16, Peano!(0)>)>(inner(&nlist![3u16]));

        const NODE: NList<u16, Peano!(1)> = nlist![3u16];
        const BOTH: (&u16, &NList<u16, Peano!(0)>) = inner(&NODE);
        assert_eq!(BOTH, (&3u16, &nlist![0u16; 0]));
    }
    {
        assert_type::<(&u64, &NList<u64, Peano!(1)>)>(inner(&nlist![5u64, 8]));
        
        const NODE: NList<u64, Peano!(2)> = nlist![5u64, 8];
        const BOTH: (&u64, &NList<u64, Peano!(1)>) = inner(&NODE);
        assert_eq!(BOTH, (&5u64, &nlist![8u64]));
    }
    {
        assert_type::<(&i8, &NList<i8, Peano!(2)>)>(inner(&nlist![13i8, 21, 34]));

        const NODE: NList<i8, Peano!(3)> = nlist![13i8, 21, 34];
        const BOTH: (&i8, &NList<i8, Peano!(2)>) = inner(&NODE);
        assert_eq!(BOTH, (&13i8, &nlist![21i8, 34]));
    }
}

#[test]
fn split_head_mut_test() {
    // asserts usability in const and generic context
    const fn inner<T, L>(list: &mut NList<T, PlusOne<L>>) -> (&mut T, &mut NList<T, L>)
    where
        L: PeanoInt
    {
        list.split_head_mut()
    }

    {
        assert_type::<(&mut u16, &mut NList<u16, Peano!(0)>)>(inner(&mut nlist![3u16]));

        let mut node: NList<u16, Peano!(1)> = nlist![3u16];
        let both: (&mut u16, &mut NList<u16, Peano!(0)>) = inner(&mut node);
        assert_eq!(both, (&mut 3u16, &mut nlist![0u16; 0]));
    }
    {
        assert_type::<(&mut u64, &mut NList<u64, Peano!(1)>)>(inner(&mut nlist![5u64, 8]));
        
        let mut node: NList<u64, Peano!(2)> = nlist![5u64, 8];
        let both: (&mut u64, &mut NList<u64, Peano!(1)>) = inner(&mut node);
        assert_eq!(both, (&mut 5u64, &mut nlist![8u64]));
    }
    {
        assert_type::<(&mut i8, &mut NList<i8, Peano!(2)>)>(inner(&mut nlist![13i8, 21, 34]));

        let mut node: NList<i8, Peano!(3)> = nlist![13i8, 21, 34];
        let both: (&mut i8, &mut NList<i8, Peano!(2)>) = inner(&mut node);
        assert_eq!(both, (&mut 13i8, &mut nlist![21i8, 34]));
    }
}

#[test]
fn into_split_head_test() {
    // asserts usability in const and generic context
    const fn inner<T, L>(list: NList<T, PlusOne<L>>) -> (T, NList<T, L>)
    where
        L: PeanoInt
    {
        list.into_split_head()
    }

    {
        assert_type::<(u16, NList<u16, Peano!(0)>)>(inner(nlist![3u16]));

        const NODE: NList<u16, Peano!(1)> = nlist![3u16];
        const BOTH: (u16, NList<u16, Peano!(0)>) = inner(NODE);
        assert_eq!(BOTH, (3u16, nlist![0u16; 0]));
    }
    {
        assert_type::<(u64, NList<u64, Peano!(1)>)>(inner(nlist![5u64, 8]));
        
        const NODE: NList<u64, Peano!(2)> = nlist![5u64, 8];
        const BOTH: (u64, NList<u64, Peano!(1)>) = inner(NODE);
        assert_eq!(BOTH, (5u64, nlist![8u64]));
    }
    {
        assert_type::<(i8, NList<i8, Peano!(2)>)>(inner(nlist![13i8, 21, 34]));

        const NODE: NList<i8, Peano!(3)> = nlist![13i8, 21, 34];
        const BOTH: (i8, NList<i8, Peano!(2)>) = inner(NODE);
        assert_eq!(BOTH, (13i8, nlist![21i8, 34]));
    }
}

#[test]
fn split_head_poly_test() {
    // asserts usability in const and generic context
    const fn inner<'a, P, T, L>(
        list: P
    ) -> (
        HktApply<'a, P::Hkt, T>, 
        HktApply<'a, P::Hkt, NList<T, L>>
    ) where
        P: Receiver<'a, NList<T, PlusOne<L>>>,
        T: 'a,
        L: PeanoInt,
    {
        NList::split_head_poly(list)
    }


    {
        const NODE: NList<u16, Peano!(1)> = nlist![3u16];        

        assert_type::<(&u16, &NList<u16, Peano!(0)>)>(inner(&NODE));
        assert_type::<(&mut u16, &mut NList<u16, Peano!(0)>)>(inner(&mut {NODE}));
        assert_type::<(u16, NList<u16, Peano!(0)>)>(inner(NODE));

        const BOTH_REF: (&u16, &NList<u16, Peano!(0)>) = inner(&NODE);
        assert_eq!(BOTH_REF, (&3u16, &nlist![0u16; 0]));

        let mut node_mut = NODE;
        let both_mut: (&mut u16, &mut NList<u16, Peano!(0)>) = inner(&mut node_mut);
        assert_eq!(both_mut, (&mut 3u16, &mut nlist![0u16; 0]));

        const BOTH_VAL: (u16, NList<u16, Peano!(0)>) = inner(NODE);
        assert_eq!(BOTH_VAL, (3u16, nlist![0u16; 0]));
    }
    {
        const NODE: NList<u64, Peano!(2)> = nlist![5u64, 8];

        assert_type::<(&u64, &NList<u64, Peano!(1)>)>(inner(&NODE));
        assert_type::<(&mut u64, &mut NList<u64, Peano!(1)>)>(inner(&mut {NODE}));
        assert_type::<(u64, NList<u64, Peano!(1)>)>(inner(NODE));
                
        const BOTH_REF: (&u64, &NList<u64, Peano!(1)>) = inner(&NODE);
        assert_eq!(BOTH_REF, (&5u64, &nlist![8u64]));

        let mut node_mut = NODE;
        let both_mut: (&mut u64, &mut NList<u64, Peano!(1)>) = inner(&mut node_mut);
        assert_eq!(both_mut, (&mut 5u64, &mut nlist![8u64]));

        const BOTH_VAL: (u64, NList<u64, Peano!(1)>) = inner(NODE);
        assert_eq!(BOTH_VAL, (5u64, nlist![8u64]));
    }
    {
        const NODE: NList<i8, Peano!(3)> = nlist![13i8, 21, 34];

        assert_type::<(&i8, &NList<i8, Peano!(2)>)>(inner(&NODE));
        assert_type::<(&mut i8, &mut NList<i8, Peano!(2)>)>(inner(&mut {NODE}));
        assert_type::<(i8, NList<i8, Peano!(2)>)>(inner(NODE));

        const BOTH_REF: (&i8, &NList<i8, Peano!(2)>) = inner(&NODE);
        assert_eq!(BOTH_REF, (&13i8, &nlist![21i8, 34]));

        let mut node: NList<i8, Peano!(3)> = nlist![13i8, 21, 34];
        let both: (&mut i8, &mut NList<i8, Peano!(2)>) = inner(&mut node);
        assert_eq!(both, (&mut 13i8, &mut nlist![21i8, 34]));

        const BOTH_VAL: (i8, NList<i8, Peano!(2)>) = inner(NODE);
        assert_eq!(BOTH_VAL, (13i8, nlist![21i8, 34]));
    }
}
