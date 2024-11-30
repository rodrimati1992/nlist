use nlist::receiver::{
    self, HktApply, HktWit, MapReceiver, MapReceiverFn, Receiver, ReceiverHkt, ReceiverWit
};
use nlist::typewit::{CallFn, TypeEq};

use crate::misc_tests::test_utils::{assertm, assert_type, assert_type_eq};


struct Wrapper(u32);

const fn field<'a, S>(this: S) -> HktApply<'a, S::Hkt, u32>
where
    S: Receiver<'a, Wrapper>
{
    let mapper = MapReceiverFn::<'a, Wrapper, u32>::NEW;
    match ReceiverWit::<'a, S, Wrapper>::NEW {
        ReceiverWit::Value(te) => te.map(mapper).to_left(te.to_right(this).0),
        ReceiverWit::Ref(te) => te.map(mapper).to_left(&te.to_right(this).0),
        ReceiverWit::RefMut(te) => te.map(mapper).to_left(&mut te.to_right(this).0),
    }
}


#[test]
fn receiver_inference_test() {
    assert_type::<u32>(field(Wrapper(3)));
    assert_eq!(field(Wrapper(3)), 3);

    assert_type::<&u32>(field(&Wrapper(3)));
    assert_eq!(field(&Wrapper(3)), &3);

    assert_type::<&mut u32>(field(&mut Wrapper(3)));
    assert_eq!(field(&mut Wrapper(3)), &mut 3);
} 


#[test]
fn hkt_apply_test() {
    fn inner_generic_on_hkt<'a, K: ReceiverHkt<'a>, T: 'a, Expected>() {
        _ = TypeEq::<<K as ReceiverHkt<'a>>::Apply<T>, receiver::HktApply<'a, K, T>>::NEW;
        assert_type_eq::<receiver::HktApply<'a, K, T>, Expected>();
    }
    fn inner<'a, T: 'a>() {
        inner_generic_on_hkt::<receiver::RefHkt<'a>, T, &'a T>();
        inner_generic_on_hkt::<receiver::RefMutHkt<'a>, T, &'a mut T>();
        inner_generic_on_hkt::<receiver::ValueHkt<'a>, T, T>();
    }

    inner::<u32>();
}

#[test]
fn map_receiver_test() {
    fn inner<'a, R: Receiver<'a, T>, T: 'a, U: 'a, Expected>() {
        assert_type_eq::<MapReceiver<'a, R, T, U>, Expected>();
        assert_type_eq::<CallFn<MapReceiverFn<'a, T, U>, R>, Expected>();

        _ = TypeEq::<MapReceiver<'a, R, T, U>, CallFn<MapReceiverFn<'a, T, U>, R>>::NEW;
    }

    inner::<u32, u32, u64, u64>();
    inner::<&u32, u32, u64, &u64>();
    inner::<&mut u32, u32, u64, &mut u64>();
}


#[test]
fn hkt_wit_test() {
    assertm!(receiver::RefHkt::HKT_WIT, HktWit::Ref{..});
    assertm!(receiver::RefMutHkt::HKT_WIT, HktWit::RefMut{..});
    assertm!(receiver::ValueHkt::HKT_WIT, HktWit::Value{..});
}

#[test]
fn receiver_assoc_items_test() {
    fn inner<'a, T: 'a>() {
        assertm!(<&'a T as Receiver<'a, T>>::RECEIVER_WIT, ReceiverWit::Ref{..});
        assertm!(<&'a mut T as Receiver<'a, T>>::RECEIVER_WIT, ReceiverWit::RefMut{..});
        assertm!(<T as Receiver<'a, T>>::RECEIVER_WIT, ReceiverWit::Value{..});
        
        _ = TypeEq::<<&'a T as Receiver<'a, T>>::Hkt, receiver::RefHkt<'a>>::NEW;
        _ = TypeEq::<<&'a mut T as Receiver<'a, T>>::Hkt, receiver::RefMutHkt<'a>>::NEW;
        _ = TypeEq::<<T as Receiver<'a, T>>::Hkt, receiver::ValueHkt<'a>>::NEW;
    }

    inner::<u32>()
}

#[test]
fn hkt_of_test() {
    fn _inner_receiver<'a, T: 'a, R: Receiver<'a, T>>() {
        _ = TypeEq::<<T as Receiver<'a, T>>::Hkt, receiver::HktOf<'a, T, T>>::NEW;
        _ = TypeEq::<<&'a T as Receiver<'a, T>>::Hkt, receiver::HktOf<'a, &'a T, T>>::NEW;
        _ = TypeEq::<<&'a mut T as Receiver<'a, T>>::Hkt, receiver::HktOf<'a, &'a mut T, T>>::NEW;
    }
}

#[test]
fn as_ref_test() {
    const fn works_in_const_and_generic<'a: 'b, 'b, P: Receiver<'a, u32>>(x: &'b P) {
        let _: &'b u32 = receiver::as_ref(x);
    }

    works_in_const_and_generic(&3);
    works_in_const_and_generic(&&3);
    works_in_const_and_generic(&&mut 3);

    assert_type::<&u32>(const { receiver::as_ref::<u32>(&3) });
    assert_eq!(receiver::as_ref::<u32>(&3), &3);

    assert_type::<&u32>(const { receiver::as_ref::<u32>(&&3) });
    assert_eq!(receiver::as_ref::<u32>(&&3), &3);

    assert_type::<&u32>(receiver::as_ref::<u32>(&&mut 3));
    assert_eq!(receiver::as_ref::<u32>(&&mut 3), &3);
} 

