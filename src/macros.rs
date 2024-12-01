pub(crate) mod internal_macros;
mod from_fn_macro;
mod iterator_macros;


///////////////////////////////////

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
/// use nlist::{NList, Peano};
///
/// const LIST: NList<u32, Peano!(4)> = nlist::nlist![3, 5, 8, 13];
///
/// assert_eq!(LIST.into_vec(), vec![3, 5, 8, 13]);
///
/// ```
///
/// # repeat elements
///
/// Repeating a [`Copy`] value to construct an [`NList`]
///
/// ```rust
/// use nlist::{NList, Peano};
///
/// // Inferring the length 
/// let list_a: NList<u8, Peano!(2)> = nlist::nlist![5; _];
/// assert_eq!(list_a.into_array(), [5, 5]);
///
/// // Passing the length explicitly
/// let list_b: NList<&str, Peano!(3)> = nlist::nlist!["heh"; 3];
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
        $crate::NList::<_, $crate::Peano!($len)>::repeat_copy($expr)
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
/// This macro is sugar for `<`[`FromUsize`]` as `[`PeanoInt`]`>::NEW`
///
/// # Example
///
/// ```rust
/// use nlist::{NList, nlist, Peano, peano};
///
/// let val_0: Peano!(0) = peano!(0);
/// let val_1: Peano!(1) = peano!(1);
/// let val_2: Peano!(2) = peano!(2);
/// let val_3: Peano!(3) = peano!(3);
/// ```
///
/// [`FromUsize`]: crate::peano::FromUsize
/// [`PeanoInt`]: crate::PeanoInt
#[macro_export]
macro_rules! peano {
    ($expr:expr) => {
        <$crate::peano::FromUsize<$expr> as $crate::PeanoInt>::NEW
    }
}

/// Converts an integer constant to a [peano integer](crate::PeanoInt)
///
/// This macro is just sugar for the [`FromUsize`] type alias
///
/// # Example
///
/// ```rust
/// use nlist::{NList, nlist, Peano};
///
/// let list_0: NList<u8, Peano!(0)> = nlist![];
/// let list_1: NList<u8, Peano!(1)> = nlist![3];
/// let list_2: NList<u8, Peano!(2)> = nlist![3, 5];
/// let list_3: NList<u8, Peano!(3)> = nlist![3, 5, 8];
/// ```
///
/// [`FromUsize`]: crate::peano::FromUsize
#[macro_export]
macro_rules! Peano {
    ($expr:expr) => {
        $crate::peano::FromUsize<{$expr}>
    }
}
