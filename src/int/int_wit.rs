use core::fmt::{self, Debug};

use typewit::TypeEq;

use crate::int::{Int, Nat, Zeros};


/// A type witness for whether `L` (a [peano integer](Int)) is [`Zeros`] or [`Nat`]
/// 
/// # Example
/// 
/// Constructing a `&str` or `u8` depending on whether `L` is zero
/// 
/// ```rust
/// use nlist::{Int, IntWit, Peano, peano};
/// use nlist::typewit::{CallFn, TypeEq};
/// 
/// assert_eq!(make::<Peano!(0)>(), "hello");
/// assert_eq!(make::<Peano!(1)>(), 0);
/// assert_eq!(make::<Peano!(2)>(), 1);
/// assert_eq!(make::<Peano!(3)>(), 2);
/// 
/// 
/// // Function which returns different types depending on the value of `L`
/// // 
/// // If L == 0, this returns a &'static str
/// // If L > 0, this returns a usize
/// // 
/// // The `-> CallFn<StrOrUsize, L>` return type calls the `StrOrUsize` type-level function 
/// // with `L` as an argument.
/// const fn make<L: Int>() -> CallFn<StrOrUsize, L> {
///     match L::INT_WIT {
///         // len_te is a proof that `L == Nat<L::ShrOne, L::BitArg>`
///         // len_te: TypeEq<L, Nat<L::ShrOne, L::BitArg>>
///         IntWit::Nat(len_te) => {
///             // te is a proof that `CallFn<StrOrUsize, L> == usize`
///             let te: TypeEq<CallFn<StrOrUsize, L>, usize> = len_te.project::<StrOrUsize>();
///             te.to_left(<L::SubOneSat>::USIZE)
///         }
/// 
///         // len_te is a proof that `L == Zeros`
///         // len_te: TypeEq<L, Zeros>
///         IntWit::Zeros(len_te) => {
///             // te is a proof that `CallFn<StrOrUsize, L> == &'static str`
///             let te: TypeEq<CallFn<StrOrUsize, L>, &'static str> = 
///                 len_te.project::<StrOrUsize>();
///
///             te.to_left("hello")
///         }
///     }
/// }
/// 
/// // StrOrUsize is a type-level function (`typewit::TypeFn` implementor),
/// // which takes a Int parameter.
/// // 
/// // In pseudocode, this is what it does on the type level:
/// // fn StrOrUsize(L: Int) -> type {
/// //      if L == 0 { &'static str } else { usize }  
/// // }
/// type StrOrUsize = peano::IfZerosAltFn<&'static str, usize>;
/// 
/// ```
pub enum IntWit<L: Int> {
    /// Proof that `L == Nat<L::ShrOne, L::BitArg>`
    Nat(TypeEq<L, Nat<L::ShrOne, L::BitArg>>),
    /// Proof that `L == Zeros`
    Zeros(TypeEq<L, Zeros>),
}



impl<L: Int> Debug for IntWit<L> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple("IntWit").field(&L::USIZE).finish()        
    }
}

impl<L: Int> Copy for IntWit<L> {}

impl<L: Int> Clone for IntWit<L> {
    fn clone(&self) -> Self {
        *self
    }
}

