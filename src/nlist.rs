use konst::destructure;

use typewit::{TypeCmp, TypeEq};


use core::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::ManuallyDrop,
};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::{
    bit::Bit,
    int::{self, proofs, FromUsize, IntoInt, IntoUsize, Int, IntWit, Nat, SubOneSat, Usize, Zeros},
    receiver::{HktApply, MapReceiverFn, Receiver, ReceiverWit},
};

////////////////////////////////////////////////////////////////////////////////

typewit::type_fn! {
    /// Type-level function (`typewit::TypeFn` implementor)
    /// from `L` to `NList<T, L>`
    pub struct NListFn<T>;

    impl<L: Int> L => NList<T, L>;
}

typewit::type_fn! {
    struct NodeFn<T>;

    impl<L: Int> L => Node<T, L>;
}

////////////////////////////////////////////////////////////////////////////////

/// Inline-allocated list of `T`
/// which statically tracks its length using the `L` type parameter.
pub struct NList<T, L: Int> {
    /// The first node in the list
    pub node: Node<T, L>,
}

/// Alias for an `NList` of `NList`s
pub type NList2D<T, LOuter, LInner> = NList<NList<T, LInner>, LOuter>;

/// A node of an empty [`NList`]
pub struct Nil<T, L>(TypeEq<L, Zeros>, PhantomData<T>);

/// A node of [`NList`] with one element and the rest of the list.
pub struct Cons<T, L: Int> {
    /// The element of this node
    pub elem: T,
    /// The rest of the list
    pub next: NList<T, L::SubOneSat>,
    #[allow(dead_code)]
    // assertion that `L == Nat<_>`, it doesn't matter if it's used.
    len_te: TypeEq<L, Nat<L::ShrOne, L::BitArg>>,
}

/// The type of the head node in `NList<T, L>`.
///
/// If `L` is:
/// - `Zeros`: this evaluates to `Nil<T, L>`
/// - `Nat<_>`: this evaluates to `Cons<T, L>`
///
pub type Node<T, L> = int::IfZero<L, Nil<T, L>, Cons<T, L>>;

////////////////////////////////////////////////////////////////////////////////

impl NList<(), Zeros> {
    /// Constructs an empty `NList`
    ///  
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{NList, nlist};
    /// 
    /// let list = NList::nil::<u8>();
    /// 
    /// assert_eq!(list, nlist![]);
    /// assert_eq!(list.into_array(), []);
    /// 
    /// ```
    /// 
    pub const fn nil<T>() -> NList<T, Zeros> {
        NList {
            node: Nil(TypeEq::NEW, PhantomData),
        }
    }
}

impl<T> NList<T, Zeros> {
    /// Constructs an `NList` with the head element, and the rest of the list.
    /// 
    /// Use this over [`cons_sub`] when the length is a concrete type
    /// or determined only by the argument.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{NList, nlist};
    /// 
    /// let list_1 = NList::cons(3, NList::nil());
    /// assert_eq!(list_1, nlist![3]);
    /// assert_eq!(list_1.copy().into_array(), [3]);
    /// 
    /// let list_2 = NList::cons(5, list_1);
    /// assert_eq!(list_2, nlist![5, 3]);
    /// assert_eq!(list_2.copy().into_array(), [5, 3]);
    /// 
    /// let list_3 = NList::cons(8, list_2);
    /// assert_eq!(list_3, nlist![8, 5, 3]);
    /// assert_eq!(list_3.copy().into_array(), [8, 5, 3]);
    /// ```
    /// 
    pub const fn cons<L: Int>(val: T, next: NList<T, L>) -> NList<T, int::AddOne<L>> {
        NList {
            node: Cons {
                elem: val,
                next: next.coerce_len(proofs::add_sub_1_identity::<L>().flip()),
                len_te: TypeEq::NEW,
            },
        }
    }
}

impl<T, NextBits: Int, B: Bit> NList<T, Nat<NextBits, B>> {
    /// Constructs an `NList` with the head element, and the rest of the list.
    /// 
    /// Use this over [`cons`] when you need to construct an NList in a 
    /// context where the length is generic.
    pub const fn cons_alt(val: T, next: NList<T, SubOneSat<Nat<NextBits, B>>>) -> Self {
        NList {
            node: Cons {
                elem: val,
                next,
                len_te: TypeEq::NEW,
            },
        }
    }
}

