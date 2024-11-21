use core::marker::PhantomData;
use core::mem::ManuallyDrop;

use const_panic::concat_panic;

use konst::destructure;

use typewit::{type_fn, CallFn, TypeCmp, TypeEq};

use super::{NList, NList2D, NListFn};
use crate::peano::{self, PeanoInt, PeanoWit, PlusOne, SubOneSat, Zero};

impl<T, L: PeanoInt, L2: PeanoInt> NList<NList<T, L2>, L> {
    /// Flattens a nested list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, Peano, nlist};
    ///
    /// const FLATTENED: NList<u32, Peano!(6)> = 
    ///     nlist![
    ///         nlist![3, 5],
    ///         nlist![8, 13],
    ///         nlist![21, 34],
    ///     ].flatten();
    ///
    /// assert_eq!(FLATTENED, nlist![3, 5, 8, 13, 21, 34]);
    /// ```
    pub const fn flatten(self) -> NList<T, peano::Mul<L, L2>> {
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
                // ManuallyDrop is necessary because the compiler doesn't know 
                // that this is non-Drop
                state_wit: ManuallyDrop<
                    CallFn<IteratingWhatFn<T, LOuter, LInner, LAcc, LRet>, LSub>
                >,
            },
            Finished {
                outer_len: TypeEq<LOuter, Zero>,
                sub_len: TypeEq<LSub, Zero>,
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
                peano::eq(LAcc::NEW, LRet::NEW),
            ) {
                (lsub_wit @ PeanoWit::PlusOne(sub_te), _, TypeCmp::Ne(_)) => {
                    let TypeCmp::Eq(output_te) = peano::eq(
                        PlusOne::<LAcc>::NEW,
                        peano::Min::<PlusOne<LAcc>, LRet>::NEW,
                    ) else {
                        concat_panic! {"somehow, LAcc > LRet: ", LAcc::USIZE, " > ", LRet::USIZE}
                    };

                    FlattenState::Iterating {
                        lsub_wit,
                        state_wit: ManuallyDrop::new(
                            sub_te
                            .project::<IteratingWhatFn<T, LOuter, LInner, LAcc, LRet>>()
                            .to_left(IteratingInner {
                                output_te: output_te.map(NListFn::NEW),
                                _phantom: PhantomData,
                            })
                        ),
                    }
                }
                (
                    lsub_wit @ PeanoWit::Zero(sub_te),
                    PeanoWit::PlusOne(outer_te),
                    TypeCmp::Ne(_),
                ) => FlattenState::Iterating {
                    lsub_wit,
                    state_wit: ManuallyDrop::new(
                        sub_te
                        .project::<IteratingWhatFn<T, LOuter, LInner, LAcc, LRet>>()
                        .to_left(IteratingOuter {
                            outer_te: outer_te.map(NListFn::NEW),
                            tail_te: TypeEq::new::<Zero>()
                                .zip(outer_te)
                                .map(SubTailLenFn::<LInner>::NEW),
                            _phantom: PhantomData,
                        })
                    ),
                },
                (PeanoWit::Zero(sub_len), PeanoWit::Zero(outer_len), TypeCmp::Eq(output_te)) => {
                    FlattenState::Finished {
                        outer_len,
                        sub_len,
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

        const fn inner<T, LSub, LOuter, LInner, LAcc, LRet>(
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

                    let fn_args: CallFn<
                        CallArgsFn<T, LOuter, LInner, LAcc, LRet>,
                        LSub,
                    > = match lsub_wit {
                        PeanoWit::Zero(zero_wit) => {
                            // works around "destructor cannot be evaluated at compile-time" error
                            _ = sub.coerce_len(zero_wit);

                            let IteratingOuter {
                                outer_te, tail_te, ..
                            } = zero_wit.map(swfn).to_right(ManuallyDrop::into_inner(state_wit));

                            destructure!{
                                (newsub, tail) = outer_te.to_right(outer).into_split_head()
                            }

                            let newsub = newsub.coerce_len(tail_te.flip());

                            zero_wit.map(cargfn).to_left((newsub, tail, output))
                        }
                        PeanoWit::PlusOne(one_wit) => {
                            let IteratingInner { output_te, .. } =
                                one_wit.map(swfn).to_right(ManuallyDrop::into_inner(state_wit));

                            destructure!{(elem, tail) = sub.coerce_len(one_wit).into_split_head()}

                            one_wit.map(cargfn).to_left((
                                tail,
                                outer,
                                output_te.to_right(NList::cons(elem, output)),
                            ))
                        }
                    };

                    destructure!{(next_sub, next_outer, next_output) = fn_args}

                    inner(next_sub, next_outer, next_output)
                }
                FlattenState::Finished { sub_len, outer_len, output_te, .. } => {
                    // these casts fix "destructor cannot be evaluated at compile-time" error
                    _ = sub.coerce_len(sub_len);
                    _ = outer.coerce_len(outer_len);

                    output_te.to_right(output)
                }
            }
        }

        inner(NList::nil(), self, NList::nil()).reverse()
    }
}
