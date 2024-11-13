use const_panic::concat_panic;
use typewit::{TypeCmp, TypeEq};

#[allow(unused_imports)]
use crate::peano::{
    self, IntoPeano, IntoUsize, PeanoInt, PeanoWit, PlusOne, SubOneSat, Usize, Zero,
};

#[allow(unused_imports)]
use super::{AddPeanoFn, Nil, Cons, NList, NListFn, NodeFn};



impl<T, L: PeanoInt> NList<T, L> {

    /// Finds the first element for which `predicate(&element)` returns true.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list = nlist![3, 5, 8, 13];
    ///
    /// assert_eq!(list.copy().find(|x| *x > 6), Some(8));
    /// assert_eq!(list.copy().find(|x| *x == 10), None);
    ///
    /// ```
    pub fn find<F>(self, mut f: F) -> Option<T>
    where
        F: FnMut(&T) -> bool,
    {
        self._find_helper(move |_, x| f(&x).then_some(x))
    }

    /// Iterates the list elements,
    /// returning the first non-None return value of `mapper(element)`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list = nlist!["foo", "bar", "10", "baz", "20"];
    ///
    /// assert_eq!(list.copy().find_map(|x| x.parse::<u8>().ok()), Some(10));
    /// assert_eq!(list.copy().find_map(|x| x.parse::<bool>().ok()), None);
    ///
    /// ```
    pub fn find_map<F, R>(self, mut mapper: F) -> Option<R>
    where
        F: FnMut(T) -> Option<R>,
    {
        self._find_helper(move |_, x| mapper(x))
    }

