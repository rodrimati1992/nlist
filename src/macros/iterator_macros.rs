

macro_rules! receiver_map_explanation {() => {"
The closure parameters depend on the value of `P`:
- If `P == NList<T, L>`: the parameters are `(T, Nlist<T, L::SubOneSat>)`
- If `P == &NList<T, L>`: the parameters are `(&T, &Nlist<T, L::SubOneSat>)`
- If `P == &mut NList<T, L>`: the parameters are `(&mut T, &mut Nlist<T, L::SubOneSat>)`
"}}

use receiver_map_explanation;


/// Helper for writing const fn equivalents of [`NList::map`](crate::NList::map)
/// 
/// This macro acts like a function with this signature:
/// 
/// ```rust
/// use nlist::{NList, PeanoInt};
///
/// use nlist::receiver::Receiver;
/// # use nlist::receiver::HktApply;
/// 
/// fn rec_map<'a, P, T, L, F, U, Hkt>(list: P, func: F) -> NList<U, L>
/// where
///     P: Receiver<'a, NList<T, L>>,
///     L: PeanoInt,
///     T: 'a,
///     F: Fn(
///         # /*
///         ... // parameter types explained below
///         # */
///         # HktApply<'a, P::Hkt, T>, 
///         # HktApply<'a, P::Hkt, NList<T, L::SubOneSat>>
///     ) -> (U, NList<U, L::SubOneSat>)
/// # { nlist::rec_map!{ list, |a, b| { func(a, b) }} }
/// ```
/// 
#[doc = receiver_map_explanation!()]
/// 
/// # Example
/// 
/// ### By value
/// 
/// Example that takes the `NList` by value
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt, nlist};
/// 
/// const LIST: NList<u128, Peano!(3)> = double(nlist![3, 5, 8]);
/// 
/// assert_eq!(LIST, nlist![6, 10, 16]);
/// 
/// const fn double<L>(list: NList<u128, L>) -> NList<u128, L>
/// where
///     L: PeanoInt
/// {
///     nlist::rec_map!{list, |elem: u128, next| (elem * 2, double(next))}
/// }
/// ```
/// 
/// ### By reference
/// 
/// Example that takes the `NList` by reference
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt, nlist};
/// 
/// const LIST: NList<u128, Peano!(3)> = double(&nlist![3, 5, 8]);
/// 
/// assert_eq!(LIST, nlist![6, 10, 16]);
/// 
/// const fn double<L>(list: &NList<u128, L>) -> NList<u128, L>
/// where
///     L: PeanoInt
/// {
///     nlist::rec_map!{list, |elem: &u128, next| (*elem * 2, double(next))}
/// }
/// ```
#[macro_export]
macro_rules! rec_map {
    ($in_list:expr, $($func:tt)*) => {
        $crate::__parse_closure_2_args!{__rec_map ($in_list,) $($func)*}
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __rec_map {
    (
        $in_list:expr, 

        |$elem:tt: $elem_ty:ty, $next:tt: $next_ty:ty| -> $ret_ty:ty $block:block
    ) => {
        match $in_list {in_list => {
            match $crate::NList::len_proof($crate::receiver::as_ref(&in_list)) {
                $crate::PeanoWit::Zero(len_te) => {
                    // works around "destructor cannot be evaluated at compile-time" error
                    _ = $crate::NList::coerce_len_poly(in_list, len_te);
                    
                    $crate::NList::nil().coerce_len(len_te.flip())
                },
                $crate::PeanoWit::PlusOne(len_te) => {
                    $crate::__::destructure!{
                        ($elem, $next) = $crate::NList::split_head_poly(
                            $crate::NList::coerce_len_poly(in_list, len_te)
                        )
                    }

                    // asserting the type here because 
                    // match ergonomics would make them different in the destructure macro.
                    let _: $elem_ty = $elem;
                    let _: $next_ty = $next;


                    $crate::__::destructure!{(elem, next): $ret_ty = $block}

                    $crate::NList::cons(elem, next).coerce_len(len_te.flip())
                },
            }
        }}
    }
}


// - all
// - any
// - find
// - find_map
// - fold
// - for_each
// - position
// - rfind
// - rfind_map
// - rposition

