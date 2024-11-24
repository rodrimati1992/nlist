
/// Helper for writing const fn equivalents of [`NList::from_fn`]
/// 
/// This macro acts like a function with this signature:
/// 
/// ```rust
/// use nlist::{NList, PeanoInt};
/// 
/// fn rec_from_fn<F, T, L>(func: F) -> NList<T, L>
/// where
///     L: PeanoInt,
///     F: FnOnce() -> (T, NList<T, L::SubOneSat>),
/// # {
/// #     nlist::rec_from_fn!(func)
/// # }
/// ```
/// 
/// The closure is called when the produced list is non-empty (i.e.: `L != 0`).
/// 
/// This doesn't pass an index to the closure, while [`NList::from_fn`] does.
/// 
/// # Alternatives
/// 
/// An alternative for constructing an NList with a closure is to use 
/// [`konst::array::from_fn_`]+[`NList::from_array`]:
/// ```rust
/// # use nlist::NList;
/// # use Some as some_closure_code;
/// # type T = Option<usize>;
/// # type L = nlist::Peano!(4);
/// const LIST: NList<T, L> = NList::from_array(konst::array::from_fn_!(some_closure_code));
/// ```
/// which works for small lengths (those that impl the [`IntoPeano`] trait)
/// 
/// # Example
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt};
/// 
/// const POWS: NList<u128, Peano!(5)> = powers_of_two();
/// 
/// assert_eq!(POWS.into_array(), [1, 2, 4, 8, 16]);
/// 
/// const fn powers_of_two<L: PeanoInt>() -> NList<u128, L> {
///     const fn inner<L: PeanoInt>(pow: u32) -> NList<u128, L> {
///         nlist::rec_from_fn!(|| (1 << pow, inner(pow + 1)))
///     }
/// 
///     inner(0)
/// }
/// ```
/// 
/// [`NList::from_array`]: crate::NList::from_array
/// [`NList::from_fn`]: crate::NList::from_fn
/// [`IntoPeano`]: crate::peano::IntoPeano
#[macro_export]
macro_rules! rec_from_fn {
    ($($closure:tt)*) => {
        $crate::__parse_closure_0_args!{__rec_from_fn () $($closure)*}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __rec_from_fn {
    (|| -> $ret_ty:ty $block:block) => {
        match <_ as $crate::PeanoInt>::PEANO_WIT {
            $crate::PeanoWit::Zero(len_te) => {
                $crate::NList::nil().coerce_len(len_te.flip())
            }
            $crate::PeanoWit::PlusOne(len_te) => {
                $crate::__::destructure!{(elem, next): $ret_ty = $block}

                $crate::NList::cons(elem, next).coerce_len(len_te.flip())
            }
        }
    };
}