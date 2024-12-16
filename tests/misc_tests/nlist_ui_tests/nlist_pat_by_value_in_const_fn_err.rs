use nlist::{NList, Int, Peano, nlist_pat, peano};

const fn multisplit<T, L>(list: NList<T, peano::Add<Peano!(2), L>>) -> (T, T, NList<T, L>)
where
    L: Int
{
    let nlist_pat![a, b, c @ ..] = list;
    (a, b, c)
}

fn main() {}