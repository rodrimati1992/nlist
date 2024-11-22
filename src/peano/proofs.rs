//! Contains proofs of arithmetic properties of [`PeanoInt`]s

use super::*;


/// Proof that `L + R` == `R + L`
pub const fn commutative_add<L, R>() -> TypeEq<Add<L, R>, Add<R, L>>
where
    L: PeanoInt,
    R: PeanoInt,
{
    // axiom
    const {
        super::eq(Add::<L, R>::NEW, Add::<R, L>::NEW).unwrap_eq()        
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
        super::eq(Mul::<L, R>::NEW, Mul::<R, L>::NEW).unwrap_eq()        
    }
}

/// Proof that `L + 0` == `L`
pub const fn add_identity<L>() -> TypeEq<Add<L, Zero>, L>
where
    L: PeanoInt,
{
    // axiom
    const {
        super::eq(Add::<L, Zero>::NEW, L::NEW).unwrap_eq()        
    }
}

/// Proof that `SubSat<L, 0>` == `L`
pub const fn sub_identity<L>() -> TypeEq<SubSat<L, Zero>, L>
where
    L: PeanoInt,
{
    // axiom
    const {
        super::eq(SubSat::<L, Zero>::NEW, L::NEW).unwrap_eq()        
    }
}


