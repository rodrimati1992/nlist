use nlist::{Peano, nlist};

const _: () = {
    let mut list = nlist![3, 5, 8];
    _ = list.index::<Peano!(2)>();
    _ = list.index::<Peano!(3)>();

    _ = list.index_mut::<Peano!(2)>();
    _ = list.index_mut::<Peano!(3)>();
};

fn main(){}