impl<T, L: Int> NList<T, L> {
    /// Constructs a list by repeating `elem`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist, Int};
    ///
    /// let nlist: NList<_, Int!(4)> = NList::repeat_copy(3);
    ///
    /// assert_eq!(nlist, nlist![3, 3, 3, 3]);
    /// assert_eq!(nlist, nlist![3; 4]);
    ///
    /// ```
    pub const fn repeat_copy(elem: T) -> Self
    where
        T: Copy,
    {
        match L::INT_WIT {
            IntWit::Zeros(len_te) => NList::nil_sub(len_te),

            IntWit::Nat(len_te) => NList::cons_sub(elem, NList::repeat_copy(elem), len_te),
        }
    }

    // TODO: uncomment when iterator methods are compile
    // /// Constructs a list by calling `f` with the index of each element.
    // ///
    // /// # Alternatives
    // ///
    // /// You can use the [`rec_from_fn`](crate::rec_from_fn)
    // /// macro to emulate this function with a const function. 
    // ///
    // /// # Example
    // ///
    // /// ```rust
    // /// use nlist::{NList, nlist, Int};
    // ///
    // /// let list: NList<_, Int!(4)> = NList::from_fn(|i| i.pow(2));
    // ///
    // /// assert_eq!(list, nlist![0, 1, 4, 9]);
    // /// ```
    // pub fn from_fn<F>(f: F) -> Self
    // where
    //     F: FnMut(usize) -> T,
    // {
    //     const { index_list() }.map(f)
    // }

    /// Constructs an NList from an array
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, Int, nlist};
    ///
    /// const LIST: NList<u8, Int!(5)> = NList::from_array([3, 5, 8, 13, 21]);
    ///
    /// assert_eq!(LIST, nlist![3, 5, 8, 13, 21]);
    ///
    /// ```
    pub const fn from_array<const N: usize>(array: [T; N]) -> Self
    where
        Usize<N>: IntoInt<Int = L>,
        L: IntoUsize<Usize = Usize<N>>,
    {
        let mut array = konst::array::map_!(array, |x| Some(ManuallyDrop::new(x)));

        const fn inner<T, const N: usize, L>(
            i: usize, 
            array: &mut [Option<ManuallyDrop<T>>; N]
        ) -> NList<T, L> 
        where
            L: Int
        {
            match L::INT_WIT {
                IntWit::Zeros(len_te) => NList::nil_sub(len_te),
                IntWit::Nat(len_te) => {
                    let elem = array[i].take().expect("all elements are only visited once");
                    let elem = ManuallyDrop::into_inner(elem);

                    NList::cons_sub(elem, inner(i + 1, array), len_te)
                }
            }
        }

        inner(0, &mut array)
    }

    /// Alternate constructor for [`NList::nil`],
    /// for constructing an empty `NList` in a generic context.
    const fn nil_sub(len_te: TypeEq<L, Zeros>) -> Self {
        NList::nil().coerce_len(len_te.flip())
    }

    /// Alternate constructor for [`NList::cons`],
    /// for constructing a `NList` out of the tail of another `NList`
    /// in a generic context.
    const fn cons_sub<L2: Int>(
        val: T,
        next: NList<T, SubOneSat<L2>>,
        len_te: TypeEq<L, L2>,
    ) -> Self {
        NList::cons(val, next).coerce_len(len_te.flip())
    }
}

////////////////////////////////////////////

/// `Copy` can't be implemented for NList,
/// you can however use the [`copy`](NList::copy) method.
impl<T, L> Clone for NList<T, L>
where
    T: Clone,
    L: Int,
{
    fn clone(&self) -> Self {
        self.each_ref().map(T::clone)
    }
}

impl<T, L> Debug for NList<T, L>
where
    T: Debug,
    L: Int,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = fmt.debug_list();

        self.each_ref().for_each(|_, elem| {
            _ = fmt.entry(elem);
        });

        fmt.finish()
    }
}

impl<T, L> Hash for NList<T, L>
where
    T: Hash,
    L: Int,
{
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // hack to prepend the length of the list to the hasher
        const { [(); usize::MAX].split_at(L::USIZE).0 }.hash(hasher);

        self.each_ref().for_each(|_, elem| elem.hash(hasher));
    }
}

impl<T, L> Default for NList<T, L>
where
    T: Default,
    L: Int,
{
    fn default() -> Self {
        Self::from_fn(|_| T::default())
    }
}

