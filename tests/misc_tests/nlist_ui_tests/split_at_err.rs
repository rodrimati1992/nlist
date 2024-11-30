use nlist::{Peano, nlist};

const _: () = {
    let mut list = nlist![3, 5, 8];
    _ = list.split_at::<Peano!(3)>();
    _ = list.split_at::<Peano!(4)>();
};

fn main(){}