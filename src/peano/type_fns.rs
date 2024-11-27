use crate::peano::{self, PeanoInt};

macro_rules! declare_type_fn {
    ($fn_name:ident, $opname:literal, $op:ident $(<$( $args:ident),* >)?) => (

        typewit::type_fn! {
            #[doc = concat!(
                "Type-level function ([`TypeFn`] implementor) form of [`peano::",
                $opname,
                "`]",
            )]
            /// [`TypeFn`]: typewit::TypeFn
            pub struct $fn_name;

            impl<L: PeanoInt $($(,$args: PeanoInt)*)?> (L $($(,$args)*)?) => 
                peano::$op<L, $($($args,)*)?>;
        }

    )
}


declare_type_fn!{ SubOneSatFn, "SubOneSat", SubOneSat }
declare_type_fn!{ IsZeroFn, "IsZero", IsZero }
declare_type_fn!{ IsLtFn, "IsLt", IsLt<R> }
declare_type_fn!{ IsLeFn, "IsLe", IsLe<R> }
declare_type_fn!{ IfZeroFn, "IfZero", IfZero<Then, Else> }
declare_type_fn!{ IfZeroPIFn, "IfZeroPI", IfZeroPI<Then, Else> }
declare_type_fn!{ SubSatFn, "SubSat", SubSat<R> }
declare_type_fn!{ AddFn, "Add", Add<R> }
declare_type_fn!{ MulFn, "Mul", Mul<R> }
declare_type_fn!{ MinFn, "Min", Min<R> }
declare_type_fn!{ MaxFn, "Max", Max<R> }

