pub(crate) mod internal_macros;
mod destructuring_macros;
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
/// # List each element
///
/// ```rust
/// use nlist::{NList, Int};
///
/// const LIST: NList<u32, Int!(4)> = nlist::nlist![3, 5, 8, 13];
///
/// assert_eq!(LIST.into_vec(), vec![3, 5, 8, 13]);
///
/// ```
///
/// # Repeat elements
///
/// Repeating a [`Copy`] value to construct an [`NList`]
///
/// ```rust
/// use nlist::{NList, Int};
///
/// // Inferring the length 
/// let list_a: NList<u8, Int!(2)> = nlist::nlist![5; _];
/// assert_eq!(list_a.into_array(), [5, 5]);
///
/// // Passing the length explicitly
/// let list_b: NList<&str, Int!(3)> = nlist::nlist!["heh"; 3];
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
        $crate::NList::<_, $crate::Int!($len)>::repeat_copy($expr)
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

/// Converts an integer constant to a [int integer](crate::Int)
///
/// This macro is sugar for `<`[`FromUsize`]` as `[`Int`]`>::NEW`
///
/// # Example
///
/// ```rust
/// use nlist::{NList, nlist, Int, int};
///
/// let val_0: Int!(0) = int!(0);
/// let val_1: Int!(1) = int!(1);
/// let val_2: Int!(2) = int!(2);
/// let val_3: Int!(3) = int!(3);
/// ```
///
/// [`FromUsize`]: crate::int::FromUsize
/// [`Int`]: crate::Int
#[macro_export]
macro_rules! int {
    ($expr:expr) => {
        <$crate::int::FromUsize<$expr> as $crate::Int>::NEW
    }
}

/// Converts an integer constant to a [int integer](crate::Int)
///
/// This macro is just sugar for the [`FromUsize`] type alias
///
/// # Example
///
/// ```rust
/// use nlist::{NList, nlist, Int};
///
/// let list_0: NList<u8, Int!(0)> = nlist![];
/// let list_1: NList<u8, Int!(1)> = nlist![3];
/// let list_2: NList<u8, Int!(2)> = nlist![3, 5];
/// let list_3: NList<u8, Int!(3)> = nlist![3, 5, 8];
/// ```
///
/// [`FromUsize`]: crate::int::FromUsize
#[macro_export]
macro_rules! Int {
    ($expr:expr) => {
        $crate::int::FromUsize<{$expr}>
    }
}