impl<T, U, L, L2> PartialEq<NList<U, L2>> for NList<T, L>
where
    T: PartialEq<U>,
    L: Int,
    L2: Int,
{
    fn eq(&self, rhs: &NList<U, L2>) -> bool {
        let TypeCmp::Eq(te_len) = int::eq::<L, L2>() else {
            return false;
        };

        let rhs = te_len.map(NListFn::NEW).in_ref().to_left(rhs);

        self.each_ref().zip(rhs.each_ref()).all(|(l, r)| l == r)
    }
}

impl<T, L> NList<T, L>
where
    L: Int,
{
    /// Total equality comparison between [`NList`]s of potentially different lengths.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{NList, nlist};
    /// 
    /// assert!(nlist![3, 5].total_eq(&nlist![3, 5]));
    /// 
    /// assert!(!nlist![3, 5].total_eq(&nlist![3]));
    /// assert!(!nlist![3, 5].total_eq(&nlist![3, 0]));
    /// assert!(!nlist![3, 5].total_eq(&nlist![3, 5, 8]));
    /// ```
    pub fn total_eq<L2>(&self, rhs: &NList<T, L2>) -> bool
    where
        T: Eq,
        L2: Int,
    {
        self == rhs
    }
}

impl<T, L> Eq for NList<T, L>
where
    T: Eq,
    L: Int,
{
}

impl<T, U, L, L2> PartialOrd<NList<U, L2>> for NList<T, L>
where
    T: PartialOrd<U>,
    L: Int,
    L2: Int,
{
    fn partial_cmp(&self, rhs: &NList<U, L2>) -> Option<Ordering> {
        match (L::INT_WIT, L2::INT_WIT) {
            (IntWit::Zeros { .. }, IntWit::Zeros { .. }) => Some(Ordering::Equal),
            (IntWit::Zeros { .. }, IntWit::Nat { .. }) => Some(Ordering::Less),
            (IntWit::Nat { .. }, IntWit::Zeros { .. }) => Some(Ordering::Greater),
            (
                IntWit::Nat(l_len_te),
                IntWit::Nat(r_len_te),
            ) => {
                let lhs = &self.as_coerce_len(l_len_te).node;
                let rhs = &rhs.as_coerce_len(r_len_te).node;

                match lhs.elem.partial_cmp(&rhs.elem) {
                    Some(Ordering::Equal) => lhs.next.partial_cmp(&rhs.next),
                    other => other,
                }
            }
        }
    }
}

impl<T, L> NList<T, L>
where
    L: Int,
{
    /// Inherent version of [`Ord::cmp`], for comparing [`NList`]s of different lengths.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{NList, nlist};
    /// use core::cmp::Ordering;
    /// 
    /// // shorter-argument
    /// assert_eq!(nlist![3, 5].cmp(&nlist![1]), Ordering::Greater);
    /// assert_eq!(nlist![3, 5].cmp(&nlist![3]), Ordering::Greater);
    /// assert_eq!(nlist![3, 5].cmp(&nlist![8]), Ordering::Less);
    /// 
    /// // same-length argument
    /// assert_eq!(nlist![3, 5].cmp(&nlist![3, 1]), Ordering::Greater);
    /// assert_eq!(nlist![3, 5].cmp(&nlist![3, 5]), Ordering::Equal);
    /// assert_eq!(nlist![3, 5].cmp(&nlist![3, 8]), Ordering::Less);
    ///
    /// // longer-argument
    /// assert_eq!(nlist![3, 5].cmp(&nlist![3, 4, 0]), Ordering::Greater);
    /// assert_eq!(nlist![3, 5].cmp(&nlist![3, 5, 0]), Ordering::Less);
    /// assert_eq!(nlist![3, 5].cmp(&nlist![3, 6, 0]), Ordering::Less);
    /// ```
    pub fn cmp<L2>(&self, rhs: &NList<T, L2>) -> Ordering
    where
        T: Ord,
        L2: Int,
    {
        self.inherent_cmp(rhs)
    }

    fn inherent_cmp<L2>(&self, rhs: &NList<T, L2>) -> Ordering
    where
        T: Ord,
        L2: Int,
    {
        match (L::INT_WIT, L2::INT_WIT) {
            (IntWit::Zeros { .. }, IntWit::Zeros { .. }) => Ordering::Equal,
            (IntWit::Zeros { .. }, IntWit::Nat { .. }) => Ordering::Less,
            (IntWit::Nat { .. }, IntWit::Zeros { .. }) => Ordering::Greater,
            (
                IntWit::Nat(l_len_te),
                IntWit::Nat(r_len_te),
            ) => {
                let lhs = &self.as_coerce_len(l_len_te).node;
                let rhs = &rhs.as_coerce_len(r_len_te).node;

                match lhs.elem.cmp(&rhs.elem) {
                    Ordering::Equal => lhs.next.inherent_cmp(&rhs.next),
                    other => other,
                }
            }
        }
    }
}

