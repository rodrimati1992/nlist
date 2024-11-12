use core::marker::PhantomData;

use const_panic::concat_panic;
use typewit::{type_fn, CallFn, TypeCmp, TypeEq};

use super::{NList, NList2D, NListFn};
use crate::peano::{self, PeanoInt, PeanoWit, PlusOne, SubOneSat, Zero};

impl<T, L: PeanoInt, L2: PeanoInt> NList<NList<T, L2>, L> {
    /// Flattens a nested list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let nested = nlist![
    ///     nlist![3, 5],
    ///     nlist![8, 13],
    ///     nlist![21, 34],
    /// ];
    ///
    /// assert_eq!(
    ///     nested.flatten(),
    ///     nlist![3, 5, 8, 13, 21, 34],
    /// );
    /// ```
    pub fn flatten(self) -> NList<T, peano::Mul<L, L2>> {
        // The current state of iteration over an NList,
        // as determined by the type arguments.
        //
        // LSub: the length of the inner List<T> that we're grabbing the elements from
        // LOuter: from `NList2D<T, LOuter, LInner>`, the length of the list of lists
        // LInner: from `NList2D<T, LOuter, LInner>`, the length of the "next" inner lists
        // LAcc: the length of the `NList<T, LAcc>` accumulator list
        // LRet: the length of the `NList<T, LRet>` that we're expected to return
        enum FlattenState<T, LSub, LOuter, LInner, LAcc, LRet>
        where
            LSub: PeanoInt,
            LOuter: PeanoInt,
            LInner: PeanoInt,
            LAcc: PeanoInt,
            LRet: PeanoInt,
        {
            Iterating {
                lsub_wit: PeanoWit<LSub>,
                state_wit: CallFn<IteratingWhatFn<T, LOuter, LInner, LAcc, LRet>, LSub>,
            },
            Finished {
                output_te: TypeEq<NList<T, LAcc>, NList<T, LRet>>,
            },
        }

        type_fn! {
            struct IteratingWhatFn<T, LOuter, LInner, LAcc, LRet>;

            impl<LSub> LSub => LSub::IfZero<
                IteratingOuter<T, LSub, LOuter, LInner, LAcc, LRet>,
                IteratingInner<T, LSub, LOuter, LInner, LAcc, LRet>,
            >
            where
                LSub: PeanoInt,
                LOuter: PeanoInt,
                LInner: PeanoInt,
                LAcc: PeanoInt,
                LRet: PeanoInt,
        }

        typewit::type_fn! {
            /// Computes the `LSub` type argument for a recursive call to `inner`.
            struct SubTailLenFn<LInner>;

            impl<LSub, LOuter> (LSub, LOuter) => LSub::IfZeroPI<
                LOuter::IfZeroPI<Zero, LInner>,
                SubOneSat<LSub>,
            >
            where
                LSub: PeanoInt,
                LOuter: PeanoInt,
                LInner: PeanoInt;
        }

        typewit::type_fn! {
            // Computes the types of the arguments for the recursive `inner` call
            struct CallArgsFn<T, LOuter, LInner, LAcc, LRet>;

            impl<LSub> LSub => (
                NList<T, CallFn<SubTailLenFn<LInner>, (LSub, LOuter)>>,
                NList2D<T, LSub::IfZeroPI<SubOneSat<LOuter>, LOuter>, LInner>,
                NList<T, LSub::IfZeroPI<LAcc, peano::Min<PlusOne<LAcc>, LRet>>>,
            )
            where
                LSub: PeanoInt,
                LOuter: PeanoInt,
                LInner: PeanoInt,
                LAcc: PeanoInt,
                LRet: PeanoInt,
        }

        // Witnesses for iteration over an `NList<T>`
        struct IteratingInner<T, LSub, LOuter, LInner, LAcc, LRet>
        where
            LSub: PeanoInt,
            LOuter: PeanoInt,
            LInner: PeanoInt,
            LAcc: PeanoInt,
            LRet: PeanoInt,
        {
            // Witness that the returned NList is at most `LRet` elements long
            output_te: TypeEq<
                NList<T, PlusOne<LAcc>>,
                // computes the minimum of `PlusOne<LAcc>` and `LRet`
                // so that `inner` doesn't monomorphize to values of
                // `LAcc` larger than `LRet`
                NList<T, peano::Min<PlusOne<LAcc>, LRet>>,
            >,
            _phantom: PhantomData<fn() -> (T, LSub, LOuter, LInner, LAcc, LRet)>,
        }

        // Witnesses for iteration over an `NList2D<T, LOuter, LInner>`
        struct IteratingOuter<T, LSub, LOuter, LInner, LAcc, LRet>
        where
            LSub: PeanoInt,
            LOuter: PeanoInt,
            LInner: PeanoInt,
            LAcc: PeanoInt,
            LRet: PeanoInt,
        {
            // Witness that the `NList2D` has at least one more sublist:
            // ```
            // NList2D<T, LOuter                    , LInner> ==
            // NList2D<T, PlusOne<LOuter::SubOneSat>, LInner>
            // ```
            outer_te:
                TypeEq<NList2D<T, LOuter, LInner>, NList2D<T, PlusOne<LOuter::SubOneSat>, LInner>>,
            // Witness that the next sublist is `LInner` long
            //
            // `CallFn<SubTailLenFn<LInner>, (LSub, LOuter)> == LInner`
            tail_te: TypeEq<CallFn<SubTailLenFn<LInner>, (LSub, LOuter)>, LInner>,
            _phantom: PhantomData<fn() -> (T, LSub, LOuter, LInner, LAcc, LRet)>,
        }

        impl<T, LSub, LOuter, LInner, LAcc, LRet> FlattenState<T, LSub, LOuter, LInner, LAcc, LRet>
        where
            LSub: PeanoInt,
            LOuter: PeanoInt,
            LInner: PeanoInt,
            LAcc: PeanoInt,
            LRet: PeanoInt,
        {
            const NEW: Self = match (
                LSub::PEANO_WIT,
                LOuter::PEANO_WIT,
                peano::cmp_peanos(LAcc::NEW, LRet::NEW),
            ) {
                (lsub_wit @ PeanoWit::PlusOne(sub_te), _, TypeCmp::Ne(_)) => {
                    let TypeCmp::Eq(output_te) = peano::cmp_peanos(
                        PlusOne::<LAcc>::NEW,
                        peano::Min::<PlusOne<LAcc>, LRet>::NEW,
                    ) else {
                        concat_panic! {"somehow, LAcc > LRet: ", LAcc::USIZE, " > ", LRet::USIZE}
                    };

                    FlattenState::Iterating {
                        lsub_wit,
                        state_wit: sub_te
                            .project::<IteratingWhatFn<T, LOuter, LInner, LAcc, LRet>>()
                            .to_left(IteratingInner {
                                output_te: output_te.map(NListFn::NEW),
                                _phantom: PhantomData,
                            }),
                    }
                }
                (
                    lsub_wit @ PeanoWit::Zero(sub_te),
                    PeanoWit::PlusOne(outer_te),
                    TypeCmp::Ne(_),
                ) => FlattenState::Iterating {
                    lsub_wit,
                    state_wit: sub_te
                        .project::<IteratingWhatFn<T, LOuter, LInner, LAcc, LRet>>()
                        .to_left(IteratingOuter {
                            outer_te: outer_te.map(NListFn::NEW),
                            tail_te: TypeEq::new::<Zero>()
                                .zip(outer_te)
                                .map(SubTailLenFn::<LInner>::NEW),
                            _phantom: PhantomData,
                        }),
                },
                (PeanoWit::Zero(_), PeanoWit::Zero(_), TypeCmp::Eq(output_te)) => {
                    FlattenState::Finished {
                        output_te: output_te.map(NListFn::NEW),
                    }
                }
                (PeanoWit::Zero(_), PeanoWit::Zero(_), TypeCmp::Ne(_)) => concat_panic! {
                    "finished iteration but LAcc != LRet, ",
                    " LAcc: " , LAcc::USIZE,
                    " LRet: ", LRet::USIZE
                },
                _ => concat_panic! {
                    "bug in flatten's implementation, ",
                    " LSub: ", LSub::USIZE,
                    " LInner: ", LInner::USIZE,
                    " LOuter: ", LOuter::USIZE,
                    " LAcc: " , LAcc::USIZE,
                    " LRet: ", LRet::USIZE
                },
            };
        }

        fn inner<T, LSub, LOuter, LInner, LAcc, LRet>(
            sub: NList<T, LSub>,
            outer: NList2D<T, LOuter, LInner>,
            output: NList<T, LAcc>,
        ) -> NList<T, LRet>
        where
            LSub: PeanoInt,
            LOuter: PeanoInt,
            LInner: PeanoInt,
            LAcc: PeanoInt,
            LRet: PeanoInt,
        {
            match FlattenState::<T, LSub, LOuter, LInner, LAcc, LRet>::NEW {
                FlattenState::Iterating {
                    lsub_wit,
                    state_wit,
                } => {
                    let swfn = IteratingWhatFn::<T, LOuter, LInner, LAcc, LRet>::NEW;
                    let cargfn = CallArgsFn::<T, LOuter, LInner, LAcc, LRet>::NEW;

                    let (next_sub, next_outer, next_output): CallFn<
                        CallArgsFn<T, LOuter, LInner, LAcc, LRet>,
                        LSub,
                    > = match lsub_wit {
                        PeanoWit::Zero(zero_wit) => {
                            let IteratingOuter {
                                outer_te, tail_te, ..
                            } = zero_wit.map(swfn).to_right(state_wit);

                            let (newsub, tail) = outer_te.to_right(outer).into_split_head();

                            let newsub = newsub.coerce_length(tail_te.flip());

                            zero_wit.map(cargfn).to_left((newsub, tail, output))
                        }
                        PeanoWit::PlusOne(one_wit) => {
                            let IteratingInner { output_te, .. } =
                                one_wit.map(swfn).to_right(state_wit);

                            let (elem, tail) = sub.coerce_length(one_wit).into_split_head();

                            one_wit.map(cargfn).to_left((
                                tail,
                                outer,
                                output_te.to_right(NList::cons(elem, output)),
                            ))
                        }
                    };

                    inner(next_sub, next_outer, next_output)
                }
                FlattenState::Finished { output_te, .. } => output_te.to_right(output),
            }
        }

        inner(NList::nil(), self, NList::nil()).reverse()
    }
}
