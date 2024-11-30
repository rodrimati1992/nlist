//! Traits and operations for type-level booleans

use crate::PeanoInt;


#[doc(no_inline)]
pub use typewit::const_marker::{Bool, BoolWit, BoolWitG};

use typewit::{HasTypeWitness, TypeEq};

//////////////////////////////////////////////////////////////////////////////

/// Type alias form of [`Boolean::IfTruePI`]
pub type IfTruePI<B, Then, Else> = <B as Boolean>::IfTruePI<Then, Else>;

/// Type alias form of [`Boolean::IfTrue`]
pub type IfTrue<B, Then, Else> = <B as Boolean>::IfTrue<Then, Else>;

/// Type alias form of [`Boolean::Not`]
pub type Not<B> = <B as Boolean>::Not;

/// Type alias form of [`Boolean::And`]
pub type And<L, R> = <L as Boolean>::And<R>;

/// Type alias form of [`Boolean::Or`]
pub type Or<L, R> = <L as Boolean>::Or<R>;

//////////////////////////////////////////////////////////////////////////////

/// Marker trait for [`typewit::const_marker::Bool`]
pub trait Boolean: 
    Copy + Clone + core::fmt::Debug + Send + Sync +
    HasTypeWitness<BoolWitG<Self>>
{
    /// Logical negation
    type Not: Boolean<Not = Self>;

    /// Logical and
    type And<Rhs: Boolean>: Boolean;
    
    /// Logical or
    type Or<Rhs: Boolean>: Boolean;

    /// Evaluates to different types depending on the type of `Self`:
    /// - if `Self == Bool<true>`: evaluates to `Then`
    /// - if `Self == Bool<false>`: evaluates to `Else`
    type IfTrue<Then, Else>;

    /// Equivalent to `IfTrue` but only takes and returns [`PeanoInt`]s
    type IfTruePI<Then: PeanoInt, Else: PeanoInt>: PeanoInt;

    /// Witness for whether `Self` is `Bool<false>` or `Bool<true>`
    const BOOL_WIT: BoolWitG<Self> = Self::WITNESS;
}

impl Boolean for Bool<false> {
    type Not = Bool<true>;
    
    type And<Rhs: Boolean> = Bool<false>;
    
    type Or<Rhs: Boolean> = Rhs;

    type IfTrue<Then, Else> = Else;

    type IfTruePI<Then: PeanoInt, Else: PeanoInt> = Else;
}

impl Boolean for Bool<true> {
    type Not = Bool<false>;
    
    type And<Rhs: Boolean> = Rhs;
    
    type Or<Rhs: Boolean> = Bool<true>;

    type IfTrue<Then, Else> = Then;

    type IfTruePI<Then: PeanoInt, Else: PeanoInt> = Then;
}


/// Diverges when given a proof of `Bool<true> == Bool<false>`
/// (which is a contradiction, because they're different types).
pub const fn contradiction(length_te: TypeEq<Bool<true>, Bool<false>>) -> ! {
    typewit::type_fn! {
        struct TrueEqualsFalseFn<T, U>;

        impl Bool<true> => T;
        impl Bool<false> => U;
    }

    length_te.map(TrueEqualsFalseFn::NEW).to_left(())
}
