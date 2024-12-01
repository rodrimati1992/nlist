use crate::peano::{self, PeanoInt};

use crate::macros::internal_macros::{alt_fn_docs, declare_type_fn};

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


typewit::type_fn!{
    #[doc = alt_fn_docs!("peano", "IfZero")]
    pub struct IfZeroAltFn<Then, Else>;

    impl<This: PeanoInt> This => peano::IfZero<This, Then, Else>
}


typewit::type_fn!{
    #[doc = alt_fn_docs!("peano", "IfZeroPI")]
    pub struct IfZeroPIAltFn<Then, Else>;

    impl<This> This => peano::IfZeroPI<This, Then, Else>
    where
        This: PeanoInt,
        Then: PeanoInt,
        Else: PeanoInt,
}

