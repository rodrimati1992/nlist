use crate::peano::{self, PeanoInt};

macro_rules! declare_type_fn {
    ($fn_name:ident, $opname:literal, $op:ident $(<$( $args:ident),* >)?, $bound:ident ) => (

        typewit::type_fn! {
            #[doc = concat!(
                "Type-level function ([`TypeFn`] implementor) form of [`peano::",
                $opname,
                "`]",
            )]
            /// [`TypeFn`]: typewit::TypeFn
            pub struct $fn_name;

            impl<L: PeanoInt $($(,$args: $bound)*)?> (L $($(,$args)*)?) => 
                peano::$op<L, $($($args,)*)?>;
        }

    )
}


declare_type_fn!{ SubOneSatFn, "SubOneSat", SubOneSat, PeanoInt }
declare_type_fn!{ IsZeroFn, "IsZero", IsZero, PeanoInt }
declare_type_fn!{ IsLtFn, "IsLt", IsLt<R>, PeanoInt }
declare_type_fn!{ IsLeFn, "IsLe", IsLe<R>, PeanoInt }
declare_type_fn!{ IfZeroFn, "IfZero", IfZero<Then, Else>, __NoBound }
declare_type_fn!{ IfZeroPIFn, "IfZeroPI", IfZeroPI<Then, Else>, PeanoInt }
declare_type_fn!{ SubSatFn, "SubSat", SubSat<R>, PeanoInt }
declare_type_fn!{ AddFn, "Add", Add<R>, PeanoInt }
declare_type_fn!{ MulFn, "Mul", Mul<R>, PeanoInt }
declare_type_fn!{ MinFn, "Min", Min<R>, PeanoInt }
declare_type_fn!{ MaxFn, "Max", Max<R>, PeanoInt }

mod nobound {
    pub trait __NoBound {}
    impl<T: ?Sized> __NoBound for T {}
}
use nobound::__NoBound;