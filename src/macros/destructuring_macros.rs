/// Macro for using [`NList`](crate::NList) in patterns.
/// 
/// This macro uses the same syntax as array patterns, with the limitation that 
/// it only supports `..` patterns at the end.
/// 
/// # Alternatives
/// 
/// The [`unlist`](crate::unlist) macro allows destructuring [`NList`] 
/// by value in some contexts where this macro can't be used,
/// refer to its docs for more details.
/// 
/// # Example
/// 
/// ### Destructuring
/// 
/// ```rust
/// use nlist::{nlist, nlist_pat};
/// 
/// // destructuring by value
/// {
///     let nlist_pat![a, b, c @ ..] = nlist![3, 5, 8, 13, 21];
///     
///     assert_eq!(a, 3);
///     assert_eq!(b, 5);
///     assert_eq!(c, nlist![8, 13, 21]);
/// }
/// 
/// // destructuring by reference
/// {
///     let nlist_pat![a, b, c @ ..] = &nlist![3, 5, 8, 13, 21];
///     
///     assert_eq!(a, &3);
///     assert_eq!(b, &5);
///     assert_eq!(c, &nlist![8, 13, 21]);
/// }
/// 
/// // destructuring by mutable reference
/// {
///     let nlist_pat![a, b, c @ ..] = &mut nlist![3, 5, 8, 13, 21];
///     
///     assert_eq!(a, &mut 3);
///     assert_eq!(b, &mut 5);
///     assert_eq!(c, &mut nlist![8, 13, 21]);
/// }
/// ```
/// 
/// ### Pattern matching
/// 
/// ```rust
/// use nlist::{NList, Int, Nat, nlist, nlist_pat};
/// 
/// assert!(starts_with_zero(&nlist![0, 1, 2]));
/// assert!(!starts_with_zero(&nlist![1]));
/// assert!(!starts_with_zero(&nlist![1, 2, 3]));
/// 
/// const fn starts_with_zero<L: Int>(nlist: &NList<u32, Nat<L>>) -> bool {
///     matches!(nlist, nlist_pat![0, ..])
/// }
/// ```
/// 
/// [`NList`]: crate::NList
#[macro_export]
macro_rules! nlist_pat {
    ($($patterns:tt)*) => ( 
        $crate::__nlist_pat!(
            ($crate::__nlist_count_elems!(($crate::Zeros) $($patterns)*)) 
            $($patterns)*
        )
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __nlist_pat {
    (($($len:ty)?) .., $($rest:tt)+) => (
        $crate::__::compile_error!{
            "nlist_pat only supports `..` patterns at the end"
        }
    );
    (($($len:ty)?) $($($($binding:ident)+)? $(_)? @)? .. $(,)?) => (
        $crate::__first_pat!($($($($binding)+,)?)? _,) 
    );
    (($($len:ty)?) $pati:pat $(, $($rest:tt)*)?) => (
        $crate::NList $(::<_, $len>)? {
            node: $crate::Cons {
                elem: $pati,
                next: $crate::__nlist_pat!{() $($($rest)*)?},
                ..
            }
        }
    );
    (($($len:ty)?) $(,)?) => (
        $crate::NList $(::<_, $len>)? { node: $crate::Nil{..} }
    );
}


#[doc(hidden)]
#[macro_export]
macro_rules! __nlist_count_elems {
    (($($len:tt)*) $($($($binding:ident)+)? $(_)? @)? .. $(, $($rem:tt)*)?) => (
        _
    );
    (($($len:tt)*) $pati:pat $(, $($rest:tt)*)?) => (
        $crate::__nlist_count_elems!{($crate::Nat<$($len)*>) $($($rest)*)?}
    );
    (($($len:tt)*) $(,)?) => (
        $($len)*
    );
}

/// Destructures an [`NList`](crate::NList) by value into its elements
/// 
/// This macro uses the same syntax as array patterns, with the limitation that 
/// it only supports `..` patterns at the end.
/// 
/// # Motivation
/// 
/// This macro exists because, as of Rust 1.83, 
/// destructuring generic `NList`s by value in const fns with [`nlist_pat`]
/// causes "destructor cannot be evaluated in const fn" errors.
/// 
/// Note: destructuring into nested non-Copy fields isn't supported in 
/// the aforementioned generic const fn context, 
/// you'll need to bind the NList element to a variable,
/// then destructure the variable with [`konst::destructure`](konst::destructure).
/// 
/// # Example
/// 
/// ```rust
/// use nlist::{NList, Int, Peano, nlist, peano, unlist};
/// 
/// assert_eq!(multisplit(nlist![3, 5]), (3, 5, nlist![]));
/// assert_eq!(multisplit(nlist![8, 13, 21]), (8, 13, nlist![21]));
/// assert_eq!(multisplit(nlist![34, 55, 89, 144]), (34, 55, nlist![89, 144]));
/// 
/// const fn multisplit<T, L>(list: NList<T, peano::Add<Peano!(2), L>>) -> (T, T, NList<T, L>)
/// where
///     L: Int
/// {
///     unlist!{[a, b, c @ ..] = list}
///     
///     (a, b, c)
/// }
/// ```
/// If the `unlist!{...}` line is replaced with
/// ```rust,compile_fail
/// # use nlist::{NList, Int, Peano, nlist, nlist_pat, peano, unlist};
/// # const fn multisplit<T, L>(list: NList<T, peano::Add<Peano!(2), L>>) -> (T, T, NList<T, L>)
/// # where
/// #   L: Int
/// # {
///     let nlist_pat![a, b, c @ ..] = list;
/// #   (a, b, c)
/// # }
/// ```
/// it causes this compilation error as of Rust 1.83.0:
/// ```text
/// error[E0493]: destructor of `NList<T, nlist::Nat<nlist::Nat<L>>>` cannot be evaluated at compile-time
///   --> src/macros/destructuring_macros.rs:134:27
///    |
/// 6  | const fn multisplit<T, L>(list: NList<T, peano::Add<Peano!(2), L>>) -> (T, T, NList<T, L>)
///    |                           ^^^^ the destructor for this type cannot be evaluated in constant functions
/// ...
/// 12 | }
///    | - value is dropped here
/// 
/// ```
#[macro_export]
macro_rules! unlist {
    ([$($patterns:tt)*] = $val:expr $(;)?) => ( 
        let list: $crate::NList<_, $crate::__nlist_count_elems!(($crate::Zeros) $($patterns)*)> = 
            $val;

        $crate::__unlist!((list) $($patterns)*)
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __unlist {
    (($list:ident) .., $($rest:tt)+) => (
        $crate::__::compile_error!{
            "nlist_pat only supports `..` patterns at the end"
        }
    );
    (($list:ident) $($($($binding:ident)+)? $(_)? @)? .. $(,)?) => (
        let $crate::__first_pat!($($($($binding)+,)?)? _,) = $list;
    );
    (($list:ident) $pati:pat $(, $($rest:tt)*)?) => (
        $crate::__::destructure!{($pati, $list) = $list.into_split_head()}

        $crate::__unlist!{($list) $($($rest)*)?}
    );
    (($list:ident) $(,)?) => ();
}
