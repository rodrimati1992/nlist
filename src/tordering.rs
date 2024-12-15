//! Type-level encoding of [`core::cmp::Ordering`]

use crate::{
    bit::{Bit, B0, B1}
};


/// Type alias equivalent of [`TOrdering::IsLess`]
pub type IsLess<Lhs> = <Lhs as TOrdering>::IsLess;

/// Type alias equivalent of [`TOrdering::IsLe`]
pub type IsLe<Lhs> = <Lhs as TOrdering>::IsLe;

/// Type alias equivalent of [`TOrdering::IsEqual`]
pub type IsEqual<Lhs> = <Lhs as TOrdering>::IsEqual;

/// Type alias equivalent of [`TOrdering::IsGe`]
pub type IsGe<Lhs> = <Lhs as TOrdering>::IsGe;

/// Type alias equivalent of [`TOrdering::IsGreater`]
pub type IsGreater<Lhs> = <Lhs as TOrdering>::IsGreater;

/// Type alias equivalent of [`TOrdering::OrdThen`]
pub type OrdThen<Lhs, Rhs> = <Lhs as TOrdering>::OrdThen<Rhs>;

/// Type-level encoding of [`core::cmp::Ordering`]
pub trait TOrdering: Copy + Clone + core::fmt::Debug + Send + Sync + 'static {
    /// Whether `Self` is `TLess`
    type IsLess: Bit;
    /// Whether `Self` is `TLess` or `TEqual`
    type IsLe: Bit;
    /// Whether `Self` is `TEqual`
    type IsEqual: Bit;
    /// Whether `Self` is `TEqual` or `TGreater`
    type IsGe: Bit;
    /// Whether `Self` is `TGreater`
    type IsGreater: Bit;

    /// Type-level equivalent of [`std::cmp::Ordering::then`]
    type OrdThen<Rhs: TOrdering>: TOrdering;
}


/// Type-level equivalent of [`std::cmp::Ordering::Less`]
#[derive(Debug, Copy, Clone)]
pub struct TLess;

impl TOrdering for TLess {
    type IsLess = B1;
    type IsLe = B1;
    type IsEqual = B0;
    type IsGe = B0;
    type IsGreater = B0;

    type OrdThen<Rhs: TOrdering> = Self;
}

/// Type-level equivalent of [`std::cmp::Ordering::Equal`]
#[derive(Debug, Copy, Clone)]
pub struct TEqual;

impl TOrdering for TEqual {
    type IsLess = B0;
    type IsLe = B1;
    type IsEqual = B1;
    type IsGe = B1;
    type IsGreater = B0;
    
    type OrdThen<Rhs: TOrdering> = Rhs;
}


/// Type-level equivalent of [`std::cmp::Ordering::Greater`]
#[derive(Debug, Copy, Clone)]
pub struct TGreater;

impl TOrdering for TGreater {
    type IsLess = B0;
    type IsLe = B0;
    type IsEqual = B0;
    type IsGe = B1;
    type IsGreater = B1;
    
    type OrdThen<Rhs: TOrdering> = Self;
}
