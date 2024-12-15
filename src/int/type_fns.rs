use crate::int::{self, Int};

use crate::macros::internal_macros::{alt_fn_docs, declare_type_fn};

declare_type_fn!{ SubOneSatFn, int, "SubOneSat", Int::SubOneSat, Int }
declare_type_fn!{ IsZerosFn, int, "IsZeros", Int::IsZeros, Int }
declare_type_fn!{ IsLtFn, int, "IsLt", Int::IsLt<R>, Int }
declare_type_fn!{ IsLeFn, int, "IsLe", Int::IsLe<R>, Int }
declare_type_fn!{ IfZeroFn, int, "IfZero", Int::IfZero<Then, Else>, __NoBound }
declare_type_fn!{ IfZeroIFn, int, "IfZeroI", Int::IfZeroI<Then, Else>, Int }
declare_type_fn!{ SubSatFn, int, "SubSat", Int::SubSat<R>, Int }
declare_type_fn!{ AddFn, int, "Add", Int::Add<R>, Int }
declare_type_fn!{ MulFn, int, "Mul", Int::Mul<R>, Int }
declare_type_fn!{ MinFn, int, "Min", Int::Min<R>, Int }
declare_type_fn!{ MaxFn, int, "Max", Int::Max<R>, Int }

mod nobound {
    pub trait __NoBound {}
    impl<T: ?Sized> __NoBound for T {}
}
use nobound::__NoBound;


typewit::type_fn!{
    #[doc = alt_fn_docs!("int", "IfZero")]
    pub struct IfZeroAltFn<Then, Else>;

    impl<This: Int> This => int::IfZero<This, Then, Else>
}


typewit::type_fn!{
    #[doc = alt_fn_docs!("int", "IfZeroI")]
    pub struct IfZeroIAltFn<Then, Else>;

    impl<This> This => int::IfZeroI<This, Then, Else>
    where
        This: Int,
        Then: Int,
        Else: Int,
}

