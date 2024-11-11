/// Constructs an `NList` from a list of elements
///
/// # Example
///
/// ```rust
/// use nlist::NList;
///
/// let list: NList<u32, _> = nlist::nlist![3, 5, 8, 13];
///
/// assert_eq!(list.into_vec(), vec![3, 5, 8, 13]);
///
/// ```
///
#[macro_export]
macro_rules! nlist {
    ($($expr:expr),* $(,)?) => {
        $crate::__nlist!{$($expr)*}
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __nlist {
    ($first:tt $($rest:tt)*) => {
        $crate::NList::cons($first, $crate::__nlist!($($rest)*))
    };
    () => {
        $crate::NList::nil()
    };
}
