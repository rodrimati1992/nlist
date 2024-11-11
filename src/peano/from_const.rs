use typewit::const_marker::Usize;

use crate::peano::{PeanoInt, Zero};
use crate::peano::PlusOne as Plus1;


/// Converts a [`Usize<N>`](Usize) into a [peano integer](crate::PeanoInt)
pub trait IntoPeano: Copy + 'static {
    /// The [peano integer](crate::PeanoInt) that this is equivalent to
    type Peano: IntoUsize<Usize = Self>;
}

/// Converts a [peano integer](crate::PeanoInt) into a [`Usize<N>`](Usize)
pub trait IntoUsize: PeanoInt {
    /// The [`Usize`] that this is equivalent to
    type Usize: IntoPeano<Peano = Self>;
}

/// Converts a [`Usize<N>`](Usize) into a [peano integer](crate::PeanoInt)
/// 
/// Note: only a few integers are supported, you can look at the docs for 
/// [`IntoPeano`] to see which.
pub type FromUsize<const N: usize> = <Usize<N> as IntoPeano>::Peano;


/// Converts a [peano integer](crate::PeanoInt) into a [`Usize<N>`](Usize)
/// 
/// Note: only a few integers are supported, you can look at the docs for 
/// [`IntoUsize`] to see which.
pub type FromPeano<I> = <I as IntoUsize>::Usize;


macro_rules! impl_into_peano {
    (
        $($int:expr => $peano:ty; )*
    ) => (
        $(
            impl IntoPeano for Usize<$int> {
                type Peano = $peano;
            }
            impl IntoUsize for $peano {
                type Usize = Usize<$int>;
            }
        )*
    )
}




macro_rules! declare_10_impls {
    ([$v0:tt $v1:tt $v2:tt $v3:tt $v4:tt $v5:tt $v6:tt $v7:tt $v8:tt $v9:tt], $base_ty:ty) => {
        const _: () = {
            type Base = $base_ty;

            impl_into_peano!{
                $v0 => Base;
                $v1 => Plus1<Base>;
                $v2 => Plus1<Plus1<Base>>;
                $v3 => Plus1<Plus1<Plus1<Base>>>;
                $v4 => Plus1<Plus1<Plus1<Plus1<Base>>>>;
                $v5 => Plus1<Plus1<Plus1<Plus1<Plus1<Base>>>>>;
                $v6 => Plus1<Plus1<Plus1<Plus1<Plus1<Plus1<Base>>>>>>;
                $v7 => Plus1<Plus1<Plus1<Plus1<Plus1<Plus1<Plus1<Base>>>>>>>;
                $v8 => Plus1<Plus1<Plus1<Plus1<Plus1<Plus1<Plus1<Plus1<Base>>>>>>>>;
                $v9 => Plus1<Plus1<Plus1<Plus1<Plus1<Plus1<Plus1<Plus1<Plus1<Base>>>>>>>>>;
            }
        };
    }
}








declare_10_impls!{[0 1 2 3 4 5 6 7 8 9], Zero}
declare_10_impls!{[10 11 12 13 14 15 16 17 18 19], Plus1<FromUsize<9>>}
declare_10_impls!{[20 21 22 23 24 25 26 27 28 29], Plus1<FromUsize<19>>}
declare_10_impls!{[30 31 32 33 34 35 36 37 38 39], Plus1<FromUsize<29>>}
declare_10_impls!{[40 41 42 43 44 45 46 47 48 49], Plus1<FromUsize<39>>}
declare_10_impls!{[50 51 52 53 54 55 56 57 58 59], Plus1<FromUsize<49>>}

const _: () = {
    type Base = Plus1<FromUsize<59>>;

    impl_into_peano!{
        60 => Base;
        61 => Plus1<Base>;
        62 => Plus1<Plus1<Base>>;
        63 => Plus1<Plus1<Plus1<Base>>>;
        64 => Plus1<Plus1<Plus1<Plus1<Base>>>>;
    }
};