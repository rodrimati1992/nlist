use const_panic::concat_panic;
use typewit::{TypeCmp, TypeEq};

use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use core::fmt::{self, Debug};
use core::marker::PhantomData;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::peano::{
    self, IntoPeano, IntoUsize, PeanoInt, PeanoWit, PlusOne, SubOneSat, Usize, Zero,
};

////////////////////////////////////////////////////////////////////////////////

typewit::type_fn! {
    struct NListFn<T>;

    impl<L: PeanoInt> L => NList<T, L>;
}

typewit::type_fn! {
    struct NodeFn<T>;

    impl<L: PeanoInt> L => Node<T, L>;
}

typewit::type_fn! {
    struct ConsListFn<T>;

    impl<L: PeanoInt> L => NList<T, PlusOne<L>>;
}

typewit::type_fn! {
    struct AddPeanoFn;

    impl<L1: PeanoInt, L2: PeanoInt> (L1, L2) => peano::Add<L1, L2>;
}

typewit::type_fn! {
    struct MulPeanoFn;

    impl<L1: PeanoInt, L2: PeanoInt> (L1, L2) => peano::Mul<L1, L2>;
}

////////////////////////////////////////////////////////////////////////////////

/// Inline-allocated list of `T`
/// which statically tracks its length using the `L` type parameter.
pub struct NList<T, L: PeanoInt> {
    /// The first node in the list
    pub node: Node<T, L>,
}

/// Alias for an `NList` of `NList`s
pub type NList2D<T, LOuter, LInner> = NList<NList<T, LInner>, LOuter>;

/// A node of an empty [`NList`]
pub struct Nil<T, L>(TypeEq<L, Zero>, PhantomData<T>);

/// A node of [`NList`] with one element and the rest of the list.
pub struct Cons<T, L: PeanoInt> {
    /// The element of this node
    pub elem: T,
    /// The rest of the list
    pub next: NList<T, L::SubOneSat>,
    #[allow(dead_code)]
    // assertion that `L == PlusOne<_>`, it doesn't matter if it's used.
    len_te: TypeEq<L, PlusOne<L::SubOneSat>>,
}

/// The type of the head node in `NList<T, L>`.
///
/// If `L` is:
/// - `Zero`: this evaluates to `Nil<T, L>`
/// - `PlusOne<_>`: this evaluates to `Cons<T, L>`
///
pub type Node<T, L> = <L as PeanoInt>::IfZero<Nil<T, L>, Cons<T, L>>;

////////////////////////////////////////////////////////////////////////////////

impl NList<(), Zero> {
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
    pub const fn nil<T>() -> NList<T, Zero> {
        NList {
            node: Nil(TypeEq::NEW, PhantomData),
        }
    }
}

impl<T, L: PeanoInt> NList<T, PlusOne<L>> {
    /// Constructs an `NList` with the head element, and the rest of the list.
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
    pub const fn cons(val: T, next: NList<T, L>) -> Self {
        NList {
            node: Cons {
                elem: val,
                next,
                len_te: TypeEq::NEW,
            },
        }
    }
}

impl<T, L: PeanoInt> NList<T, L> {
    /// Constructs a list by repeating `elem`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist, peano};
    ///
    /// let nlist: NList<_, peano!(4)> = NList::repeat_copy(3);
    ///
    /// assert_eq!(nlist, nlist![3, 3, 3, 3]);
    /// assert_eq!(nlist, nlist![3; 4]);
    ///
    /// ```
    pub const fn repeat_copy(elem: T) -> Self
    where
        T: Copy,
    {
        match Self::WIT {
            NodeWit::Nil { len_te, .. } => NList::nil_sub(len_te),

            NodeWit::Cons { len_te, .. } => NList::cons_sub(elem, NList::repeat_copy(elem), len_te),
        }
    }

