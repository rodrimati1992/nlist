use core::fmt::{self, Debug};

use typewit::TypeEq;

use crate::peano::{PeanoInt, PlusOne, Zero};


/// A type witness for whether `L` (a [peano integer](PeanoInt)) is [`Zero`] or [`PlusOne`]
/// 
/// # Example
/// 
/// Constructing a `&str` or `u8` depending on whether `L` is zero
/// 
/// ```rust
/// use nlist::{PeanoInt, PeanoWit, Peano, peano};
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
/// const fn make<L: PeanoInt>() -> CallFn<StrOrUsize, L> {
///     match L::PEANO_WIT {
///         // len_te is a proof that `L == PlusOne<L::SubOneSat>`
///         // len_te: TypeEq<L, PlusOne<L::SubOneSat>>
///         PeanoWit::PlusOne(len_te) => {
///             // te is a proof that `CallFn<StrOrUsize, L> == usize`
///             let te: TypeEq<CallFn<StrOrUsize, L>, usize> = len_te.project::<StrOrUsize>();
///             te.to_left(<L::SubOneSat>::USIZE)
///         }
/// 
///         // len_te is a proof that `L == Zero`
///         // len_te: TypeEq<L, Zero>
///         PeanoWit::Zero(len_te) => {
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
/// // which takes a PeanoInt parameter.
/// // 
/// // In pseudocode, this is what it does on the type level:
/// // fn StrOrUsize(L: PeanoInt) -> type {
/// //      if L == 0 { &'static str } else { usize }  
/// // }
/// type StrOrUsize = peano::IfZeroAltFn<&'static str, usize>;
/// 
/// ```
pub enum PeanoWit<L: PeanoInt> {
    /// Proof that `L == PlusOne<L::SubOneSat>`
    PlusOne(TypeEq<L, PlusOne<L::SubOneSat>>),
    /// Proof that `L == Zero`
    Zero(TypeEq<L, Zero>),
}



impl<L: PeanoInt> Debug for PeanoWit<L> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple("PeanoWit").field(&L::USIZE).finish()        
    }
}

impl<L: PeanoInt> Copy for PeanoWit<L> {}

impl<L: PeanoInt> Clone for PeanoWit<L> {
    fn clone(&self) -> Self {
        *self
    }
}