/// For calling `cmp` on `NList`s of different lengths,
/// there is an inherent `cmp` method.
impl<T, L> Ord for NList<T, L>
where
    T: Ord,
    L: Int,
{
    fn cmp(&self, rhs: &NList<T, L>) -> Ordering {
        self.inherent_cmp(rhs)
    }
}

////////////////////////////////////////////

impl<T, L: Int> NList<T, L> {
    /// Gets the first [`Node`] of this `NList` by value.
    /// 
    pub const fn into_node(self) -> Node<T, L> {
        destructure!{Self{node} = self}

        node
    }
}
impl<T, NextBits: Int, B: Bit> NList<T, Nat<NextBits, B>> {
    /// Returns a reference to the first element of the list
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list_a = nlist![3];
    /// assert_eq!(list_a.head(), &3);
    ///
    /// let list_b = nlist![5, 3];
    /// assert_eq!(list_b.head(), &5);
    ///
    /// let list_c = nlist![8, 5, 3];
    /// assert_eq!(list_c.head(), &8);
    ///
    /// ```
    pub const fn head(&self) -> &T {
        &self.node.elem
    }

    /// Returns a mutable reference ot the first element of the list
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let mut list_a = nlist![3];
    /// assert_eq!(list_a.head_mut(), &mut 3);
    ///
    /// let mut list_b = nlist![5, 3];
    /// assert_eq!(list_b.head_mut(), &mut 5);
    ///
    /// let mut list_c = nlist![8, 5, 3];
    /// assert_eq!(list_c.head_mut(), &mut 8);
    ///
    /// ```
    pub const fn head_mut(&mut self) -> &mut T {
        &mut self.node.elem
    }

    /// Returns the first element of the list by value
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list_a = nlist![3];
    /// assert_eq!(list_a.into_head(), 3);
    ///
    /// let list_b = nlist![5, 3];
    /// assert_eq!(list_b.into_head(), 5);
    ///
    /// let list_c = nlist![8, 5, 3];
    /// assert_eq!(list_c.into_head(), 8);
    ///
    /// ```
    pub fn into_head(self) -> T {
        self.node.elem
    }

    /// Const alternative of [`into_head`](Self::into_head), 
    /// returns the first element of the list by value.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{NList, Int, nlist};
    ///
    /// const FIRST_A: u32 = nlist![3].into_head_const();
    /// assert_eq!(FIRST_A, 3);
    ///
    /// const FIRST_B: u32 = nlist![5, 3].into_head_const();
    /// assert_eq!(FIRST_B, 5);
    ///
    /// const FIRST_C: u32 = nlist![8, 5, 3].into_head_const();
    /// assert_eq!(FIRST_C, 8);
    ///
    /// ```
    pub const fn into_head_const(self) -> T
    where
        T: Copy
    {
        destructure!{Self{node} = self}
        destructure!{Cons{elem, next, len_te: _} = node}
        next.assert_copy_drop();
        
        elem
    }

    /// Returns a reference to the remainder of the list
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list_a = nlist![3];
    /// assert_eq!(list_a.tail(), &nlist![]);
    ///
    /// let list_b = nlist![5, 3];
    /// assert_eq!(list_b.tail(), &nlist![3]);
    ///
    /// let list_c = nlist![8, 5, 3];
    /// assert_eq!(list_c.tail(), &nlist![5, 3]);
    ///
    /// ```
    pub const fn tail(&self) -> &NList<T, SubOneSat<Nat<NextBits, B>>> {
        &self.node.next
    }

    /// Returns a mutable reference to the remainder of the list
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let mut list_a = nlist![3];
    /// assert_eq!(list_a.tail_mut(), &mut nlist![]);
    ///
    /// let mut list_b = nlist![5, 3];
    /// assert_eq!(list_b.tail_mut(), &mut nlist![3]);
    ///
    /// let mut list_c = nlist![8, 5, 3];
    /// assert_eq!(list_c.tail_mut(), &mut nlist![5, 3]);
    ///
    /// ```
    pub const fn tail_mut(&mut self) -> &mut NList<T, SubOneSat<Nat<NextBits, B>>> {
        &mut self.node.next
    }

