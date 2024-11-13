use core::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt,
    hash::{Hash, Hasher},
};

use super::{PeanoInt, PlusOne, Zero};


//////////////////////////////////////////////////////////////
//          formatting impls
//////////////////////////////////////////////////////////////

macro_rules! delegate_fmt_trait {
    ($trait:ident) => {
        impl fmt::$trait for Zero {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::$trait::fmt(&0, fmt)
            }
        }

        impl<T> fmt::$trait for PlusOne<T>
        where
            T: PeanoInt,
        {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::$trait::fmt(&Self::USIZE, fmt)
            }
        }
    }
}

delegate_fmt_trait!{Binary}
delegate_fmt_trait!{Debug}
delegate_fmt_trait!{Display}
delegate_fmt_trait!{LowerHex}
delegate_fmt_trait!{Octal}
delegate_fmt_trait!{UpperHex}


//////////////////////////////////////////////////////////////
//          comparison impls
//////////////////////////////////////////////////////////////


impl<Rhs: PeanoInt> PartialEq<Rhs> for Zero {
    fn eq(&self, _: &Rhs) -> bool {
        0 == Rhs::USIZE
    }
}

impl Eq for Zero {}

impl<Rhs: PeanoInt> PartialOrd<Rhs> for Zero {
    fn partial_cmp(&self, _: &Rhs) -> Option<Ordering> {
        0.partial_cmp(&Rhs::USIZE)
    }
}

impl Ord for Zero {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

///////////////

impl<T: PeanoInt, Rhs: PeanoInt> PartialEq<Rhs> for PlusOne<T> {
    fn eq(&self, _: &Rhs) -> bool {
        T::USIZE == Rhs::USIZE
    }
}

impl<T: PeanoInt> Eq for PlusOne<T> {}

impl<T: PeanoInt, Rhs: PeanoInt> PartialOrd<Rhs> for PlusOne<T> {
    fn partial_cmp(&self, _: &Rhs) -> Option<Ordering> {
        T::USIZE.partial_cmp(&Rhs::USIZE)
    }
}

impl<T: PeanoInt> Ord for PlusOne<T> {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

//////////////////////////////////////////////////////////////

impl Hash for Zero {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        0usize.hash(hasher)
    }
}

impl<T: PeanoInt> Hash for PlusOne<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        T::USIZE.hash(hasher)
    }
}


impl Default for Zero {
    fn default() -> Self {
        Zero
    }
}

impl<T: PeanoInt> Default for PlusOne<T> {
    fn default() -> Self {
        Self::NEW
    }
}


impl<T: PeanoInt> Copy for PlusOne<T> {}

impl<T: PeanoInt> Clone for PlusOne<T> {
    fn clone(&self) -> Self {
        *self
    }
}
