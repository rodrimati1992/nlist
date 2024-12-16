use core::marker::PhantomData;
use core::mem::ManuallyDrop;

use const_panic::concat_panic;

use konst::destructure;

use typewit::{type_fn, CallFn, TypeCmp, TypeEq};

use super::{NList, NList2D, NListFn};
use crate::peano::{self, Int, IntWit, Nat, SubOneSat, Zeros};

impl<T, L: Int, L2: Int> NList<NList<T, L2>, L> {
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
            LSub: Int,
            LOuter: Int,
            LInner: Int,
            LAcc: Int,
            LRet: Int,
        {
            Iterating {
                lsub_wit: IntWit<LSub>,
                // ManuallyDrop is necessary because the compiler doesn't know 
                // that this is non-Drop
                state_wit: ManuallyDrop<
                    CallFn<IteratingWhatFn<T, LOuter, LInner, LAcc, LRet>, LSub>
                >,
            },
            Finished {
                outer_len: TypeEq<LOuter, Zeros>,
                sub_len: TypeEq<LSub, Zeros>,
                output_te: TypeEq<NList<T, LAcc>, NList<T, LRet>>,
            },
        }

        type_fn! {
            struct IteratingWhatFn<T, LOuter, LInner, LAcc, LRet>;

            impl<LSub> LSub => peano::IfZero<
                LSub,
                IteratingOuter<T, LSub, LOuter, LInner, LAcc, LRet>,
                IteratingInner<T, LSub, LOuter, LInner, LAcc, LRet>,
            >
            where
                LSub: Int,
                LOuter: Int,
                LInner: Int,
                LAcc: Int,
                LRet: Int,
        }

        typewit::type_fn! {
            /// Computes the `LSub` type argument for a recursive call to `inner`.
            struct SubTailLenFn<LInner>;

            impl<LSub, LOuter> (LSub, LOuter) => peano::IfZeroI<
                LSub,
                peano::IfZeroI<LOuter, Zeros, LInner>,
                SubOneSat<LSub>,
            >
            where
                LSub: Int,
                LOuter: Int,
                LInner: Int;
        }

        typewit::type_fn! {
            // Computes the types of the arguments for the recursive `inner` call
            struct CallArgsFn<T, LOuter, LInner, LAcc, LRet>;

            impl<LSub> LSub => (
                NList<T, CallFn<SubTailLenFn<LInner>, (LSub, LOuter)>>,
                NList2D<T, peano::IfZeroI<LSub, SubOneSat<LOuter>, LOuter>, LInner>,
                NList<T, peano::IfZeroI<LSub, LAcc, peano::Min<Nat<LAcc>, LRet>>>,
            )
            where
                LSub: Int,
                LOuter: Int,
                LInner: Int,
                LAcc: Int,
                LRet: Int,
        }

        // Witnesses for iteration over an `NList<T>`
        struct IteratingInner<T, LSub, LOuter, LInner, LAcc, LRet>
        where
            LSub: Int,
            LOuter: Int,
            LInner: Int,
            LAcc: Int,
            LRet: Int,
        {
            // Witness that the returned NList is at most `LRet` elements long
            output_te: TypeEq<
                NList<T, Nat<LAcc>>,
                // computes the minimum of `Nat<LAcc>` and `LRet`
                // so that `inner` doesn't monomorphize to values of
                // `LAcc` larger than `LRet`
                NList<T, peano::Min<Nat<LAcc>, LRet>>,
            >,
            _phantom: PhantomData<fn() -> (T, LSub, LOuter, LInner, LAcc, LRet)>,
        }

        // Witnesses for iteration over an `NList2D<T, LOuter, LInner>`
        struct IteratingOuter<T, LSub, LOuter, LInner, LAcc, LRet>
        where
            LSub: Int,
            LOuter: Int,
            LInner: Int,
            LAcc: Int,
            LRet: Int,
        {
            // Witness that the `NList2D` has at least one more sublist:
            // ```
            // NList2D<T, LOuter                    , LInner> ==
            // NList2D<T, Nat<LOuter::SubOneSat>, LInner>
            // ```
            outer_te:
                TypeEq<NList2D<T, LOuter, LInner>, NList2D<T, Nat<LOuter::SubOneSat>, LInner>>,
            // Witness that the next sublist is `LInner` long
            //
            // `CallFn<SubTailLenFn<LInner>, (LSub, LOuter)> == LInner`
            tail_te: TypeEq<CallFn<SubTailLenFn<LInner>, (LSub, LOuter)>, LInner>,
            _phantom: PhantomData<fn() -> (T, LSub, LOuter, LInner, LAcc, LRet)>,
        }