    /// Returns the remainder of the list by value
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list_a = nlist![3];
    /// assert_eq!(list_a.into_tail(), nlist![]);
    ///
    /// let list_b = nlist![5, 3];
    /// assert_eq!(list_b.into_tail(), nlist![3]);
    ///
    /// let list_c = nlist![8, 5, 3];
    /// assert_eq!(list_c.into_tail(), nlist![5, 3]);
    ///
    /// ```
    pub fn into_tail(self) -> NList<T, SubOneSat<Nat<NextBits, B>>> {
        self.node.next
    }

    /// Const alternative of [`into_tail`](Self::into_tail), 
    /// returns the remainder of the list by value.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{NList, Int, nlist};
    ///
    /// const LIST_A: NList<u32, Int!(0)> = nlist![3].into_tail_const();
    /// assert_eq!(LIST_A, nlist![]);
    ///
    /// const LIST_B: NList<u32, Int!(1)> = nlist![5, 3].into_tail_const();
    /// assert_eq!(LIST_B, nlist![3]);
    ///
    /// const LIST_C: NList<u32, Int!(2)> = nlist![8, 5, 3].into_tail_const();
    /// assert_eq!(LIST_C, nlist![5, 3]);
    ///
    /// ```
    pub const fn into_tail_const(self) -> NList<T, SubOneSat<Nat<NextBits, B>>>
    where
        T: Copy
    {
        destructure!{Self{node} = self}
        destructure!{Cons{elem: _, next, len_te: _} = node}
        
        next
    }

    /// Returns a pair of references to the first element and the remainder of the list
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list_a = nlist![3];
    /// assert_eq!(list_a.split_head(), (&3, &nlist![]));
    ///
    /// let list_b = nlist![5, 3];
    /// assert_eq!(list_b.split_head(), (&5, &nlist![3]));
    ///
    /// let list_c = nlist![8, 5, 3];
    /// assert_eq!(list_c.split_head(), (&8, &nlist![5, 3]));
    ///
    /// ```
    pub const fn split_head(&self) -> (&T, &NList<T, SubOneSat<Nat<NextBits, B>>>) {
        let Cons { elem, next, .. } = &self.node;

        (elem, next)
    }

    /// Returns a pair of mutable references to the first element and the remainder of the list
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let mut list_a = nlist![3];
    /// assert_eq!(list_a.split_head_mut(), (&mut 3, &mut nlist![]));
    ///
    /// let mut list_b = nlist![5, 3];
    /// assert_eq!(list_b.split_head_mut(), (&mut 5, &mut nlist![3]));
    ///
    /// let mut list_c = nlist![8, 5, 3];
    /// assert_eq!(list_c.split_head_mut(), (&mut 8, &mut nlist![5, 3]));
    ///
    /// ```
    pub const fn split_head_mut(
        &mut self
    ) -> (&mut T, &mut NList<T, SubOneSat<Nat<NextBits, B>>>) {
        let Cons { elem, next, .. } = &mut self.node;

        (elem, next)
    }

    /// Returns a by-value pair of first element and the remainder of the list
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list_a = nlist![3];
    /// assert_eq!(list_a.into_split_head(), (3, nlist![]));
    ///
    /// let list_b = nlist![5, 3];
    /// assert_eq!(list_b.into_split_head(), (5, nlist![3]));
    ///
    /// let list_c = nlist![8, 5, 3];
    /// assert_eq!(list_c.into_split_head(), (8, nlist![5, 3]));
    ///
    /// ```
    pub const fn into_split_head(self) -> (T, NList<T, SubOneSat<Nat<NextBits, B>>>) {
        destructure!{Self{node} = self}
        destructure!{Cons{elem, next, len_te: _} = node}

        (elem, next)
    }

