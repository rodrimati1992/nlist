use crate::peano::{self, PeanoInt};

use crate::macros::internal_macros::declare_type_fn;

declare_type_fn!{ SubOneSatFn, peano, "SubOneSat", PeanoInt::SubOneSat, PeanoInt }
declare_type_fn!{ IsZeroFn, peano, "IsZero", PeanoInt::IsZero, PeanoInt }
declare_type_fn!{ IsLtFn, peano, "IsLt", PeanoInt::IsLt<R>, PeanoInt }
declare_type_fn!{ IsLeFn, peano, "IsLe", PeanoInt::IsLe<R>, PeanoInt }
declare_type_fn!{ IfZeroFn, peano, "IfZero", PeanoInt::IfZero<Then, Else>, __NoBound }
declare_type_fn!{ IfZeroPIFn, peano, "IfZeroPI", PeanoInt::IfZeroPI<Then, Else>, PeanoInt }
declare_type_fn!{ SubSatFn, peano, "SubSat", PeanoInt::SubSat<R>, PeanoInt }
declare_type_fn!{ AddFn, peano, "Add", PeanoInt::Add<R>, PeanoInt }
declare_type_fn!{ MulFn, peano, "Mul", PeanoInt::Mul<R>, PeanoInt }
declare_type_fn!{ MinFn, peano, "Min", PeanoInt::Min<R>, PeanoInt }
declare_type_fn!{ MaxFn, peano, "Max", PeanoInt::Max<R>, PeanoInt }

mod nobound {
    pub trait __NoBound {}
    impl<T: ?Sized> __NoBound for T {}
}
use nobound::__NoBound;