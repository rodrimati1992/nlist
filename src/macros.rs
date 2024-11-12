/// Constructs an [`NList`](crate::NList) 
///
/// This macro can be used in two ways:
/// - `nlist![a, b, c]`: creates an NList with the listed elements
/// - `nlist![val; LEN]`: creates an NList by repeating a [`Copy`] value `LEN` times.
/// (`LEN` must be either a usize expression or `_`)
///
/// # Example
///
/// # list of elements
///
/// ```rust
/// use nlist::{NList, peano};
///
/// let list: NList<u32, peano!(4)> = nlist::nlist![3, 5, 8, 13];
///
/// assert_eq!(list.into_vec(), vec![3, 5, 8, 13]);
///
/// ```
///
/// # repeat elements
///
/// Repeating a [`Copy`] value to construct an [`NList`]
///
/// ```rust
/// use nlist::{NList, peano};
///
/// // Inferring the length 
/// let list_a: NList<u8, peano!(2)> = nlist::nlist![5; _];
/// assert_eq!(list_a.into_array(), [5, 5]);
///
/// // Passing the length explicitly
/// let list_b: NList<&str, peano!(3)> = nlist::nlist!["heh"; 3];
/// assert_eq!(list_b.into_array(), ["heh", "heh", "heh"]);
///
/// ```
///
/// [`Copy`]: core::marker::Copy
/// [`NList`]: crate::NList
#[macro_export]
macro_rules! nlist {
    ($expr:expr; _) => {
        $crate::NList::repeat_copy($expr)
    };
    ($expr:expr; $len:expr) => {
        $crate::NList::<_, $crate::peano!($len)>::repeat_copy($expr)
    };
    ($($expr:expr),* $(,)?) => {
        $crate::__nlist!{$($expr)*}
    };
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
/// # Example
///
/// ```rust
/// use nlist::{NList, nlist, peano};
///
/// let list_0: NList<u8, peano!(0)> = nlist![];
/// let list_1: NList<u8, peano!(1)> = nlist![3];
/// let list_2: NList<u8, peano!(2)> = nlist![3, 5];
/// let list_3: NList<u8, peano!(3)> = nlist![3, 5, 8];
/// ```
///
/// [`FromUsize`]: crate::peano::FromUsize
#[macro_export]
macro_rules! peano {
    ($expr:expr) => {
        $crate::peano::FromUsize<$expr>
    }
}
