use const_panic::concat_panic;
use typewit::{const_marker::Bool, TypeEq};

use super::{Cons, NList};

#[allow(unused_imports)]
use crate::peano::{self, PeanoInt, PeanoWit, PlusOne, SubOneSat, Zero};

use crate::boolean::{IfTruePI, Boolean};

impl<T, L: PeanoInt> NList<T, L> {
    /// Returns a reference to the element at the `index` index.
    ///
    /// Returns `None` if the index is greater than `self.len()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist};
    ///
    /// let list = nlist![3, 5, 8, 13];
    ///
    /// assert_eq!(list.get(0), Some(&3));
    /// assert_eq!(list.get(1), Some(&5));
    /// assert_eq!(list.get(2), Some(&8));
    /// assert_eq!(list.get(3), Some(&13));
    /// assert_eq!(list.get(4), None);
    ///
    /// ```
    pub const fn get(&self, index: usize) -> Option<&T> {
        match L::PEANO_WIT {
            PeanoWit::Zero { .. } => None,
            PeanoWit::PlusOne(len_te) => {
                let Cons { elem, next, .. } = &self.as_coerce_len(len_te).node;

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
    /// Returns `None` if the index is greater than `self.len()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist};
    ///
    /// let mut list = nlist![3, 5, 8, 13];
    ///
    /// assert_eq!(list.get_mut(0), Some(&mut 3));
    /// assert_eq!(list.get_mut(1), Some(&mut 5));
    /// assert_eq!(list.get_mut(2), Some(&mut 8));
    /// assert_eq!(list.get_mut(3), Some(&mut 13));
    /// assert_eq!(list.get_mut(4), None);
    ///
    /// ```
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match L::PEANO_WIT {
            PeanoWit::Zero { .. } => None,
            PeanoWit::PlusOne(len_te) => {
                let Cons { elem, next, .. } = &mut self.as_mut_coerce_len(len_te).node;

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
    /// use nlist::{NList, nlist, Peano};
    ///
    /// let list = nlist![3, 5, 8, 13];
    ///
    /// assert_eq!(list.index::<Peano!(0)>(), &3);
    /// assert_eq!(list.index::<Peano!(1)>(), &5);
    /// assert_eq!(list.index::<Peano!(2)>(), &8);
    /// assert_eq!(list.index::<Peano!(3)>(), &13);
    ///
    ///
    /// ```
    pub const fn index<I>(&self) -> &T
    where
        I: PeanoInt<IsLt<L> = Bool<true>>,
    {
        self.index_alt::<I>(TypeEq::NEW)
    }

    /// Alternative version of [`index`] which takes a proof of `I < L` as an argument.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, Peano, nlist, peano};
    /// use nlist::peano::{PeanoInt, proofs};
    /// use nlist::boolean::Bool;
    /// use nlist::typewit::TypeEq;
    /// 
    /// let list = nlist![3, 5, 8, 13];
    /// 
    /// assert_eq!(at(&list, peano!(0)), (&3, &3));
    /// assert_eq!(at(&list, peano!(1)), (&3, &5));
    /// assert_eq!(at(&list, peano!(2)), (&5, &8));
    /// assert_eq!(at(&list, peano!(3)), (&8, &13));
    /// 
    /// const fn at<T, L, At>(list: &NList<T, L>, _at: At) -> (&T, &T) 
    /// where
    ///     L: PeanoInt,
    ///     At: PeanoInt<IsLt<L> = Bool<true>>,
    /// {
    ///     type Dist = Peano!(1);
    ///
    ///     // passes a proof of `At < L` as argument, gets back a proof that `At - Dist < L`
    ///     let sub_lt_l = proofs::compose_sub_lt::<At, _, _>(TypeEq::NEW);
    ///     
    ///     (
    ///         list.index_alt::<peano::SubSat<At, Dist>>(sub_lt_l),
    ///         list.index::<At>(),
    ///     )
    /// }
    /// ```
    /// 
    pub const fn index_alt<I>(&self, i_lt_l_te: TypeEq<I::IsLt<L>, Bool<true>>) -> &T 
    where
        I: PeanoInt
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

        let this = i_lt_l_te.project::<IndexListLenFn<T, I, L>>()
            .in_ref()
            .to_left(self);
        inner(this, I::NEW)
    }

    /// Returns a mutable reference to the element at the `I` index.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist, Peano};
    ///
    /// let mut list = nlist![3, 5, 8, 13];
    ///
    /// assert_eq!(list.index_mut::<Peano!(0)>(), &mut 3);
    /// assert_eq!(list.index_mut::<Peano!(1)>(), &mut 5);
    /// assert_eq!(list.index_mut::<Peano!(2)>(), &mut 8);
    /// assert_eq!(list.index_mut::<Peano!(3)>(), &mut 13);
    ///
    ///
    /// ```
    pub const fn index_mut<I>(&mut self) -> &mut T
    where
        I: PeanoInt<IsLt<L> = Bool<true>>,
    {
        self.index_mut_alt::<I>(TypeEq::NEW)
    }


    /// Alternative version of [`index_mut`] which takes a proof of `I < L` as an argument.
    ///
    pub const fn index_mut_alt<I>(
        &mut self,
        i_lt_l_te: TypeEq<I::IsLt<L>, Bool<true>>,
    ) -> &mut T
    where
        I: PeanoInt,
    {
        const fn inner<T, L, At>(list: &mut NList<T, L>, at: At) -> &mut T
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

        let this = i_lt_l_te.project::<IndexListLenFn<T, I, L>>()
            .in_mut()
            .to_left(self);
        inner(this, I::NEW)
    }
}


typewit::type_fn! {
    struct IndexListLenFn<T, I: PeanoInt, L: PeanoInt>;

    impl<B: Boolean> B => NList<T, IfTruePI<B, L, PlusOne<I>>>
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
