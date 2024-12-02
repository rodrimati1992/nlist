//! Contains proofs of arithmetic properties of [`PeanoInt`]s
//! 
//! These properties are useful in generic contexts, where the compiler does no
//! reasoning WRT the arithmetic properties of [`PeanoInt`]s. 
//! 
//! # Alternative
//! 
//! An easier approach is to use [`peano::eq`]`::<foo, bar>().unwrap_eq()`,
//! but this is prone to causing panics if `foo` or `bar` might change to become unequal.
//! (the proof approach is completely panic-free, though)
//! 
//! # Example
//! 
//! Defining a version of [`NList::flatten`](crate::NList::flatten) function that returns 
//! ```rust
//! # use nlist::{NList, Peano, peano}; 
//! # type T = ();
//! # type LOuter = Peano!(0);
//! # type LInner = Peano!(0);
//! # let _: 
//! NList<T, peano::Mul<LInner, LOuter>>
//! # ;
//! ```
//! instead of
//! ```rust
//! # use nlist::{NList, Peano, peano}; 
//! # type T = ();
//! # type LOuter = Peano!(0);
//! # type LInner = Peano!(0);
//! # let _: 
//! NList<T, peano::Mul<LOuter, LInner>>
//! # ;
//! ```
//! 
//! ```rust
//! use nlist::{NList, NList2D, PeanoInt, nlist, peano};
//! 
//! 
//! const fn flatten_comm<T, LOuter, LInner>(
//!     list: NList2D<T, LOuter, LInner>,
//! ) -> NList<T, peano::Mul<LInner, LOuter>>
//! where   
//!     LOuter: PeanoInt,
//!     LInner: PeanoInt,
//! {
//!     let flat: NList<T, peano::Mul<LOuter, LInner>> = list.flatten();
//!     
//!     flat.coerce_len(peano::proofs::commutative_mul::<LOuter, LInner>())
//! }
//! 
//! ```
//! [`peano::eq`]: crate::peano::eq

use super::*;


/// Proof that `L + R` == `R + L`
pub const fn commutative_add<L, R>() -> TypeEq<Add<L, R>, Add<R, L>>
where
    L: PeanoInt,
    R: PeanoInt,
{
    // axiom
    const {
        super::eq::<Add<L, R>, Add<R, L>>().unwrap_eq()        
    }
}

/// Proof that `L * R` == `R * L`
pub const fn commutative_mul<L, R>() -> TypeEq<Mul<L, R>, Mul<R, L>>
where
    L: PeanoInt,
    R: PeanoInt,
{
    // axiom
    const {
        super::eq::<Mul<L, R>, Mul<R, L>>().unwrap_eq()        
    }
}

/// Proof that `L + 0` == `L`
pub const fn add_identity<L>() -> TypeEq<Add<L, Zero>, L>
where
    L: PeanoInt,
{
    // axiom
    const {
        super::eq::<Add<L, Zero>, L>().unwrap_eq()        
    }
}

/// Proof that `SubSat<L, 0>` == `L`
pub const fn sub_identity<L>() -> TypeEq<SubSat<L, Zero>, L>
where
    L: PeanoInt,
{
    // axiom
    const {
        super::eq::<SubSat<L, Zero>, L>().unwrap_eq()        
    }
}

/// Proof that, if A < C, then`SubSat<A, B> < C`
/// 
pub const fn compose_sub_lt<A, B, C>(
    _a_is_lt_b: TypeEq<IsLt<A, C>, Bool<true>>
) -> TypeEq<IsLt<SubSat<A, B>, C>, Bool<true>>
where
    A: PeanoInt,
    B: PeanoInt,
    C: PeanoInt,
{
    // not in a const block because if this function is monomorphized
    // in a context where A >= C, it could cause a compilation error.
    match Boolean::BOOL_WIT {
        BoolWitG::True(x) => x,
        BoolWitG::False(_) => panic!("axiom"),
    }
}