    /// Finds the first index for which `predicate` returns true.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list = nlist!["hello", "hi", "goodbye", "world", "hi"];
    ///
    /// assert_eq!(list.copy().position(|x| x == "hi"), Some(1));
    /// assert_eq!(list.copy().position(|x| x.is_empty()), None);
    ///
    /// ```
    pub fn position<F>(self, mut predicate: F) -> Option<usize>
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
            match L::PEANO_WIT {
                PeanoWit::Zero { .. } => None,
                PeanoWit::PlusOne(len_te) => {
                    let Cons { elem, next, .. } = list.coerce_len(len_te).node;

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
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list = nlist![3, 5, 8, 13, 2];
    ///
    /// assert_eq!(list.copy().rfind(|x| *x > 6), Some(13));
    /// assert_eq!(list.copy().rfind(|x| *x == 10), None);
    ///
    /// ```
    pub fn rfind<F>(self, mut f: F) -> Option<T>
    where
        F: FnMut(&T) -> bool,
    {
        self._rfind_helper(move |_, x| f(&x).then_some(x))
    }

    /// Iterates the list elements in reverse,
    /// returning the first non-None return value of `mapper(element)`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list = nlist!["foo", "bar", "10", "baz", "20", "qux"];
    ///
    /// assert_eq!(list.copy().rfind_map(|x| x.parse::<u8>().ok()), Some(20));
    /// assert_eq!(list.copy().rfind_map(|x| x.parse::<bool>().ok()), None);
    ///
    /// ```
    pub fn rfind_map<F, R>(self, mut mapper: F) -> Option<R>
    where
        F: FnMut(T) -> Option<R>,
    {
        self._rfind_helper(move |_, x| mapper(x))
    }

    /// Iterates the list elements in reverse,
    /// Finds the index of the first element for which `predicate` returns true.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list = nlist!["hello", "hi", "goodbye", "world", "hi"];
    ///
    /// assert_eq!(list.copy().rposition(|x| x == "hi"), Some(4));
    /// assert_eq!(list.copy().rposition(|x| x.is_empty()), None);
    ///
    /// ```
    pub fn rposition<F>(self, mut predicate: F) -> Option<usize>
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
            match L::PEANO_WIT {
                PeanoWit::Zero { .. } => None,
                PeanoWit::PlusOne(len_te) => {
                    let Cons { elem, next, .. } = list.coerce_len(len_te).node;

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

    /// Consumes and returns a reversed version of this list
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list = nlist![3, 5, 8, 13];
    ///
    /// assert_eq!(list.reverse(), nlist![13, 8, 5, 3]);
    ///
    /// ```
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
            const NEW: Self = match (LI::PEANO_WIT, peano::cmp(LA::NEW, LR::NEW)) {
                (PeanoWit::PlusOne(input_te), TypeCmp::Ne(_)) => {
                    let TypeCmp::Eq(output_te) =
                        peano::cmp(PlusOne::<LA>::NEW, peano::Min::<PlusOne<LA>, LR>::NEW)
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

        match L::PEANO_WIT {
            PeanoWit::Zero { .. } => self,
            PeanoWit::PlusOne(len_te) => {
                let (elem, tail) = self.coerce_len(len_te).into_split_head();

                inner(tail, NList::cons(elem, NList::nil()))
            }
        }
    }

    /// Concatenates this list with another one
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let first = nlist![3, 5];
    /// let second = nlist![8, 13, 21, 34];
    ///
    /// assert_eq!(first.concat(second), nlist![3, 5, 8, 13, 21, 34]);
    ///
    /// ```
    pub fn concat<L2>(self, other: NList<T, L2>) -> NList<T, peano::Add<L, L2>>
    where
        L2: PeanoInt,
    {
        fn inner<T, LA, LB>(lhs: NList<T, LA>, rhs: NList<T, LB>) -> NList<T, peano::Add<LA, LB>>
        where
            LA: PeanoInt,
            LB: PeanoInt,
        {
            match LA::PEANO_WIT {
                PeanoWit::Zero(len_te) => len_te
                    .zip(TypeEq::new::<LB>())
                    .map(AddPeanoFn::NEW)
                    .map(NListFn::NEW)
                    .to_left(rhs),
                PeanoWit::PlusOne(len_te) => {
                    let Cons { elem, next, .. } = lhs.coerce_len(len_te).node;

                    let len_te = len_te.zip(TypeEq::new::<LB>()).map(AddPeanoFn::NEW);

                    NList::cons_sub(elem, inner(next, rhs), len_te)
                }
            }
        }

        inner(self, other)
    }

    /// Zips this list with another one of the same length
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let first = nlist![3, 5, 8];
    /// let second = nlist![13, 21, 34];
    ///
    /// assert_eq!(first.zip(second), nlist![(3, 13), (5, 21), (8, 34)]);
    ///
    /// ```
    pub fn zip<U>(self, other: NList<U, L>) -> NList<(T, U), L> {
        fn inner<T, U, L>(lhs: NList<T, L>, rhs: NList<U, L>) -> NList<(T, U), L>
        where
            L: PeanoInt,
        {
            match L::PEANO_WIT {
                PeanoWit::Zero(len_te) => NList::nil_sub(len_te),
                PeanoWit::PlusOne(len_te) => {
                    let lhs = len_te.map(NodeFn::NEW).to_right(lhs.node);
                    let rhs = len_te.map(NodeFn::NEW).to_right(rhs.node);

                    NList::cons_sub((lhs.elem, rhs.elem), inner(lhs.next, rhs.next), len_te)
                }
            }
        }

        inner(self, other)
    }


    /// Maps the elements of this list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list = nlist![3, 5, 8, 13];
    ///
    /// assert_eq!(list.map(|x| x * 2), nlist![6, 10, 16, 26]);
    ///
    /// ```
    pub fn map<F, R>(self, mut f: F) -> NList<R, L>
    where
        F: FnMut(T) -> R,
    {
        match L::PEANO_WIT {
            PeanoWit::Zero(len_te) => NList::nil_sub(len_te),

            PeanoWit::PlusOne(len_te) => {
                let Cons { elem, next, .. } = self.coerce_len(len_te).node;
                NList::cons_sub(f(elem), next.map(f), len_te)
            }
        }
    }

    /// Loops over the elements in the list, along with their index.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list = nlist![3, 5, 8, 13];
    /// let mut out = Vec::new();
    ///
    /// list.for_each(|i, x| out.push((i, x)));
    ///
    /// assert_eq!(out, vec![(0, 3), (1, 5), (2, 8), (3, 13)]);
    ///
    /// ```
    pub fn for_each<F>(self, f: F)
    where
        F: FnMut(usize, T),
    {
        fn inner<T, L, F>(list: NList<T, L>, index: usize, mut func: F)
        where
            L: PeanoInt,
            F: FnMut(usize, T),
        {
            if let PeanoWit::PlusOne(len_te) = L::PEANO_WIT {
                let Cons { elem, next, .. } = list.coerce_len(len_te).node;
                func(index, elem);
                inner(next, index + 1, func)
            }
        }

        inner(self, 0, f)
    }

    /// Returns whether `predicate(elem)` returns true for any element.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist};
    ///
    /// let list = nlist![3, 5, 8, 13];
    ///
    /// // calling `any` on an empty list always returns false
    /// assert!(!NList::nil::<u8>().any(|_| false));
    ///
    /// assert!(list.copy().any(|x| x % 2 == 0));
    /// assert!(!list.copy().any(|x| x > 20));
    ///
    /// ```
    pub fn any<F>(self, mut predicate: F) -> bool
    where
        F: FnMut(T) -> bool,
    {
        !self.all(|x| !predicate(x))
    }

    /// Returns whether `predicate(&elem)` returns true for all elements.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::{NList, nlist};
    ///
    /// let list = nlist![3, 5, 8, 13];
    ///
    /// // calling `all` on an empty list always returns true
    /// assert!(NList::nil::<u8>().all(|_| false));
    ///
    /// assert!(list.copy().all(|x| x < 20));
    /// assert!(!list.copy().all(|x| x % 2 == 0));
    ///
    /// ```
    pub fn all<F>(self, predicate: F) -> bool
    where
        F: FnMut(T) -> bool,
    {
        fn inner<T, L, F>(list: NList<T, L>, mut func: F) -> bool
        where
            L: PeanoInt,
            F: FnMut(T) -> bool,
        {
            match L::PEANO_WIT {
                PeanoWit::Zero { .. } => true,
                PeanoWit::PlusOne(len_te) => {
                    let Cons { elem, next, .. } = list.coerce_len(len_te).node;
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
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list = nlist![3, 5, 8, 13];
    ///
    /// let res = list.fold(Vec::new(), |mut out, elem| {
    ///     out.push(elem);
    ///     out
    /// });
    /// 
    /// assert_eq!(res, vec![3, 5, 8, 13]);
    ///
    /// ```
    pub fn fold<F, A>(self, initial_value: A, mut f: F) -> A
    where
        F: FnMut(A, T) -> A,
    {
        match L::PEANO_WIT {
            PeanoWit::Zero { .. } => initial_value,
            PeanoWit::PlusOne(len_te) => {
                let Cons { elem, next, .. } = self.coerce_len(len_te).node;
                next.fold(f(initial_value, elem), f)
            }
        }
    }
}

impl<T, L: PeanoInt> NList<T, PlusOne<L>> {
    /// Equivalent to [`fold`](NList::fold), where the head element is passed as the `initial_value`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nlist::nlist;
    ///
    /// let list = nlist![3, 5, 8, 13];
    ///
    /// assert_eq!(list.reduce(|l, r| l + r), 29);
    ///
    /// ```
    pub fn reduce<F>(self, f: F) -> T
    where
        F: FnMut(T, T) -> T,
    {
        self.node.next.fold(self.node.elem, f)
    }
}