    /// Generic version of `split_head` that can take 
    /// `NList` by value/reference/mutable reference,
    /// and returns the corresponding pair of (head, tail).
    /// 
    /// `P` and the return type can only be these:
    /// - If `P == NList<T, Nat<L>>`: 
    /// the return type is `(T, NList<T, L>)`
    /// - If `P == &'a NList<T, Nat<L>>`: 
    /// the return type is `(&'a T, &'a NList<T, L>)`
    /// - If `P == &'a mut NList<T, Nat<L>>`: 
    /// the return type is `(&'a mut T, &'a mut NList<T, L>)`
    pub const fn split_head_poly<'a, P>(
        this: P
    ) -> (HktApply<'a, P::Hkt, T>, HktApply<'a, P::Hkt, NList<T, SubOneSat<Nat<NextBits, B>>>>)
    where
        P: Receiver<'a, NList<T, SubOneSat<Nat<NextBits, B>>>>,
    {
        typewit::type_fn! {
            struct SplitHeadFn<'a, T, NextBits: Int, B: Bit>;

            impl<P: Receiver<'a, NList<T, Nat<NextBits, B>>>> P 
            => (
                HktApply<'a, P::Hkt, T>, 
                HktApply<'a, P::Hkt, NList<T, Nat<NextBits, B>>>
            ) where
                T: 'a
        }

        let func = SplitHeadFn::<'a, T, NextBits, B>::NEW;

        match ReceiverWit::<'a, P, NList<T, SubOneSat<Nat<NextBits, B>>>>::NEW {
            ReceiverWit::Value(te) => {
                let ret = te.to_right(this).into_split_head();
                te.map(func).to_left(ret)
            },
            ReceiverWit::Ref(te) => {
                let ret = te.to_right(this).split_head();
                te.map(func).to_left(ret)
            },
            ReceiverWit::RefMut(te) => {
                let ret = te.to_right(this).split_head_mut();
                te.map(func).to_left(ret)
            },
        }
    }
}


// TODO: uncomment
// mod indexing;

impl<T, L: Int> NList<T, L> {
    /// Returns the length of the list
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{NList, nlist};
    ///
    /// assert_eq!(NList::nil::<u32>().len(), 0);
    ///
    /// assert_eq!(nlist![5].len(), 1);
    ///
    /// assert_eq!(nlist![8, 5].len(), 2);
    ///
    /// ```
    pub const fn len(&self) -> usize {
        L::USIZE
    }

    /// Returns whether the list is empty
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use nlist::{NList, nlist};
    ///
    /// assert!(NList::nil::<u32>().is_empty());
    ///
    /// assert!(!nlist![5].is_empty());
    ///
    /// assert!(!nlist![8, 5].is_empty());
    ///
    /// ```
    pub const fn is_empty(&self) -> bool {
        L::USIZE == 0
    }

}

// TODO: uncomment
// mod flatten;
// mod splitting;
// mod iteratorlike;

impl<T, L: Int> NList<T, L> {
    /// Converts this list into a `Vec`
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list = nlist![3, 5, 8];
    ///
    /// assert_eq!(list.into_vec(), vec![3, 5, 8]);
    ///
    /// ```
    #[cfg(feature = "alloc")]
    pub fn into_vec(self) -> Vec<T> {
        fn to_vec_inner<T, L: Int>(out: &mut Vec<T>, nlist: NList<T, L>) {
            if let IntWit::Nat(len_te) = L::INT_WIT {
                let Cons { elem, next, .. } = nlist.coerce_len(len_te).node;

                out.push(elem);
                to_vec_inner(out, next);
            }
        }

        let mut out = Vec::new();
        to_vec_inner(&mut out, self);
        out
    }

