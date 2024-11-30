use crate::receiver::{MapReceiver, Receiver};

typewit::type_fn! {
    /// A [`TypeFn`](typewit::TypeFn) version of [`MapReceiver`]
    /// which takes the receiver type as an argument.
    pub struct MapReceiverFn<'a, T, U>;

    impl<R> R => MapReceiver<'a, R, T, U>
    where 
        R: Receiver<'a, T>,
        T: 'a,
        U: 'a;
}
