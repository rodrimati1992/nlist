use core::mem::ManuallyDrop as MD;

use nlist::{Peano, PeanoInt, NList, nlist};
use nlist::peano::{IntoUsize, Usize};

use crate::misc_tests::test_utils::assert_type;


#[test]
fn assert_copy_drop_test() {
    const fn inner<T, L>(list: NList<T, L>) 
    where
        T: Copy, 
        L: PeanoInt
    {
        list.assert_copy_drop();
    }

    inner(nlist![0u8; 0]);
    inner(nlist![3]);
    inner(nlist![3, 5]);
    inner(nlist![3, 5, 8]);
}

#[test]
fn assert_copy_test() {
    const fn inner<T, L>(list: NList<T, L>) -> MD<NList<T, L>>
    where
        T: Copy, 
        L: PeanoInt
    {
        list.assert_copy()
    }


    assert_type::<MD<NList<u8, Peano!(0)>>>(nlist![0u8; 0].assert_copy());
    assert_eq!(inner(nlist![0u8; 0]), MD::new(nlist![0u8; 0]));
    
    assert_type::<MD<NList<i32, Peano!(1)>>>(nlist![3].assert_copy());
    assert_eq!(inner(nlist![3]), MD::new(nlist![3]));
    
    assert_type::<MD<NList<i32, Peano!(2)>>>(nlist![3, 5].assert_copy());
    assert_eq!(inner(nlist![3, 5]), MD::new(nlist![3, 5]));
    
    assert_type::<MD<NList<i32, Peano!(3)>>>(nlist![3, 5, 8].assert_copy());
    assert_eq!(inner(nlist![3, 5, 8]), MD::new(nlist![3, 5, 8]));
}

#[test]
fn each_ref_test() {
    const fn inner<T, L>(list: &NList<T, L>) -> NList<&T, L>
    where
        L: PeanoInt
    {
        list.each_ref()
    }

    assert_type::<NList<&u8, Peano!(0)>>(nlist![0u8; 0].each_ref());
    assert_eq!(inner(&nlist![0u8; 0]), nlist![&0u8; 0]);
    
    assert_type::<NList<&i32, Peano!(1)>>(nlist![3].each_ref());
    assert_eq!(inner(&nlist![3]), nlist![&3]);
    
    assert_type::<NList<&i32, Peano!(2)>>(nlist![3, 5].each_ref());
    assert_eq!(inner(&nlist![3, 5]), nlist![&3, &5]);
    
    assert_type::<NList<&i32, Peano!(3)>>(nlist![3, 5, 8].each_ref());
    assert_eq!(inner(&nlist![3, 5, 8]), nlist![&3, &5, &8]);
}

#[test]
fn each_mut_test() {
    const fn inner<T, L>(list: &mut NList<T, L>) -> NList<&mut T, L>
    where
        L: PeanoInt
    {
        list.each_mut()
    }

    assert_type::<NList<&mut u8, Peano!(0)>>(nlist![0u8; 0].each_mut());
    assert_eq!(inner(&mut nlist![0u8; 0]), NList::nil::<&mut u8>());
    
    assert_type::<NList<&mut i32, Peano!(1)>>(nlist![3].each_mut());
    assert_eq!(inner(&mut nlist![3]), nlist![&mut 3]);
    
    assert_type::<NList<&mut i32, Peano!(2)>>(nlist![3, 5].each_mut());
    assert_eq!(inner(&mut nlist![3, 5]), nlist![&mut 3, &mut 5]);
    
    assert_type::<NList<&mut i32, Peano!(3)>>(nlist![3, 5, 8].each_mut());
    assert_eq!(inner(&mut nlist![3, 5, 8]), nlist![&mut 3, &mut 5, &mut 8]);
}

#[test]
fn into_vec_test() {
    // making sure there's no length restriction
    fn inner<T, L>(list: NList<T, L>) -> Vec<T>
    where
        L: PeanoInt
    {
        list.into_vec()
    }

    assert_type::<Vec<u8>>(nlist![0u8; 0].into_vec());
    assert_eq!(inner(nlist![0u8; 0]), vec![0u8; 0]);

    assert_type::<Vec<u16>>(nlist![3u16].into_vec());
    assert_eq!(inner(nlist![3u16]), vec![3u16]);

    assert_type::<Vec<u32>>(nlist![5u32, 8].into_vec());
    assert_eq!(inner(nlist![5u32, 8]), vec![5u32, 8]);

    assert_type::<Vec<u64>>(nlist![13u64, 21, 34].into_vec());
    assert_eq!(inner(nlist![13u64, 21, 34]), vec![13u64, 21, 34]);
}

#[test]
fn into_array_test() {
    const fn inner<T, L, const U: usize>(list: NList<T, L>) -> [T; U]
    where
        L: PeanoInt + IntoUsize<Usize = Usize<U>>
    {
        list.into_array()
    }

    assert_type::<[u8; 0]>(nlist![0u8; 0].into_array());
    assert_eq!(inner(nlist![0u8; 0]), [0u8; 0]);

    assert_type::<[u16; 1]>(nlist![3u16].into_array());
    assert_eq!(inner(nlist![3u16]), [3u16]);

    assert_type::<[u32; 2]>(nlist![5u32, 8].into_array());
    assert_eq!(inner(nlist![5u32, 8]), [5u32, 8]);

    assert_type::<[u64; 3]>(nlist![13u64, 21, 34].into_array());
    assert_eq!(inner(nlist![13u64, 21, 34]), [13u64, 21, 34]);

}
