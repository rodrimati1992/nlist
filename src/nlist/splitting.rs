use const_panic::concat_panic;
use konst::destructure;
use typewit::{const_marker::Bool, TypeCmp, TypeEq};

use super::NList;
use crate::peano::{self, PeanoInt, PeanoWit, PlusOne, SubOneSat, Zero};
use crate::boolean::{IfTrueI, Boolean};

impl<T, L: PeanoInt> NList<T, L> {
    /// Splits this list at the `At` index.
    ///
    /// # Examples
    ///
    /// ### Basic usage
    ///
    /// ```rust
    /// use nlist::{NList, Peano, nlist};
    ///
    /// let list = nlist![3, 5, 8, 13, 21, 34];
    ///
    /// let (before, after) = list.split_at::<Peano!(4)>();
    /// assert_eq!(before, nlist![3, 5, 8, 13]);
    /// assert_eq!(after, nlist![21, 34]);
    ///
    /// ```
    pub const fn split_at<At>(self) -> (NList<T, At>, NList<T, peano::SubSat<L, At>>)
    where
        At: peano::PeanoInt<IsLe<L> = Bool<true>>,
    {
        self.split_at_alt(TypeEq::new::<At::IsLe<L>>())
    }

    /// Alternate version of `split_at` which takes a proof of `At <= L`
    /// instead of requiring it as a bound.
    /// 
    /// # Example
    /// 
    /// ### Conditional splitting
    ///
    /// This example demonstrates how `split_at_alt` can be used inside a generic function,
    /// only calling `split_at_alt` when the split index is in bounds.
    ///
    /// ```rust
    /// use nlist::{NList, NListFn, Peano, PeanoInt, PeanoInt as PInt, nlist, peano};
    /// 
    /// use nlist::boolean::{Boolean, BoolWitG};
    /// 
    /// assert_eq!(insert_at_3(nlist![]), nlist![100, 103]);
    /// assert_eq!(insert_at_3(nlist![3]), nlist![3, 100, 103]);
    /// assert_eq!(insert_at_3(nlist![3, 5]), nlist![3, 5, 100, 103]);
    /// assert_eq!(insert_at_3(nlist![3, 5, 8]), nlist![3, 5, 8, 100, 103]);
    /// assert_eq!(insert_at_3(nlist![3, 5, 8, 13]), nlist![3, 5, 8, 100, 103, 13]);
    /// assert_eq!(insert_at_3(nlist![3, 5, 8, 13, 21]), nlist![3, 5, 8, 100, 103, 13, 21]);
    /// 
    /// 
    /// type Added = Peano!(2);
    /// 
    /// fn insert_at_3<L>(list: NList<u32, L>) -> NList<u32, peano::Add<L, Added>> 
    /// where
    ///     L: PeanoInt   
    /// {
    ///     let to_add = nlist![100, 103];
    /// 
    ///     type At = Peano!(3);
    /// 
    ///     let opt_te = const {
    ///         match peano::IsLe::<At, L>::BOOL_WIT {
    ///             BoolWitG::True(is_le_te) => 
    ///                 // `.unwrap_eq()` can only panic if this function has a bug
    ///                 Some((is_le_te, peano::eq().unwrap_eq())),
    ///             BoolWitG::False(_) => None,
    ///         }
    ///     };
    /// 
    ///     // Because the compiler doesn't understand arithmetic properties of PeanoInt,
    ///     // this function has to assert lengths in the above const block.
    ///     //
    ///     // `is_le_te`: proof that `At <= L`, which allows splitting the list at `At`.
    ///     // 
    ///     // `ret_len_te`: proof that the length of `before.concat(to_add).concat(after)`
    ///     // (`At + Added + (L - At)`) is the same as the return type: (`L + Added`).
    ///     if let Some((is_le_te, ret_len_te)) = opt_te {
    ///         let (before, after) = list.split_at_alt::<At>(is_le_te);
    ///         
    ///         before.concat(to_add).concat(after).coerce_len(ret_len_te)
    ///     } else {
    ///         list.concat(to_add)
    ///     }
    /// }
    /// ```
    ///
    pub const fn split_at_alt<At>(
        self,
        at_le_l_proof: TypeEq<At::IsLe<L>, Bool<true>>,
    ) -> (NList<T, At>, NList<T, peano::SubSat<L, At>>)
    where
        At: peano::PeanoInt,
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
                peano::eq::<peano::Min<SubOneSat<L>, Rem>, Rem>(),
                peano::eq::<L, Rem>(),
            ) {
                (PeanoWit::PlusOne(input_te), PeanoWit::PlusOne(at_te), TypeCmp::Eq(rem_te), _) => {
                    SplitState::Iterating {
                        input_te,
                        at_te,
                        rem_te,
                    }
                }
                (_, PeanoWit::Zero(at_te), _, TypeCmp::Eq(rem_te)) => {
                    SplitState::Finished { at_te, rem_te }
                }
                _ => concat_panic! {
                    "bug in `NList::split_at`, ",
                    " L: ", L::USIZE,
                    " At: ", At::USIZE,
                    " Rem: ", Rem::USIZE,
                },
            };
        }

        const fn inner<T, L, At, Rem>(list: NList<T, L>) -> (NList<T, At>, NList<T, Rem>)
        where
            L: PeanoInt,
            At: PeanoInt,
            Rem: PeanoInt,
        {
            match SplitState::<L, At, Rem>::NEW {
                SplitState::Iterating {
                    input_te,
                    at_te,
                    rem_te,
                } => {
                    destructure!{(head, tail) = list.coerce_len(input_te).into_split_head()}

                    destructure!{(prefix, suffix) = inner(tail)}

                    (
                        NList::cons_sub(head, prefix, at_te),
                        suffix.coerce_len(rem_te),
                    )
                }
                SplitState::Finished { at_te, rem_te } => {
                    (NList::nil_sub(at_te), list.coerce_len(rem_te))
                }
            }
        }

        at_le_l_proof
            .project::<SplitAtRetTypeFn::<T, At, L>>()
            .to_right(inner(self))
    }
}

typewit::type_fn!{
    struct SplitAtRetTypeFn<T, At, L>;

    impl<B> B => (NList<T, IfTrueI<B, At, L>>, NList<T, peano::SubSat<L, At>>)
    where
        B: Boolean,
        At: PeanoInt,
        L: PeanoInt,
}