        impl<T, LSub, LOuter, LInner, LAcc, LRet> FlattenState<T, LSub, LOuter, LInner, LAcc, LRet>
        where
            LSub: Int,
            LOuter: Int,
            LInner: Int,
            LAcc: Int,
            LRet: Int,
        {
            const NEW: Self = match (
                LSub::INT_WIT,
                LOuter::INT_WIT,
                peano::eq::<LAcc, LRet>(),
            ) {
                (lsub_wit @ IntWit::Nat(sub_te), _, TypeCmp::Ne(_)) => {
                    let TypeCmp::Eq(output_te) = 
                        peano::eq::<Nat<LAcc>, peano::Min<Nat<LAcc>, LRet>>() 
                    else {
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
                    lsub_wit @ IntWit::Zeros(sub_te),
                    IntWit::Nat(outer_te),
                    TypeCmp::Ne(_),
                ) => FlattenState::Iterating {
                    lsub_wit,
                    state_wit: ManuallyDrop::new(
                        sub_te
                        .project::<IteratingWhatFn<T, LOuter, LInner, LAcc, LRet>>()
                        .to_left(IteratingOuter {
                            outer_te: outer_te.map(NListFn::NEW),
                            tail_te: TypeEq::new::<Zeros>()
                                .zip(outer_te)
                                .map(SubTailLenFn::<LInner>::NEW),
                            _phantom: PhantomData,
                        })
                    ),
                },
                (IntWit::Zeros(sub_len), IntWit::Zeros(outer_len), TypeCmp::Eq(output_te)) => {
                    FlattenState::Finished {
                        outer_len,
                        sub_len,
                        output_te: output_te.map(NListFn::NEW),
                    }
                }
                (IntWit::Zeros(_), IntWit::Zeros(_), TypeCmp::Ne(_)) => concat_panic! {
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
            LSub: Int,
            LOuter: Int,
            LInner: Int,
            LAcc: Int,
            LRet: Int,
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
                        IntWit::Zeros(zero_wit) => {
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
                        IntWit::Nat(one_wit) => {
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

        // `inner` assumes that LSub, LOuter, and LRet are all non-zero,
        // so FlattenWit allows us to assert that they're all non-zero when its called.
        match FlattenWit::<T, L, L2>::NEW {
            FlattenWit::ReturnsEmpty(ret_te) => {
                // leak-safety: list contains no elements, 
                // because it returns a zero-length list after flattening
                core::mem::forget(self);

                ret_te.to_left(NList::nil())
            }
            FlattenWit::ReturnsNonEmpty{arg_te, ret_te} => {
                let this = arg_te.to_right(self);
                let ret = inner(NList::nil(), this, NList::nil()).reverse();
                ret_te.to_left(ret)                
            }
        }
    }
}


typewit::type_fn! {
    struct NList2DFn<T>;

    impl<LOuter, LInner> (LOuter, LInner) => NList<NList<T, LInner>, LOuter>
    where
        LOuter: Int,
        LInner: Int,        
}

enum FlattenWit<T, LOuter, LInner> 
where
    LOuter: Int,
    LInner: Int,
{
    ReturnsNonEmpty {
        arg_te: TypeEq<
            NList<NList<T, LInner>, LOuter>,
            NList<NList<T, Nat<LInner::SubOneSat>>, Nat<LOuter::SubOneSat>>,
        >,
        ret_te: TypeEq<
            NList<T, peano::Mul<LOuter, LInner>>, 
            NList<T, peano::Mul<Nat<LOuter::SubOneSat>, Nat<LInner::SubOneSat>>>,
        >,
    },
    ReturnsEmpty(
        TypeEq<
            NList<T, peano::Mul<LOuter, LInner>>, 
            NList<T, Zeros>, 
        >
    ),
}

impl<T, LOuter, LInner> FlattenWit<T, LOuter, LInner>
where
    LOuter: Int,
    LInner: Int,
{
    const NEW: Self = match (LOuter::INT_WIT, LInner::INT_WIT) {
        (IntWit::Nat(louter_te), IntWit::Nat(linner_te)) => {
            let arg_te = louter_te.zip(linner_te).map(NList2DFn::NEW);

            let ret_te = louter_te.zip(linner_te).map(peano::MulFn::NEW).map(NListFn::NEW);

            Self::ReturnsNonEmpty { arg_te, ret_te }
        }
        (IntWit::Zeros(louter_te), _) => {
            let ret_te = louter_te
                .zip(TypeEq::new::<LInner>())
                .map(peano::MulFn::NEW)
                .map(NListFn::NEW);

            Self::ReturnsEmpty(ret_te)
        }
        (_, IntWit::Zeros(linner_te)) => {
            let ret_te = TypeEq::new::<LOuter>()
                .zip(linner_te)
                .map(peano::MulFn::NEW)
                .join(peano::proofs::commutative_mul::<LOuter, Zeros>())
                .map(NListFn::NEW);

            Self::ReturnsEmpty(ret_te)
        }
    };
}