    /// Converts this list into an array
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist, Int};
    ///
    ///
    /// let list: NList<&str, Int!(4)> = nlist!["hello", "world", "foo", "bar"];
    ///
    /// let array: [&str; 4] = list.into_array();
    ///
    /// assert_eq!(array, ["hello", "world", "foo", "bar"])
    ///
    /// ```
    ///
    pub const fn into_array<const N: usize>(self) -> [T; N]
    where
        L: IntoUsize<Usize = Usize<N>>,
    {
        let mut array = [const { None::<ManuallyDrop<T>> }; N];

        const fn inner<T, L, const N: usize>(
            list: NList<T, L>, 
            index: usize, 
            out: &mut [Option<ManuallyDrop<T>>; N]
        ) where
            L: Int,
        {
            match L::INT_WIT {
                IntWit::Zeros(len_te) => {
                    // works around "destructor cannot be evaluated at compile-time" error
                    _ = list.coerce_len(len_te);
                }
                IntWit::Nat(len_te) => {
                    destructure!{(elem, next) = list.coerce_len(len_te).into_split_head()}
                    
                    out[index] = Some(ManuallyDrop::new(elem));

                    inner(next, index + 1, out)
                }
            }
        }

        inner(self, 0, &mut array);

        konst::array::map_!(array, |x| {
            ManuallyDrop::into_inner(
                x.expect("for_each should have filled all elements")
            )                
        })
    }

    /// Makes a bytewise [`Copy`] of the list, element by element.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist, Int};
    ///
    /// let list: NList<u8, Int!(3)> = nlist![3, 5, 8];
    /// let copy: NList<u8, Int!(3)> = list.copy();
    ///
    /// assert_eq!(list, nlist![3, 5, 8]);
    /// assert_eq!(copy, nlist![3, 5, 8]);
    ///
    /// ```
    /// 
    /// [`Copy`]: core::marker::Copy
    pub const fn copy(&self) -> Self
    where
        T: Copy,
    {
        match L::INT_WIT {
            IntWit::Zeros(len_te) => NList::nil_sub(len_te),

            IntWit::Nat(len_te) => {
                let Cons { elem, next, .. } = &self.as_coerce_len(len_te).node;
                NList::cons_sub(*elem, next.copy(), len_te)
            }
        }
    }

    /// Helper method for dropping NList in a const context where `T: Copy`.
    /// 
    pub const fn assert_copy_drop(self)
    where
        T: Copy
    {
        core::mem::forget(self)
    }

    /// Helper method for conditionally consuming `NList` of Copy elements in a 
    /// const context.
    /// 
    pub const fn assert_copy(self) -> ManuallyDrop<Self>
    where
        T: Copy
    {
        core::mem::ManuallyDrop::new(self)
    }

    /// Gets a list of references to each element of this list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist, Int};
    ///
    /// let list: NList<u8, Int!(3)> = nlist![3, 5, 8];
    ///
    /// let refs: NList<&u8, Int!(3)> = list.each_ref();
    /// assert_eq!(refs, nlist![&3, &5, &8]);
    ///
    /// ```
    /// 
    /// [`Copy`]: core::marker::Copy
    pub const fn each_ref(&self) -> NList<&T, L> {
        match L::INT_WIT {
            IntWit::Zeros(len_te) => NList::nil_sub(len_te),

            IntWit::Nat(len_te) => {
                let Cons { elem, next, .. } = &self.as_coerce_len(len_te).node;
                NList::cons_sub(elem, next.each_ref(), len_te)
            }
        }
    }

    /// Gets a list of mutable references to each element of this list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist, Int};
    ///
    /// let mut list: NList<u8, Int!(3)> = nlist![3, 5, 8];
    ///
    /// let muts: NList<&mut u8, Int!(3)> = list.each_mut();
    /// assert_eq!(muts, nlist![&3, &5, &8]);
    ///
    /// ```
    /// 
    /// [`Copy`]: core::marker::Copy
    pub const fn each_mut(&mut self) -> NList<&mut T, L> {
        match L::INT_WIT {
            IntWit::Zeros(len_te) => NList::nil_sub(len_te),

            IntWit::Nat(len_te) => {
                let Cons { elem, next, .. } = &mut self.as_mut_coerce_len(len_te).node;
                NList::cons_sub(elem, next.each_mut(), len_te)
            }
        }
    }

}


impl<T, L: Int> NList<T, L> {
    /// Alternate way to get [`IntWit`] for `L`
    pub const fn len_proof(&self) -> IntWit<L> {
        <L as Int>::INT_WIT
    }

