#![allow(dead_code)]

/// Hack for implying a trait on type parameters.
/// 
/// credit: https://docs.rs/imply-hack/latest/imply_hack/
pub trait Imply<T: ?Sized>: imply_inner::ImplyInner<T, Is = T> {}

impl<T: ?Sized, U: ?Sized> Imply<T> for U {}

mod imply_inner {
    pub trait ImplyInner<T: ?Sized> {
        type Is: ?Sized;
    }
    
    impl<T: ?Sized, U: ?Sized> ImplyInner<T> for U {
        type Is = T;
    }
}