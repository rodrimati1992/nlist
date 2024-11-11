use const_panic::concat_panic;
use typewit::{TypeCmp, TypeEq};

use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use core::fmt::{self, Debug};
use core::marker::PhantomData;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::peano::{self, PeanoInt, PeanoWit, PlusOne, SubOneSat, Zero};

////////////////////////////////////////////////////////////////////////////////

/// The type of the head node in `NList<T, L>`.
///
/// If `L` is:
/// - `Zero`: this evaluates to `Nil<T, L>`
/// - `PlusOne<_>`: this evaluates to `Cons<T, L>`
///
pub type Node<T, L> = <L as PeanoInt>::IfZero<Nil<T, L>, Cons<T, L>>;

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

////////////////////////////////////////////////////////////////////////////////

/// Inline-allocated list of `T`
/// which statically tracks its length using the `L` type parameter.
pub struct NList<T, L: PeanoInt> {
    /// The first node in the list
    pub node: Node<T, L>,
}

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

impl<T> NList<T, Zero> {
    /// Constructs an empty `NList`
    pub const fn nil() -> NList<T, Zero> {
        NList {
            node: Nil(TypeEq::NEW, PhantomData),
        }
    }
}

impl<T, L: PeanoInt> NList<T, PlusOne<L>> {
    /// Constructs an `NList` with the head element, and the rest of the list.
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
    pub fn from_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        const { index_list() }.map(f)
    }

    /// Alternate constructor for [`NList::nil`],
    /// for constructing an empty `NList` in a generic context.
    pub const fn nil_sub(len_te: TypeEq<L, Zero>) -> Self {
        NList::nil().coerce_length(len_te.flip())
    }

    /// Alternate constructor for [`NList::cons`],
    /// for constructing a `NList` out of the tail of another `NList`
    /// in a generic context.
    pub const fn cons_sub<L2: PeanoInt>(
        val: T,
        next: NList<T, L2>,
        len_te: TypeEq<L, PlusOne<L2>>,
    ) -> Self {
        NList::cons(val, next).coerce_length(len_te.flip())
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
                NodeWit::Cons { node_te: l_node_te, .. }, 
                NodeWit::Cons { node_te: r_node_te, .. }
            ) => {
                let lhs = l_node_te.in_ref().to_right(&self.node);
                let rhs = r_node_te.in_ref().to_right(&rhs.node);

                match lhs.elem.partial_cmp(&rhs.elem) {
                    Some(Ordering::Equal) => lhs.next.partial_cmp(&rhs.next),
                    other => other
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
    pub fn cmp<L2>(&self, rhs: &NList<T, L2>) -> Ordering 
    where
        T: Ord,
        L2: PeanoInt,
    {
        match (NList::<T, L>::WIT, NList::<T, L2>::WIT) {
            (NodeWit::Nil { .. }, NodeWit::Nil { .. }) => Ordering::Equal,
            (NodeWit::Nil { .. }, NodeWit::Cons { .. }) => Ordering::Less,
            (NodeWit::Cons { .. }, NodeWit::Nil { .. }) => Ordering::Greater,
            (
                NodeWit::Cons { node_te: l_node_te, .. }, 
                NodeWit::Cons { node_te: r_node_te, .. }
            ) => {
                let lhs = l_node_te.in_ref().to_right(&self.node);
                let rhs = r_node_te.in_ref().to_right(&rhs.node);

                match lhs.elem.cmp(&rhs.elem) {
                    Ordering::Equal => lhs.next.cmp(&rhs.next),
                    other => other
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
        self.each_ref()
            .zip(rhs.each_ref())
            .fold(Ordering::Equal, |accum, (l, r)| match accum {
                Ordering::Equal => l.cmp(r),
                accum => accum,
            })
    }
}

////////////////////////////////////////////

impl<T, L: PeanoInt> NList<T, PlusOne<L>> {
    /// Returns a reference to the first element of the list
    pub const fn head(&self) -> &T {
        &self.node.elem
    }

    /// Returns a mutable reference ot the first element of the list
    pub fn head_mut(&mut self) -> &mut T {
        &mut self.node.elem
    }

    /// Returns the first element of the list by value
    pub fn into_head(self) -> T {
        self.node.elem
    }

    /// Returns a reference to the remainder of the list
    pub const fn tail(&self) -> &NList<T, L> {
        &self.node.next
    }

    /// Returns a mutable reference to the remainder of the list
    pub fn tail_mut(&mut self) -> &mut NList<T, L> {
        &mut self.node.next
    }

    /// Returns the remainder of the list by value
    pub fn into_tail(self) -> NList<T, L> {
        self.node.next
    }

    /// Returns a pair of references to the first element and the remainder of the list
    pub const fn split_head(&self) -> (&T, &NList<T, L>) {
        let Cons { elem, next, .. } = &self.node;

        (elem, next)
    }

    /// Returns a pair of mutable references to the first element and the remainder of the list
    pub fn split_head_mut(&mut self) -> (&mut T, &mut NList<T, L>) {
        let Cons { elem, next, .. } = &mut self.node;

        (elem, next)
    }

    /// Returns a by-value pair of first element and the remainder of the list
    pub fn into_split_head(self) -> (T, NList<T, L>) {
        (self.node.elem, self.node.next)
    }
}

impl<T, L: PeanoInt> NList<T, L> {
    /// Returns a reference to the element at the `index` index.
    ///
    /// Returns `None` if the index is out of bounds.
    pub const fn get(&self, index: usize) -> Option<&T> {
        match Self::WIT {
            NodeWit::Nil { .. } => None,
            NodeWit::Cons { node_te, .. } => {
                let Cons { elem, next, .. } = node_te.in_ref().to_right(&self.node);

                if let Some(sub1) = index.checked_sub(1) {
                    next.get(sub1)
                } else {
                    Some(elem)
                }
            }
        }
    }

    /// Returns a mutable reference to the element at the `index` index.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match Self::WIT {
            NodeWit::Nil { .. } => None,
            NodeWit::Cons { node_te, .. } => {
                let Cons { elem, next, .. } = node_te.in_mut().to_right(&mut self.node);

                if let Some(sub1) = index.checked_sub(1) {
                    next.get_mut(sub1)
                } else {
                    Some(elem)
                }
            }
        }
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
                let (elem, tail) = self.coerce_length(len_te).into_split_head();

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
    pub const fn coerce_length<L2: PeanoInt>(self, len_te: TypeEq<L, L2>) -> NList<T, L2> {
        len_te.map(NListFn::NEW).to_right(self)
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
        /// Proof that `Node<T, L> == Nil<T, PlusOne<L::SubOneSat>>`
        node_te: TypeEq<Node<T, L>, Cons<T, PlusOne<L::SubOneSat>>>,
        /// Proof that `L == PlusOne<L::SubOneSat>`
        len_te: TypeEq<L, PlusOne<L::SubOneSat>>,
    },
}

const fn index_list<L: PeanoInt>() -> NList<usize, L> {
    match <NList<usize, L>>::WIT {
        NodeWit::Nil { len_te, .. } => NList::nil_sub(len_te),

        NodeWit::Cons { len_te, .. } => NList::cons_sub(L::USIZE, index_list(), len_te),
    }
}
