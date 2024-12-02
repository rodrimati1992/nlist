use nlist::{nlist, nlist_pat};

fn main() {
    let nlist_pat![.., _] = nlist![3, 5, 8];
}