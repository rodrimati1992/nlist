

macro_rules! fn_equivalent {(
    fn_name = $fn_name:literal
    fn_ret  = $fn_ret:literal
    closure_ret = $closure_ret:literal
    $(additional_generics=$additional_generics:literal)?
    $(additional_params=$additional_params:literal)?
    $(additional_args=$additional_args:literal)?
) => {concat!("
This macro acts like a function with this signature:

```rust
use nlist::{NList, PeanoInt};

use nlist::receiver::Receiver;
# use nlist::receiver::HktApply;

fn ", $fn_name,"<'a, P, T, L, F",
    $(", ", $additional_generics,)?
">(list: P ", $(", ", $additional_params,)? ", func: F) -> ", $fn_ret,"
where
    P: Receiver<'a, NList<T, L>>,
    L: PeanoInt,
    T: 'a,
    F: Fn(
        # /*
        ... // parameter types explained below
        # */
        # HktApply<'a, P::Hkt, T>, 
        # HktApply<'a, P::Hkt, NList<T, L::SubOneSat>>
    ) -> ", $closure_ret, "
# { nlist::", $fn_name,"!{ list ", $(", ", $additional_args,)? ", |a, b| { func(a, b) }} }
```

The closure parameters depend on the value of `P`:
- If `P == NList<T, L>`: the parameters are `(T, Nlist<T, L::SubOneSat>)`
- If `P == &NList<T, L>`: the parameters are `(&T, &Nlist<T, L::SubOneSat>)`
- If `P == &mut NList<T, L>`: the parameters are `(&mut T, &mut Nlist<T, L::SubOneSat>)`
")}}

use fn_equivalent;


macro_rules! early_termination_warning {() => {"

# Note

Because iteration over the list might terminate before the list is fully consumed,
by-value iteration over non-Copy types does not work in const,
and by-value iteration over Copy types requires doing what the by-value example does.
"}}

use early_termination_warning;


//////////////////////////////////////////////////////////////////////////////

/// Helper for writing const fn equivalents of [`NList::all`](crate::NList::all)
///
#[doc = fn_equivalent!(
    fn_name = "rec_all"
    fn_ret  = "bool"
    closure_ret = "bool"
)]
#[doc = early_termination_warning!()]
/// 
/// # Example
/// 
/// ### By reference
/// 
/// Example that takes an `NList` by reference
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt, nlist};
/// 
/// const ALL_EVEN: bool = all_even(&nlist![3, 5, 8]);
/// 
/// assert!(!ALL_EVEN);
/// 
/// const fn all_even<L>(list: &NList<u128, L>) -> bool
/// where
///     L: PeanoInt
/// {
///     nlist::rec_all!{list, |elem: &u128, next| *elem % 2 == 0 && all_even(next)}
/// }
/// ```
/// 
/// ### By value
/// 
/// Example that takes an `NList` of `Copy` elements by value
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt, nlist};
/// 
/// use std::mem::ManuallyDrop as MD;
/// 
/// 
/// const ALL_EMPTY_ARE_ODD: bool = all_odd(nlist![]);
/// assert!(ALL_EMPTY_ARE_ODD);
/// 
/// const ALL_ODD: bool = all_odd(nlist![3, 5, 13]);
/// assert!(ALL_ODD);
/// 
/// const fn all_odd<L>(list: NList<u128, L>) -> bool
/// where
///     L: PeanoInt
/// {
///     nlist::rec_all!{list, |elem: u128, next| {
///         // works around "destructor cannot be evaluated at compile-time" error
///         let next = next.assert_copy();
///
///         elem % 2 == 1 && all_odd(MD::into_inner(next))
///     }}
///
/// }
/// ```
/// 
#[macro_export]
macro_rules! rec_all {
    ($in_list:expr, $($func:tt)*) => {
        $crate::__parse_closure_2_args!{__rec_all ($in_list,) $($func)*}
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __rec_all {
    (
        $in_list:expr, 

        |$elem:tt: $elem_ty:ty, $next:tt: $next_ty:ty| -> $ret_ty:ty $block:block
    ) => {
        $crate::__rec_shared!{
            $in_list,
            len_te,
            || true,
            |$elem: $elem_ty, $next: $next_ty| {
                let ret: $ret_ty = $block;
                let _: $crate::__::bool = ret;

                ret
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

/// Helper for writing const fn equivalents of [`NList::any`](crate::NList::any)
/// 
#[doc = fn_equivalent!(
    fn_name = "rec_any"
    fn_ret  = "bool"
    closure_ret = "bool"
)]
#[doc = early_termination_warning!()]
/// 
/// # Example
/// 
/// ### By reference
/// 
/// Example that takes an `NList` by reference
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt, nlist};
/// 
/// const EVEN_IN_EMPTY: bool = any_even(&nlist![]);
/// assert!(!EVEN_IN_EMPTY);
///
/// const ANY_EVEN: bool = any_even(&nlist![3, 5, 9]);
/// assert!(!ANY_EVEN);
/// 
/// 
/// const fn any_even<L>(list: &NList<u128, L>) -> bool
/// where
///     L: PeanoInt
/// {
///     nlist::rec_any!{list, |elem: &u128, next| *elem % 2 == 0 || any_even(next)}
/// }
/// ```
/// 
/// ### By value
/// 
/// Example that takes an `NList` of `Copy` elements by value
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt, nlist};
/// 
/// use std::mem::ManuallyDrop as MD;
/// 
/// 
/// const ANY_ODD: bool = any_odd(nlist![8, 13, 24]);
/// assert!(ANY_ODD);
/// 
/// const fn any_odd<L>(list: NList<u128, L>) -> bool
/// where
///     L: PeanoInt
/// {
///     nlist::rec_any!{list, |elem: u128, next| {
///         // works around "destructor cannot be evaluated at compile-time" error
///         let next = next.assert_copy();
///
///         elem % 2 == 1 || any_odd(MD::into_inner(next))
///     }}
///
/// }
/// ```
/// 
#[macro_export]
macro_rules! rec_any {
    ($in_list:expr, $($func:tt)*) => {
        $crate::__parse_closure_2_args!{__rec_any ($in_list,) $($func)*}
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __rec_any {
    (
        $in_list:expr, 

        |$elem:tt: $elem_ty:ty, $next:tt: $next_ty:ty| -> $ret_ty:ty $block:block
    ) => {
        $crate::__rec_shared!{
            $in_list,
            len_te,
            || false,
            |$elem: $elem_ty, $next: $next_ty| {
                let ret: $ret_ty = $block;
                let _: $crate::__::bool = ret;

                ret
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

/// Helper for writing const fn equivalents of [`NList::find_map`](crate::NList::find_map)
/// 
#[doc = fn_equivalent!(
    fn_name = "rec_find_map"
    fn_ret  = "Option<U>"
    closure_ret = "Option<U>"
    additional_generics="U"
)]
#[doc = early_termination_warning!()]
/// 
/// # Example
/// 
/// ### By reference
/// 
/// Example that takes an `NList` by reference
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt, nlist};
/// 
/// const EMPTY: Option<i128> = finder(&nlist![]);
/// assert!(EMPTY.is_none());
///
/// const FOUND: Option<i128> = finder(&nlist![3, 5, 9]);
/// assert_eq!(FOUND, Some(1));
/// 
/// 
/// const fn finder<L>(list: &NList<u128, L>) -> Option<i128>
/// where
///     L: PeanoInt
/// {
///     nlist::rec_find_map!{list, |elem: &u128, next| {
///         if *elem % 4 == 1 {
///             Some((*elem / 4) as i128)
///         } else {
///             finder(next)
///         }
///     }}
/// }
/// ```
/// 
/// ### By value
/// 
/// Example that takes an `NList` of `Copy` elements by value
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt, nlist};
/// 
/// use std::mem::ManuallyDrop as MD;
/// 
/// const EMPTY: Option<u8> = finder(nlist![]);
/// assert!(EMPTY.is_none());
///
/// const FOUND: Option<u8> = finder(nlist![3, 5, 9]);
/// assert_eq!(FOUND, Some(1));
/// 
/// 
/// const fn finder<L>(list: NList<u128, L>) -> Option<u8>
/// where
///     L: PeanoInt
/// {
///     nlist::rec_find_map!{list, |elem: u128, next| {
///         // works around "destructor cannot be evaluated at compile-time" error
///         let next = next.assert_copy();
///
///         if elem % 4 == 1 {
///             Some((elem % 4) as u8)
///         } else {
///             finder(MD::into_inner(next))
///         }
///     }}
/// }
/// ```
/// 
/// 
#[macro_export]
macro_rules! rec_find_map {
    ($in_list:expr, $($func:tt)*) => {
        $crate::__parse_closure_2_args!{__rec_find_map ($in_list,) $($func)*}
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __rec_find_map {
    (
        $in_list:expr, 

        |$elem:tt: $elem_ty:ty, $next:tt: $next_ty:ty| -> $ret_ty:ty $block:block
    ) => {
        $crate::__rec_shared!{
            $in_list,
            len_te,
            || None,
            |$elem: $elem_ty, $next: $next_ty| {
                let ret: $ret_ty = $block;
                let _: Option<_> = ret;

                ret
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

/// Helper for writing const fn equivalents of [`NList::map`](crate::NList::map)
/// 
#[doc = fn_equivalent!(
    fn_name = "rec_map"
    fn_ret  = "NList<U, L>"
    closure_ret = "(U, NList<U, L::SubOneSat>)"
    additional_generics="U"
)]
/// 
/// # Example
/// 
/// ### By value
/// 
/// Example that takes an `NList` by value
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
/// Example that takes an `NList` by reference
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
        $crate::__rec_shared!{
            $in_list,
            len_te,
            || $crate::NList::nil().coerce_len(len_te.flip()),
            |$elem: $elem_ty, $next: $next_ty| {
                $crate::__::destructure!{(elem, next): $ret_ty = $block}

                $crate::NList::cons(elem, next).coerce_len(len_te.flip())
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

/// Helper for writing const fn equivalents of [`NList::for_each`](crate::NList::for_each)
/// 
#[doc = fn_equivalent!(
    fn_name = "rec_for_each"
    fn_ret  = "()"
    closure_ret = "()"
)]
/// 
/// # Example
/// 
/// ### By value
/// 
/// Example that takes an `NList` by value
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt, nlist};
/// 
/// const SUM: u128 = {
///     let mut sum = 0;
///     add_to(&mut sum, nlist![3, 5, 8]);
///     sum
/// };
/// 
/// assert_eq!(SUM, 16);
/// 
/// const fn add_to<L>(mutator: &mut u128, list: NList<u128, L>)
/// where
///     L: PeanoInt
/// {
///     nlist::rec_for_each!{list, |elem: u128, next| { 
///         *mutator += elem;
///         add_to(mutator, next)
///     }}
/// }
/// ```
/// 
/// ### By reference
/// 
/// Example that takes an `NList` by reference
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt, nlist};
/// 
/// const SUM: u128 = {
///     let mut sum = 0;
///     add_to(&mut sum, &nlist![3, 5, 8]);
///     sum
/// };
/// 
/// assert_eq!(SUM, 16);
/// 
/// const fn add_to<L>(mutator: &mut u128, list: &NList<u128, L>)
/// where
///     L: PeanoInt
/// {
///     nlist::rec_for_each!{list, |elem: &u128, next| { 
///         *mutator += *elem;
///         add_to(mutator, next)
///     }}
/// }
/// ```
#[macro_export]
macro_rules! rec_for_each {
    ($in_list:expr, $($func:tt)*) => {
        $crate::__parse_closure_2_args!{__rec_for_each ($in_list,) $($func)*}
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __rec_for_each {
    (
        $in_list:expr, 

        |$elem:tt: $elem_ty:ty, $next:tt: $next_ty:ty| -> $ret_ty:ty $block:block
    ) => {
        $crate::__rec_shared!{
            $in_list,
            len_te,
            || (),
            |$elem: $elem_ty, $next: $next_ty| {
                let ret: $ret_ty = $block;
                let _: () = ret;
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

/// Helper for writing const fn equivalents of [`NList::fold`](crate::NList::fold)
/// 
#[doc = fn_equivalent!(
    fn_name = "rec_fold"
    fn_ret  = "U"
    closure_ret = "U"
    additional_generics="U"
    additional_params="accum: U"
    additional_args="accum"
)]
/// 
/// # Accumulator Argument
/// 
/// The `accum` argument is only evaluated if the list is empty,
/// which means that you can pass it by value and still use it in the passed in closure.
/// 
/// # Example
/// 
/// ### By value
/// 
/// Example that takes an `NList` by value
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt, nlist};
/// 
/// const SUM: u128 = add_up(0, nlist![1, 2, 3, 4, 5]);
/// 
/// assert_eq!(SUM, 15);
/// 
/// const fn add_up<L>(sum: u128, list: NList<u128, L>) -> u128
/// where
///     L: PeanoInt
/// {
///     nlist::rec_fold!{list, sum, |elem: u128, next| add_up(sum + elem, next)}
/// }
/// ```
/// 
/// ### By reference
/// 
/// Example that takes an `NList` by reference
/// 
/// ```rust
/// use nlist::{NList, Peano, PeanoInt, nlist};
/// 
/// const SUM: u128 = add_up(0, &nlist![3, 5, 8]);
/// 
/// assert_eq!(SUM, 16);
/// 
/// const fn add_up<L>(sum: u128, list: &NList<u128, L>) -> u128
/// where
///     L: PeanoInt
/// {
///     nlist::rec_fold!{list, sum, |elem: &u128, next| add_up(sum + *elem, next)}
/// }
/// ```
#[macro_export]
macro_rules! rec_fold {
    ($in_list:expr, $accum:expr, $($func:tt)*) => {
        $crate::__parse_closure_2_args!{__rec_fold ($in_list, $accum,) $($func)*}
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __rec_fold {
    (
        $in_list:expr, 
        $accum:expr,

        |$elem:tt: $elem_ty:ty, $next:tt: $next_ty:ty| -> $ret_ty:ty $block:block
    ) => {
        $crate::__rec_shared!{
            $in_list,
            len_te,
            || $accum,
            |$elem: $elem_ty, $next: $next_ty| {
                let ret: $ret_ty = $block;
                ret
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! __rec_shared {
    (
        $in_list:expr, 
        $len_te:ident,

        || $on_nil:expr,
        |$elem:tt: $elem_ty:ty, $next:tt: $next_ty:ty| $on_cons:block
    ) => {
        match $in_list {in_list => {
            match $crate::NList::len_proof($crate::receiver::as_ref(&in_list)) {
                $crate::PeanoWit::Zero($len_te) => {
                    // works around "destructor cannot be evaluated at compile-time" error
                    _ = $crate::NList::coerce_len_poly(in_list, $len_te);
                    
                    $on_nil
                }
                $crate::PeanoWit::PlusOne($len_te) => {
                    $crate::__::destructure!{
                        ($elem, $next) = $crate::NList::split_head_poly(
                            $crate::NList::coerce_len_poly(in_list, $len_te)
                        )
                    }

                    // asserting the type here because 
                    // match ergonomics would make them different in the destructure macro.
                    let _: $elem_ty = $elem;
                    let _: $next_ty = $next;

                    $on_cons
                }
            }
        }}
    }
}