    /// Given a proof that `L == L2`, coerces `NList<T, L>` to `NList<T, L2>`
    /// 
    /// # Example
    /// 
    /// Emulating specialization over list length,
    /// the function behaves differently for length 3 and 5 than with other lengths.
    /// 
    /// ```rust
    /// use nlist::{NList, Int, Int, nlist, int};
    /// use nlist::typewit::TypeCmp;
    /// 
    /// pub const fn make_list<L: Int>() -> NList<usize, L> {
    ///     if let TypeCmp::Eq(len3_te) = int::eq::<Int!(3), L>() {
    ///         // len3_te is a proof that `Int!(3) == L`
    ///         // which allows us to coerce `NList<T, Int!(3)>` to `NList<T, L>`
    ///         nlist![3, 5, 8].coerce_len(len3_te)
    ///     } else if let TypeCmp::Eq(len5_te) = int::eq::<Int!(5), L>() {
    ///         // len5_te is a proof that `Int!(5) == L`
    ///         nlist![3, 5, 8, 13, 21].coerce_len(len5_te)
    ///     } else {
    ///         NList::repeat_copy(L::USIZE)
    ///     }
    /// }
    /// 
    /// assert_eq!(make_list::<Int!(0)>(), nlist![]);
    /// assert_eq!(make_list::<Int!(1)>(), nlist![1]);
    /// assert_eq!(make_list::<Int!(2)>(), nlist![2, 2]);
    /// assert_eq!(make_list::<Int!(3)>(), nlist![3, 5, 8]);
    /// assert_eq!(make_list::<Int!(4)>(), nlist![4, 4, 4, 4]);
    /// assert_eq!(make_list::<Int!(5)>(), nlist![3, 5, 8, 13, 21]);
    /// assert_eq!(make_list::<Int!(6)>(), nlist![6, 6, 6, 6, 6, 6]);
    /// 
    /// 
    /// ```
    pub const fn coerce_len<L2: Int>(self, len_te: TypeEq<L, L2>) -> NList<T, L2> {
        len_te.map(NListFn::NEW).to_right(self)
    }

    /// Given a proof that `L == L2`, coerces `&NList<T, L>` to `&NList<T, L2>`
    /// 
    pub const fn as_coerce_len<L2: Int>(&self, len_te: TypeEq<L, L2>) -> &NList<T, L2> {
        len_te.map(NListFn::NEW).in_ref().to_right(self)
    }

    /// Given a proof that `L == L2`, coerces `&mut NList<T, L>` to `&mut NList<T, L2>`
    pub const fn as_mut_coerce_len<L2>(&mut self, len_te: TypeEq<L, L2>) -> &mut NList<T, L2>
    where
        L2: Int,
    {
        len_te.map(NListFn::NEW).in_mut().to_right(self)
    }

    /// Generic version of `coerce_len` that can take 
    /// `NList` by value/reference/mutable reference,
    /// and returns the corresponding value/reference/mutable of an `NList` with that length.
    /// 
    /// `P` and the return type can only be these:
    /// - If `P == NList<T, L>`: the return type is `NList<T, L2>`
    /// - If `P == &'a NList<T, L>`: the return type is `&'a NList<T, L2>`
    /// - If `P == &'a mut NList<T, L>`: the return type is `&'a mut NList<T, L2>`
    /// 
    /// 
    pub const fn coerce_len_poly<'a, P, L2>(
        this: P, 
        len_te: TypeEq<L, L2>
    ) -> HktApply<'a, P::Hkt, NList<T, L2>>
    where
        P: Receiver<'a, NList<T, L>>,
        L2: Int,
    {
        let func = MapReceiverFn::<NList<T, L>, NList<T, L2>>::NEW;

        match ReceiverWit::<'a, P, NList<T, L>>::NEW {
            ReceiverWit::Value(te) => {
                let ret = te.to_right(this).coerce_len(len_te);
                te.map(func).to_left(ret)
            },
            ReceiverWit::Ref(te) => {
                let ret = te.to_right(this).as_coerce_len(len_te);
                te.map(func).to_left(ret)
            },
            ReceiverWit::RefMut(te) => {
                let ret = te.to_right(this).as_mut_coerce_len(len_te);
                te.map(func).to_left(ret)
            },
        }
    }
}


impl<T, L, const N: usize> From<NList<T, L>> for [T; N]
where
    L: IntoUsize<Usize = Usize<N>>,
{
    fn from(list: NList<T, L>) -> [T; N] {
        list.into_array()
    }
}

impl<T, const N: usize> From<[T; N]> for NList<T, FromUsize<N>>
where
    Usize<N>: IntoInt,
{
    fn from(list: [T; N]) -> NList<T, FromUsize<N>> {
        NList::from_array(list)
    }
}

const fn index_list<L: Int>() -> NList<usize, L> {
    const fn inner<OuterL, L>() -> NList<usize, L> 
    where
        OuterL: Int,
        L: Int,
    {
        match L::INT_WIT {
            IntWit::Zeros(len_te) => NList::nil_sub(len_te),

            IntWit::Nat(len_te) => NList::cons_sub(
                const { OuterL::USIZE - L::USIZE }, 
                inner::<OuterL, _>(), 
                len_te
            ),
        }
    }

    inner::<L, L>()
}
