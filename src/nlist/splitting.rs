use core::marker::PhantomData;

use const_panic::concat_panic;
use typewit::{TypeCmp, TypeEq};

use super::NList;
use crate::peano::{self, PeanoInt, PeanoWit, PlusOne, SubOneSat, Zero};

impl<T, L: PeanoInt> NList<T, L> {
    /// Splits this list at the `At` index.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist, peano};
    ///
    /// let list = nlist![3, 5, 8, 13, 21, 34];
    ///
    /// let (before, after) = list.split_at::<peano!(4)>();
    /// assert_eq!(before, nlist![3, 5, 8, 13]);
    /// assert_eq!(after, nlist![21, 34]);
    ///
    /// ```
    pub fn split_at<At>(self) -> (NList<T, At>, NList<T, L::SubSat<At>>)
    where
        At: PeanoInt<Min<L> = At>,
    {
        enum SplitState<L, At, Rem>
        where
            L: PeanoInt,
            At: PeanoInt,
            Rem: PeanoInt,
        {
            Iterating {
                input_te: TypeEq<L, PlusOne<SubOneSat<L>>>,
                at_te: TypeEq<At, PlusOne<SubOneSat<At>>>,
                // necessary so that, when this enum is `Self::Finished`,
                // the recursive call to `inner` in the dead `Iterating` branch 
                // doesn't cause const panics.
                rem_te: TypeEq<peano::Min<SubOneSat<L>, Rem>, Rem>,
            },
            Finished {
                at_te: TypeEq<At, Zero>,
                rem_te: TypeEq<L, Rem>,
            },
        }

        impl<L, At, Rem> SplitState<L, At, Rem>
        where
            L: PeanoInt,
            At: PeanoInt,
            Rem: PeanoInt,
        {
            const NEW: Self = match (
                L::PEANO_WIT,
                At::PEANO_WIT,
                peano::cmp_peanos(peano::Min::<SubOneSat<L>, Rem>::NEW, Rem::NEW),
                peano::cmp_peanos(L::NEW, Rem::NEW),
            ) {
                (PeanoWit::PlusOne(input_te), PeanoWit::PlusOne(at_te), TypeCmp::Eq(rem_te), _) => {
                    SplitState::Iterating { input_te, at_te, rem_te }
                }
                (_, PeanoWit::Zero(at_te), _, TypeCmp::Eq(rem_te)) => {
                    SplitState::Finished { at_te, rem_te }
                }
                _ => concat_panic! {
                    "bug in `NList::split_at`, ", 
                    " L: ", L::USIZE,
                    " At: ", At::USIZE,
                    " Rem: ", Rem::USIZE,
                }
            };
        }

        fn inner<T, L, At, Rem>(list: NList<T, L>) -> (NList<T, At>, NList<T, Rem>) 
        where
            L: PeanoInt,
            At: PeanoInt,
            Rem: PeanoInt,
        {
            match SplitState::<L, At, Rem>::NEW {
                SplitState::Iterating{input_te, at_te, rem_te} => {
                    let (head, tail) = list.coerce_length(input_te).into_split_head();

                    let (prefix, suffix) = inner(tail);
                    (NList::cons_sub(head, prefix, at_te), suffix.coerce_length(rem_te))
                }
                SplitState::Finished{at_te, rem_te} => {
                    (NList::nil_sub(at_te), list.coerce_length(rem_te))
                }
            }
        }

        inner(self)
    }
}