    /// Constructs a list by calling `f` with the index of each element.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist, peano};
    ///
    /// let list: NList<_, peano!(4)> = NList::from_fn(|i| i.pow(2));
    ///
    /// assert_eq!(list, nlist![0, 1, 4, 9]);
    /// ```
    pub fn from_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        const { index_list() }.map(f)
    }

    /// Constructs an NList from an array
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist};
    ///
    /// let nlist = NList::from_array([3, 5, 8, 13, 21]);
    ///
    /// assert_eq!(nlist, nlist![3, 5, 8, 13, 21]);
    ///
    /// ```
    pub fn from_array<const N: usize>(array: [T; N]) -> Self
    where
        Usize<N>: IntoPeano<Peano = L>,
    {
        let mut iter = array.into_iter();

        Self::from_fn(|_| {
            iter.next()
                .expect("calling iter.next() `L` times shouldn't panic")
        })
    }

    /// Alternate constructor for [`NList::nil`],
    /// for constructing an empty `NList` in a generic context.
    const fn nil_sub(len_te: TypeEq<L, Zero>) -> Self {
        NList::nil().coerce_len(len_te.flip())
    }

    /// Alternate constructor for [`NList::cons`],
    /// for constructing a `NList` out of the tail of another `NList`
    /// in a generic context.
    const fn cons_sub<L2: PeanoInt>(
        val: T,
        next: NList<T, L2>,
        len_te: TypeEq<L, PlusOne<L2>>,
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
    L: PeanoInt,
{
    fn clone(&self) -> Self {
        self.each_ref().map(T::clone)
    }
}

impl<T, L> Debug for NList<T, L>
where
    T: Debug,
    L: PeanoInt,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = fmt.debug_list();

        self.each_ref().for_each(|_, elem| {
            _ = fmt.entry(elem);
        });

        fmt.finish()
    }
}

impl<T, U, L, L2> PartialEq<NList<U, L2>> for NList<T, L>
where
    T: PartialEq<U>,
    L: PeanoInt,
    L2: PeanoInt,
{
    fn eq(&self, rhs: &NList<U, L2>) -> bool {
        let TypeCmp::Eq(te_len) = peano::cmp_peanos(L::NEW, L2::NEW) else {
            return false;
        };

        let rhs = te_len.map(NListFn::NEW).in_ref().to_left(rhs);

        self.each_ref().zip(rhs.each_ref()).all(|(l, r)| l == r)
    }
}

impl<T, L> NList<T, L>
where
    L: PeanoInt,
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
        L2: PeanoInt,
    {
        self == rhs
    }
}

impl<T, L> Eq for NList<T, L>
where
    T: Eq,
    L: PeanoInt,
{
}

