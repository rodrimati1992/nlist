/// Constructs an [`NList`](crate::NList) from a list of elements
///
/// # Example
///
/// ```rust
/// use nlist::peano;
/// use nlist::NList;
///
/// let list: NList<u32, peano!(4)> = nlist::nlist![3, 5, 8, 13];
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


///////////////////////////////////


/// Converts an integer constant to a [peano integer](crate::PeanoInt)
/// 
/// This macro is just sugar for the [`FromUsize`] type alias
/// 
/// [`FromUsize`]: crate::peano::FromUsize
#[macro_export]
macro_rules! peano {
    ($expr:expr) => {
        $crate::peano::FromUsize<$expr>
    }
}
