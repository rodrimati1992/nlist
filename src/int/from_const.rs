use typewit::const_marker::Usize;

use crate::bit::{B0, B1};
use crate::int::{Int, Nat, Zeros};

/// Converts a [`Usize<N>`](Usize) into a [peano integer](crate::Int)
pub trait IntoInt: Copy + 'static {
    /// The [peano integer](crate::Int) that this is equivalent to
    type Int: IntoUsize<Usize = Self>;
}

/// Converts a [peano integer](crate::Int) into a [`Usize<N>`](Usize)
pub trait IntoUsize: Int {
    /// The [`Usize`] that this is equivalent to
    type Usize: IntoInt<Int = Self>;
}

/// Converts a [`Usize<N>`](Usize) into a [peano integer](crate::Int)
///
/// Note: only a few integers are supported, you can look at the docs for
/// [`IntoInt`] to see which.
pub type FromUsize<const N: usize> = <Usize<N> as IntoInt>::Int;

/// Converts a [peano integer](crate::Int) into a [`Usize<N>`](Usize)
///
/// Note: only a few integers are supported, you can look at the docs for
/// [`IntoUsize`] to see which.
pub type FromInt<I> = <I as IntoUsize>::Usize;

macro_rules! impl_into_peano {
    (
        $($int:expr => $peano:ty; )*
    ) => (
        $(
            impl IntoInt for Usize<$int> {
                type Int = $peano;
            }
            impl IntoUsize for $peano {
                type Usize = Usize<$int>;
            }
        )*
    )
}

impl_into_peano! {
    0 => Zeros;

    1 => Nat<Zeros, B1>;
    2 => Nat<Nat<Zeros, B1>, B0>;
    3 => Nat<Nat<Zeros, B1>, B1>;
    4 => Nat<Nat<Nat<Zeros, B1>, B0>, B0>;
    5 => Nat<Nat<Nat<Zeros, B1>, B0>, B1>;
    6 => Nat<Nat<Nat<Zeros, B1>, B1>, B0>;
    7 => Nat<Nat<Nat<Zeros, B1>, B1>, B1>;
    8 => Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B0>;
    9 => Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B1>;
    10 => Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B0>;
    11 => Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B1>;
    12 => Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B0>;
    13 => Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B1>;
    14 => Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B0>;
    15 => Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B1>;
    16 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B0>, B0>;
    17 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B0>, B1>;
    18 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B1>, B0>;
    19 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B1>, B1>;
    20 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B0>, B0>;
    21 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B0>, B1>;
    22 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B1>, B0>;
    23 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B1>, B1>;
    24 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B0>, B0>;
    25 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B0>, B1>;
    26 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B1>, B0>;
    27 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B1>, B1>;
    28 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B0>, B0>;
    29 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B0>, B1>;
    30 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B1>, B0>;
    31 => Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B1>, B1>;
    32 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B0>, B0>, B0>;
    33 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B0>, B0>, B1>;
    34 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B0>, B1>, B0>;
    35 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B0>, B1>, B1>;
    36 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B1>, B0>, B0>;
    37 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B1>, B0>, B1>;
    38 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B1>, B1>, B0>;
    39 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B1>, B1>, B1>;
    40 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B0>, B0>, B0>;
    41 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B0>, B0>, B1>;
    42 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B0>, B1>, B0>;
    43 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B0>, B1>, B1>;
    44 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B1>, B0>, B0>;
    45 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B1>, B0>, B1>;
    46 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B1>, B1>, B0>;
    47 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B1>, B1>, B1>, B1>;
    48 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B0>, B0>, B0>;
    49 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B0>, B0>, B1>;
    50 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B0>, B1>, B0>;
    51 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B0>, B1>, B1>;
    52 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B1>, B0>, B0>;
    53 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B1>, B0>, B1>;
    54 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B1>, B1>, B0>;
    55 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B0>, B1>, B1>, B1>;
    56 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B0>, B0>, B0>;
    57 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B0>, B0>, B1>;
    58 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B0>, B1>, B0>;
    59 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B0>, B1>, B1>;
    60 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B1>, B0>, B0>;
    61 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B1>, B0>, B1>;
    62 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B1>, B1>, B0>;
    63 => Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B1>, B1>, B1>, B1>, B1>;
    64 => Nat<Nat<Nat<Nat<Nat<Nat<Nat<Zeros, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
}



/*
used this code to generate the above macro args (other than 0 => Zeros)

fn main(){
    for num in 1..=64u32 {
        let digits = 32 - num.leading_zeros();
        let mut cur_digit = digits;
        let mut rev = num.reverse_bits() >> num.leading_zeros();
        
        print!("{num} => ");
        print!("{}Zeros", "Nat<".repeat(cur_digit as _));
        
        while cur_digit != 0 {
            print!(", B{}", rev % 2);
            rev >>= 1;
            cur_digit -= 1;
            print!(">")
        }
        println!(";")
    }
}
*/