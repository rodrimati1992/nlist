use const_panic::concat_panic;
use typewit::TypeEq;

use super::{Cons, NList, NodeWit};

#[allow(unused_imports)]
use crate::peano::{self, PeanoInt, PeanoWit, PlusOne, SubOneSat, Zero};

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

    /// Returns a reference to the element at the `I` index.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist, peano};
    ///
    /// let list = nlist![3, 5, 8, 13];
    ///
    /// assert_eq!(list.index::<peano!(0)>(), &3);
    /// assert_eq!(list.index::<peano!(1)>(), &5);
    /// assert_eq!(list.index::<peano!(2)>(), &8);
    /// assert_eq!(list.index::<peano!(3)>(), &13);
    ///
    ///
    /// ```
    pub const fn index<I>(&self) -> &T
    where
        I: PeanoInt,
        L: PeanoInt<Max<PlusOne<I>> = L>,
    {
        const fn inner<T, L, At>(list: &NList<T, L>, at: At) -> &T
        where
            L: PeanoInt,
            At: PeanoInt,
        {
            match IndexState::<L, At>::NEW {
                IndexState::Iterating { l_te, at_te } => {
                    inner(list.as_coerce_len(l_te).tail(), at_te.to_right(at).sub_one)
                }
                IndexState::Finished { l_te } => list.as_coerce_len(l_te).head(),
            }
        }

        inner(self, I::NEW)
    }

    /// Returns a mutable reference to the element at the `I` index.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist, peano};
    ///
    /// let mut list = nlist![3, 5, 8, 13];
    ///
    /// assert_eq!(list.index_mut::<peano!(0)>(), &mut 3);
    /// assert_eq!(list.index_mut::<peano!(1)>(), &mut 5);
    /// assert_eq!(list.index_mut::<peano!(2)>(), &mut 8);
    /// assert_eq!(list.index_mut::<peano!(3)>(), &mut 13);
    ///
    ///
    /// ```
    pub fn index_mut<I>(&mut self) -> &mut T
    where
        I: PeanoInt,
        PlusOne<I>: PeanoInt<Max<L> = L>,
    {
        fn inner<T, L, At>(list: &mut NList<T, L>, at: At) -> &mut T
        where
            L: PeanoInt,
            At: PeanoInt,
        {
            match IndexState::<L, At>::NEW {
                IndexState::Iterating { l_te, at_te } => {
                    inner(list.as_mut_coerce_len(l_te).tail_mut(), at_te.to_right(at).sub_one)
                }
                IndexState::Finished { l_te } => list.as_mut_coerce_len(l_te).head_mut(),
            }
        }

        inner(self, I::NEW)
    }
}

enum IndexState<L, At>
where
    L: PeanoInt,
    At: PeanoInt,
{
    Iterating {
        // The `At::IfZeroPI<L` part is necessary so that, 
        // when this enum is `Self::Finished`,
        // the recursive call to `inner` in the dead `Iterating` branch
        // doesn't cause const panics.
        // If rustc didn't evaluate const code in dead branches, this field would be:
        // `TypeEq<L, PlusOne<L::SubOneSat>>`
        l_te: TypeEq<L, PlusOne<At::IfZeroPI<L, L::SubOneSat>>>,
        at_te: TypeEq<At, PlusOne<At::SubOneSat>>,
    },
    Finished {
        l_te: TypeEq<L, PlusOne<L::SubOneSat>>,
    },
}

typewit::type_fn! {
    struct MapTailFn<L>;

    impl<At> At => PlusOne<At::IfZeroPI<L, L::SubOneSat>>
    where
        At: PeanoInt,
        L: PeanoInt,
}

impl<L, At> IndexState<L, At>
where
    L: PeanoInt,
    At: PeanoInt,
{
    const NEW: Self = match (L::PEANO_WIT, At::PEANO_WIT) {
        (PeanoWit::PlusOne(l_te), PeanoWit::PlusOne(at_te)) => Self::Iterating {
            l_te: l_te.join(at_te.project::<MapTailFn<L>>().flip()),
            at_te,
        },
        (PeanoWit::PlusOne(l_te), PeanoWit::Zero(_)) => IndexState::Finished { l_te },
        _ => concat_panic! {
            "indexing bug: ",
            " L: ", L::USIZE,
            " At: ", At::USIZE,
        },
    };
}