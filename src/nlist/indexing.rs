use const_panic::concat_panic;
use typewit::{const_marker::Bool, TypeEq};

use super::{Cons, NList};

use crate::boolean::{IfTrueI, Boolean};

#[allow(unused_imports)]
use crate::int::{self, Int, IntWit, Nat, SubOneSat, Zeros};

use crate::tordering::TLess;


impl<T, L: Int> NList<T, L> {
    /// Returns a reference to the element at the `index` index.
    ///
    /// Returns `None` if `index >= self.len()`.
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
        match L::INT_WIT {
            IntWit::Zeros { .. } => None,
            IntWit::Nat(len_te) => {
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
    /// Returns `None` if `index >= self.len()`.
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
        match L::INT_WIT {
            IntWit::Zeros { .. } => None,
            IntWit::Nat(len_te) => {
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
        I: Int<Cmp<L> = TLess>,
    {
        self.index_alt::<I>(TypeEq::NEW)
    }

    /// Alternative version of [`index`](Self::index) which takes a proof of `I < L` as an argument.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, Peano, nlist, int};
    /// use nlist::int::{Int, proofs};
    /// use nlist::boolean::Bool;
    /// use nlist::typewit::TypeEq;
    /// 
    /// let list = nlist![3, 5, 8, 13];
    /// 
    /// assert_eq!(at(&list, int!(0)), (&3, &3));
    /// assert_eq!(at(&list, int!(1)), (&3, &5));
    /// assert_eq!(at(&list, int!(2)), (&5, &8));
    /// assert_eq!(at(&list, int!(3)), (&8, &13));
    /// 
    /// const fn at<T, L, At>(list: &NList<T, L>, _at: At) -> (&T, &T) 
    /// where
    ///     L: Int,
    ///     At: Int<Cmp<L> = TLess>,
    /// {
    ///     type Dist = Peano!(1);
    ///
    ///     // passes a proof of `At < L` as argument, gets back a proof that `At - Dist < L`
    ///     let sub_lt_l = proofs::compose_sub_lt::<At, Dist, L>(TypeEq::NEW);
    ///     
    ///     (
    ///         list.index_alt::<int::SubSat<At, Dist>>(sub_lt_l),
    ///         list.index::<At>(),
    ///     )
    /// }
    /// ```
    /// 
    pub const fn index_alt<I>(&self, i_lt_l_te: TypeEq<I::Cmp<L>, TLess>) -> &T 
    where
        I: Int
    {
        const fn inner<T, L, At>(list: &NList<T, L>, at: At) -> &T
        where
            L: Int,
            At: Int,
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
        I: Int<Cmp<L> = TLess>,
    {
        self.index_mut_alt::<I>(TypeEq::NEW)
    }


    /// Alternative version of [`index_mut`](Self::index_mut) 
    /// which takes a proof of `I < L` as an argument.
    ///
    pub const fn index_mut_alt<I>(
        &mut self,
        i_lt_l_te: TypeEq<I::Cmp<L>, TLess>,
    ) -> &mut T
    where
        I: Int,
    {
        const fn inner<T, L, At>(list: &mut NList<T, L>, at: At) -> &mut T
        where
            L: Int,
            At: Int,
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
    struct IndexListLenFn<T, I: Int, L: Int>;

    impl<B: Boolean> B => NList<T, IfTrueI<B, L, Nat<I>>>
}


enum IndexState<L, At>
where
    L: Int,
    At: Int,
{
    Iterating {
        // The `IfZeroI<At, L` part is necessary so that, 
        // when this enum is `Self::Finished`,
        // the recursive call to `inner` in the dead `Iterating` branch
        // doesn't cause const panics.
        // If rustc didn't evaluate const code in dead branches, this field would be:
        // `TypeEq<L, Nat<L::SubOneSat>>`
        l_te: TypeEq<L, int::AddOne<int::IfZeroI<At, L, L::SubOneSat>>>,
        at_te: TypeEq<At, int::CoerceNat<At>>,
    },
    Finished {
        l_te: TypeEq<L, int::CoerceNat<L>>,
    },
}

typewit::type_fn! {
    struct MapTailFn<L>;

    impl<At> At => Nat<int::IfZeroI<At, L, L::SubOneSat>>
    where
        At: Int,
        L: Int,
}

impl<L, At> IndexState<L, At>
where
    L: Int,
    At: Int,
{
    const NEW: Self = match (L::INT_WIT, At::INT_WIT) {
        (IntWit::Nat(l_te), IntWit::Nat(at_te)) => Self::Iterating {
            l_te: l_te.join(at_te.project::<MapTailFn<L>>().flip()),
            at_te,
        },
        (IntWit::Nat(l_te), IntWit::Zeros(_)) => IndexState::Finished { l_te },
        _ => concat_panic! {
            "indexing bug: ",
            " L: ", L::USIZE,
            " At: ", At::USIZE,
        },
    };
}
