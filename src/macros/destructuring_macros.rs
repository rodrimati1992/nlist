/// Macro for using NList in patterns.
/// 
/// This macro uses the same syntax as array patterns, with the limitation that 
/// it only supports `..` patterns at the end.
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
/// use nlist::{NList, PeanoInt, PlusOne, nlist, nlist_pat};
/// 
/// assert!(starts_with_zero(&nlist![0, 1, 2]));
/// assert!(!starts_with_zero(&nlist![1]));
/// assert!(!starts_with_zero(&nlist![1, 2, 3]));
/// 
/// const fn starts_with_zero<L: PeanoInt>(nlist: &NList<u32, PlusOne<L>>) -> bool {
///     matches!(nlist, nlist_pat![0, ..])
/// }
/// ```
#[macro_export]
macro_rules! nlist_pat {
    ($($patterns:tt)*) => ( 
        $crate::__nlist_pat!(
            ($crate::__nlist_count_elems!(($crate::Zero) $($patterns)*)) 
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
    (($($len:tt)*) $($($($binding:ident)+)? $(_)? @)? .. $(,)?) => (
        _
    );
    (($($len:tt)*) $pati:pat $(, $($rest:tt)*)?) => (
        $crate::__nlist_count_elems!{($crate::PlusOne<$($len)*>) $($($rest)*)?}
    );
    (($($len:tt)*) $(,)?) => (
        $($len)*
    );
}


