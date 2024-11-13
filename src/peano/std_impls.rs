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

impl PartialEq<usize> for Zero {
    fn eq(&self, rhs: &usize) -> bool {
        0 == *rhs
    }
}

impl Eq for Zero {}

impl<Rhs: PeanoInt> PartialOrd<Rhs> for Zero {
    fn partial_cmp(&self, _: &Rhs) -> Option<Ordering> {
        0.partial_cmp(&Rhs::USIZE)
    }
}

impl PartialOrd<usize> for Zero {
    fn partial_cmp(&self, rhs: &usize) -> Option<Ordering> {
        0.partial_cmp(rhs)
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
        Self::USIZE == Rhs::USIZE
    }
}

impl<T: PeanoInt> PartialEq<usize> for PlusOne<T> {
    fn eq(&self, rhs: &usize) -> bool {
        Self::USIZE == *rhs
    }
}

impl<T: PeanoInt> Eq for PlusOne<T> {}

impl<T: PeanoInt, Rhs: PeanoInt> PartialOrd<Rhs> for PlusOne<T> {
    fn partial_cmp(&self, _: &Rhs) -> Option<Ordering> {
        Self::USIZE.partial_cmp(&Rhs::USIZE)
    }
}

impl<T: PeanoInt> PartialOrd<usize> for PlusOne<T> {
    fn partial_cmp(&self, rhs: &usize) -> Option<Ordering> {
        Self::USIZE.partial_cmp(rhs)
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
        Self::USIZE.hash(hasher)
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
