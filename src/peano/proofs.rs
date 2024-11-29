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

/// Proof that, if A < B, then`SubSat<A, C> < B`
/// 
pub const fn compose_sub_lt<A, B, C>(
    _a_is_lt_b: TypeEq<IsLt<A, B>, Bool<true>>
) -> TypeEq<IsLt<SubSat<A, C>, B>, Bool<true>>
where
    A: PeanoInt,
    B: PeanoInt,
    C: PeanoInt,
{
    // not in a const block because if this function is monomorphized
    // in a context where A >= B, it could cause a compilation error.
    match Boolean::BOOL_WIT {
        BoolWitG::True(x) => x,
        BoolWitG::False(_) => panic!("axiom"),
    }
}