impl<T, U, L, L2> PartialOrd<NList<U, L2>> for NList<T, L>
where
    T: PartialOrd<U>,
    L: PeanoInt,
    L2: PeanoInt,
{
    fn partial_cmp(&self, rhs: &NList<U, L2>) -> Option<Ordering> {
        match (NList::<T, L>::WIT, NList::<U, L2>::WIT) {
            (NodeWit::Nil { .. }, NodeWit::Nil { .. }) => Some(Ordering::Equal),
            (NodeWit::Nil { .. }, NodeWit::Cons { .. }) => Some(Ordering::Less),
            (NodeWit::Cons { .. }, NodeWit::Nil { .. }) => Some(Ordering::Greater),
            (
                NodeWit::Cons {
                    node_te: l_node_te, ..
                },
                NodeWit::Cons {
                    node_te: r_node_te, ..
                },
            ) => {
                let lhs = l_node_te.in_ref().to_right(&self.node);
                let rhs = r_node_te.in_ref().to_right(&rhs.node);

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
    L: PeanoInt,
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
        L2: PeanoInt,
    {
        self.inherent_cmp(rhs)
    }

    fn inherent_cmp<L2>(&self, rhs: &NList<T, L2>) -> Ordering
    where
        T: Ord,
        L2: PeanoInt,
    {
        match (NList::<T, L>::WIT, NList::<T, L2>::WIT) {
            (NodeWit::Nil { .. }, NodeWit::Nil { .. }) => Ordering::Equal,
            (NodeWit::Nil { .. }, NodeWit::Cons { .. }) => Ordering::Less,
            (NodeWit::Cons { .. }, NodeWit::Nil { .. }) => Ordering::Greater,
            (
                NodeWit::Cons {
                    node_te: l_node_te, ..
                },
                NodeWit::Cons {
                    node_te: r_node_te, ..
                },
            ) => {
                let lhs = l_node_te.in_ref().to_right(&self.node);
                let rhs = r_node_te.in_ref().to_right(&rhs.node);

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
    L: PeanoInt,
{
    fn cmp(&self, rhs: &NList<T, L>) -> Ordering {
        self.inherent_cmp(rhs)
    }
}

////////////////////////////////////////////

impl<T, L: PeanoInt> NList<T, PlusOne<L>> {
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
    pub fn head_mut(&mut self) -> &mut T {
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
    pub const fn tail(&self) -> &NList<T, L> {
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
    pub fn tail_mut(&mut self) -> &mut NList<T, L> {
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
    pub fn into_tail(self) -> NList<T, L> {
        self.node.next
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
    pub const fn split_head(&self) -> (&T, &NList<T, L>) {
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
    pub fn split_head_mut(&mut self) -> (&mut T, &mut NList<T, L>) {
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
    pub fn into_split_head(self) -> (T, NList<T, L>) {
        (self.node.elem, self.node.next)
    }
}

mod indexing;

impl<T, L: PeanoInt> NList<T, L> {
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

    /// Finds the first element for which `predicate(&element)` returns true.
    pub fn find<F>(self, mut f: F) -> Option<T>
    where
        F: FnMut(&T) -> bool,
    {
        self._find_helper(move |_, x| f(&x).then_some(x))
    }

    /// Iterates the list elements,
    /// returning the first non-None return value of `mapper(element)`.
    pub fn find_map<F, R>(self, mut mapper: F) -> Option<R>
    where
        F: FnMut(T) -> Option<R>,
    {
        self._find_helper(move |_, x| mapper(x))
    }

    /// Finds the first index for which `predicate` returns true.
    pub fn position<F, R>(self, mut predicate: F) -> Option<usize>
    where
        F: FnMut(T) -> bool,
    {
        self._find_helper(move |i, x| predicate(x).then_some(i))
    }

    fn _find_helper<F, R>(self, f: F) -> Option<R>
    where
        F: FnMut(usize, T) -> Option<R>,
    {
        fn inner<T, L, F, R>(list: NList<T, L>, index: usize, mut f: F) -> Option<R>
        where
            L: PeanoInt,
            F: FnMut(usize, T) -> Option<R>,
        {
            match <NList<T, L>>::WIT {
                NodeWit::Nil { .. } => None,
                NodeWit::Cons { node_te, .. } => {
                    let Cons { elem, next, .. } = node_te.to_right(list.node);

                    if let x @ Some(_) = f(index, elem) {
                        x
                    } else {
                        inner(next, index + 1, f)
                    }
                }
            }
        }

        inner(self, 0, f)
    }

    ///////
    // r-prefixed methods

    /// Iterates the list elements in reverse,
    /// returns the first element for which `predicate(&element)` returns true.
    pub fn rfind<F>(self, mut f: F) -> Option<T>
    where
        F: FnMut(&T) -> bool,
    {
        self._rfind_helper(move |_, x| f(&x).then_some(x))
    }

    /// Iterates the list elements in reverse,
    /// returning the first non-None return value of `mapper(element)`.
    pub fn rfind_map<F, R>(self, mut mapper: F) -> Option<R>
    where
        F: FnMut(T) -> Option<R>,
    {
        self._rfind_helper(move |_, x| mapper(x))
    }

    /// Iterates the list elements in reverse,
    /// Finds the index of the first element for which `predicate` returns true.
    pub fn rposition<F, R>(self, mut predicate: F) -> Option<usize>
    where
        F: FnMut(T) -> bool,
    {
        self._rfind_helper(move |i, x| predicate(x).then_some(i))
    }

    fn _rfind_helper<F, R>(self, mut f: F) -> Option<R>
    where
        F: FnMut(usize, T) -> Option<R>,
    {
        fn inner<T, L, F, R>(list: NList<T, L>, index: usize, f: &mut F) -> Option<R>
        where
            L: PeanoInt,
            F: FnMut(usize, T) -> Option<R>,
        {
            match <NList<T, L>>::WIT {
                NodeWit::Nil { .. } => None,
                NodeWit::Cons { node_te, .. } => {
                    let Cons { elem, next, .. } = node_te.to_right(list.node);

                    if let x @ Some(_) = inner(next, index + 1, f) {
                        x
                    } else {
                        f(index, elem)
                    }
                }
            }
        }

        inner(self, 0, &mut f)
    }
}

mod flatten;
mod splitting;

impl<T, L: PeanoInt> NList<T, L> {
    /// Consumes and returns a reversed version of this list
    pub fn reverse(self) -> NList<T, L> {
        enum ReverseState<T, LI, LA, LR>
        where
            LI: PeanoInt,
            LA: PeanoInt,
            LR: PeanoInt,
        {
            Iterating {
                input_te: TypeEq<NList<T, LI>, NList<T, PlusOne<SubOneSat<LI>>>>,
                output_te: TypeEq<
                    NList<T, PlusOne<LA>>,
                    // computes the minimum of `PlusOne<LA>` and `LR`
                    // so that `inner` doesn't monomorphize to values of
                    // `LA` larger than `LR`
                    NList<T, peano::Min<PlusOne<LA>, LR>>,
                >,
            },
            Finished {
                output_te: TypeEq<NList<T, LA>, NList<T, LR>>,
            },
        }

        impl<T, LI, LA, LR> ReverseState<T, LI, LA, LR>
        where
            LI: PeanoInt,
            LA: PeanoInt,
            LR: PeanoInt,
        {
            const NEW: Self = match (LI::PEANO_WIT, peano::cmp_peanos(LA::NEW, LR::NEW)) {
                (PeanoWit::PlusOne(input_te), TypeCmp::Ne(_)) => {
                    let TypeCmp::Eq(output_te) =
                        peano::cmp_peanos(PlusOne::<LA>::NEW, peano::Min::<PlusOne<LA>, LR>::NEW)
                    else {
                        concat_panic! {"somehow, LA > LR: ", LA::USIZE, " > ", LR::USIZE}
                    };

                    ReverseState::Iterating {
                        input_te: input_te.map(NListFn::NEW),
                        output_te: output_te.map(NListFn::NEW),
                    }
                }
                (PeanoWit::Zero(_), TypeCmp::Eq(output_te)) => ReverseState::Finished {
                    output_te: output_te.map(NListFn::NEW),
                },
                _ => concat_panic! {"somehow, ", LI::USIZE, " + ", LA::USIZE, "!=", LR::USIZE},
            };
        }

        fn inner<T, LI, LA, LR>(input: NList<T, LI>, output: NList<T, LA>) -> NList<T, LR>
        where
            LI: PeanoInt,
            LA: PeanoInt,
            LR: PeanoInt,
        {
            match ReverseState::<T, LI, LA, LR>::NEW {
                ReverseState::Iterating {
                    input_te,
                    output_te,
                } => {
                    let (elem, tail) = input_te.to_right(input).into_split_head();

                    inner(tail, output_te.to_right(NList::cons(elem, output)))
                }
                ReverseState::Finished { output_te, .. } => output_te.to_right(output),
            }
        }

        match Self::WIT {
            NodeWit::Nil { .. } => self,
            NodeWit::Cons { len_te, .. } => {
                let (elem, tail) = self.coerce_len(len_te).into_split_head();

                inner(tail, NList::cons(elem, NList::nil()))
            }
        }
    }

    /// Concatenates this list with another one
    pub fn concat<L2>(self, other: NList<T, L2>) -> NList<T, peano::Add<L, L2>>
    where
        L2: PeanoInt,
    {
        fn inner<T, LA, LB>(lhs: NList<T, LA>, rhs: NList<T, LB>) -> NList<T, peano::Add<LA, LB>>
        where
            LA: PeanoInt,
            LB: PeanoInt,
        {
            match <NList<T, LA>>::WIT {
                NodeWit::Nil { len_te, .. } => len_te
                    .zip(TypeEq::new::<LB>())
                    .map(AddPeanoFn::NEW)
                    .map(NListFn::NEW)
                    .to_left(rhs),
                NodeWit::Cons { node_te, len_te } => {
                    let Cons { elem, next, .. } = node_te.to_right(lhs.node);

                    let len_te = len_te.zip(TypeEq::new::<LB>()).map(AddPeanoFn::NEW);

                    NList::cons_sub(elem, inner(next, rhs), len_te)
                }
            }
        }

        inner(self, other)
    }

    /// Zips this list with another one of the same length
    pub fn zip<U>(self, other: NList<U, L>) -> NList<(T, U), L> {
        fn inner<T, U, L>(lhs: NList<T, L>, rhs: NList<U, L>) -> NList<(T, U), L>
        where
            L: PeanoInt,
        {
            match <NList<T, L>>::WIT {
                NodeWit::Nil { len_te, .. } => NList::nil_sub(len_te),
                NodeWit::Cons { len_te, .. } => {
                    let lhs = len_te.map(NodeFn::NEW).to_right(lhs.node);
                    let rhs = len_te.map(NodeFn::NEW).to_right(rhs.node);

                    NList::cons_sub((lhs.elem, rhs.elem), inner(lhs.next, rhs.next), len_te)
                }
            }
        }

        inner(self, other)
    }

    /// Converts this list into a `Vec`
    #[cfg(feature = "alloc")]
    pub fn into_vec(self) -> Vec<T> {
        fn to_vec_inner<T, L: PeanoInt>(out: &mut Vec<T>, nlist: NList<T, L>) {
            if let NodeWit::Cons { node_te, .. } = <NList<T, L>>::WIT {
                let Cons { elem, next, .. } = node_te.to_right(nlist.node);

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
    /// use nlist::{NList, nlist, peano};
    ///
    ///
    /// let list: NList<&str, peano!(4)> = nlist!["hello", "world", "foo", "bar"];
    ///
    /// let array: [&str; 4] = list.into_array();
    ///
    /// assert_eq!(array, ["hello", "world", "foo", "bar"])
    ///
    /// ```
    ///
    pub fn into_array<const N: usize>(self) -> [T; N]
    where
        L: IntoUsize<Usize = Usize<N>>,
    {
        let mut array = [const { None }; N];

        self.for_each(|i, v| array[i] = Some(v));

        array.map(|o| o.expect("for_each should have filled all elements"))
    }

    /// Makes a bytewise copy of the list element by element.
    pub const fn copy(&self) -> Self
    where
        T: Copy,
    {
        match Self::WIT {
            NodeWit::Nil { len_te, .. } => NList::nil_sub(len_te),

            NodeWit::Cons { node_te, len_te } => {
                let Cons { elem, next, .. } = node_te.in_ref().to_right(&self.node);
                NList::cons_sub(*elem, next.copy(), len_te)
            }
        }
    }

    /// Gets a list of references to each element of this list.
    pub const fn each_ref(&self) -> NList<&T, L> {
        match Self::WIT {
            NodeWit::Nil { len_te, .. } => NList::nil_sub(len_te),

            NodeWit::Cons { node_te, len_te } => {
                let Cons { elem, next, .. } = node_te.in_ref().to_right(&self.node);
                NList::cons_sub(elem, next.each_ref(), len_te)
            }
        }
    }

    /// Gets a list of mutable references to each element of this list.
    pub fn each_mut(&mut self) -> NList<&mut T, L> {
        match Self::WIT {
            NodeWit::Nil { len_te, .. } => NList::nil_sub(len_te),

            NodeWit::Cons { node_te, len_te } => {
                let Cons { elem, next, .. } = node_te.in_mut().to_right(&mut self.node);
                NList::cons_sub(elem, next.each_mut(), len_te)
            }
        }
    }

    /// Maps the elements of this list.
    pub fn map<F, R>(self, mut f: F) -> NList<R, L>
    where
        F: FnMut(T) -> R,
    {
        match Self::WIT {
            NodeWit::Nil { len_te, .. } => NList::nil_sub(len_te),

            NodeWit::Cons { node_te, len_te } => {
                let Cons { elem, next, .. } = node_te.to_right(self.node);
                NList::cons_sub(f(elem), next.map(f), len_te)
            }
        }
    }

    /// Loops over the elements in the list, along with their index.
    pub fn for_each<F>(self, f: F)
    where
        F: FnMut(usize, T),
    {
        fn inner<T, L, F>(list: NList<T, L>, index: usize, mut func: F)
        where
            L: PeanoInt,
            F: FnMut(usize, T),
        {
            if let NodeWit::Cons { node_te, .. } = <NList<T, L>>::WIT {
                let Cons { elem, next, .. } = node_te.to_right(list.node);
                func(index, elem);
                inner(next, index + 1, func)
            }
        }

        inner(self, 0, f)
    }

    /// Returns whether `predicate(elem)` returns true for any element.
    pub fn any<F>(self, mut predicate: F) -> bool
    where
        F: FnMut(T) -> bool,
    {
        !self.all(|x| !predicate(x))
    }

    /// Returns whether `predicate(&elem)` returns true for all elements.
    pub fn all<F>(self, predicate: F) -> bool
    where
        F: FnMut(T) -> bool,
    {
        fn inner<T, L, F>(list: NList<T, L>, mut func: F) -> bool
        where
            L: PeanoInt,
            F: FnMut(T) -> bool,
        {
            match <NList<T, L>>::WIT {
                NodeWit::Nil { .. } => true,
                NodeWit::Cons { node_te, .. } => {
                    let Cons { elem, next, .. } = node_te.to_right(list.node);
                    if func(elem) {
                        inner(next, func)
                    } else {
                        false
                    }
                }
            }
        }

        inner(self, predicate)
    }

    /// Folds over this list.
    pub fn fold<F, A>(self, initial_value: A, mut f: F) -> A
    where
        F: FnMut(A, T) -> A,
    {
        match Self::WIT {
            NodeWit::Nil { .. } => initial_value,
            NodeWit::Cons { node_te, .. } => {
                let Cons { elem, next, .. } = node_te.to_right(self.node);
                next.fold(f(initial_value, elem), f)
            }
        }
    }
}

impl<T, L: PeanoInt> NList<T, PlusOne<L>> {
    /// Equivalent to [`fold`](NList::fold), where the head element is passed as the `initial_value`.
    pub fn reduce<F>(self, f: F) -> T
    where
        F: FnMut(T, T) -> T,
    {
        self.node.next.fold(self.node.elem, f)
    }
}

impl<T, L: PeanoInt> NList<T, L> {
    /// Given a proof that `L == L2`, coerces `NList<T, L>` to `NList<T, L2>`
    pub const fn coerce_len<L2: PeanoInt>(self, len_te: TypeEq<L, L2>) -> NList<T, L2> {
        len_te.map(NListFn::NEW).to_right(self)
    }

    /// Given a proof that `L == L2`, coerces `&NList<T, L>` to `&NList<T, L2>`
    pub const fn as_coerce_len<L2: PeanoInt>(&self, len_te: TypeEq<L, L2>) -> &NList<T, L2> {
        len_te.map(NListFn::NEW).in_ref().to_right(self)
    }

    /// Given a proof that `L == L2`, coerces `&mut NList<T, L>` to `&mut NList<T, L2>`
    pub fn as_mut_coerce_len<L2>(&mut self, len_te: TypeEq<L, L2>) -> &mut NList<T, L2>
    where
        L2: PeanoInt,
    {
        len_te.map(NListFn::NEW).in_mut().to_right(self)
    }
}

impl<T, L: PeanoInt> NList<T, L> {
    /// Type witness for the type of the `node` field in `NList<T, L>`,
    /// and the `L` parameter itself
    pub const WIT: NodeWit<T, L> = match L::PEANO_WIT {
        PeanoWit::Zero(len_te) => NodeWit::Nil {
            node_te: len_te.map(NodeFn::NEW),
            len_te,
        },
        PeanoWit::PlusOne(len_te) => NodeWit::Cons {
            node_te: len_te.map(NodeFn::NEW),
            len_te,
        },
    };
}

impl<T, L, const N: usize> From<NList<T, L>> for [T; N]
where
    L: IntoUsize<Usize = Usize<N>>,
{
    fn from(list: NList<T, L>) -> [T; N] {
        list.into_array()
    }
}

impl<T, L, const N: usize> From<[T; N]> for NList<T, L>
where
    L: PeanoInt,
    Usize<N>: IntoPeano<Peano = L>,
{
    fn from(list: [T; N]) -> NList<T, L> {
        NList::from_array(list)
    }
}

/// Type witness for the type of the `node` field in [`NList<T, L>`](NList),
/// and the `L` parameter itself
pub enum NodeWit<T, L: PeanoInt> {
    /// Proof that the list is empty
    Nil {
        /// Proof that `Node<T, L> == Nil<T, Zero>`
        node_te: TypeEq<Node<T, L>, Nil<T, Zero>>,
        /// Proof that `L == Zero`
        len_te: TypeEq<L, Zero>,
    },
    /// Proof that the list is nonempty
    Cons {
        /// Proof that `Node<T, L> == Cons<T, PlusOne<L::SubOneSat>>`
        node_te: TypeEq<Node<T, L>, Cons<T, PlusOne<L::SubOneSat>>>,
        /// Proof that `L == PlusOne<L::SubOneSat>`
        len_te: TypeEq<L, PlusOne<L::SubOneSat>>,
    },
}

const fn index_list<L: PeanoInt>() -> NList<usize, L> {
    const fn inner<OuterL, L>() -> NList<usize, L> 
    where
        OuterL: PeanoInt,
        L: PeanoInt,
    {
        match L::PEANO_WIT {
            PeanoWit::Zero(len_te) => NList::nil_sub(len_te),

            PeanoWit::PlusOne(len_te) => NList::cons_sub(
                const { OuterL::USIZE - L::USIZE }, 
                inner::<OuterL, _>(), 
                len_te
            ),
        }
    }

    inner::<L, L>()
}
