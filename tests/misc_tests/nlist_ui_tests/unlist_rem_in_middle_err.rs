use nlist::{nlist, unlist};

fn main() {
    unlist!{[.., _] = nlist![3, 5, 8]}
}