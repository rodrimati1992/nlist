//! Abstraction over `T`/`&'a T`/`&'a mut T`

use core::marker::PhantomData;

use typewit::{type_fn, Identity, TypeEq};


/// Trait that abstracts over `T`/`&'a T`/`&'a mut T`
pub trait Receiver<'a, T: 'a>: Sized {
    /// Marker type for abstractly representing values/references/mutable references
    // 
    // WORKAROUND: 
    // Not using `Map<T> = Self` because it gives bizarre type equality errors 
    // in downstream uses.
    // If all the tests pass with this bound:
    // ```
    // ReceiverHkt<'a, Map<T> = Self>
    // ```
    // you can use this bound instead of the current bound
    type Hkt: ReceiverHkt<'a, Map<T>: Identity<Type = Self>>;
}


/// Marker trait for abstractly representing values/references/mutable references
pub trait ReceiverHkt<'a>: Sized {
    /// Maps `X` to different types depending on Self:
    /// - if `Self == ValueHkt`: `Self::Map<X> == X`
    /// - if `Self == RefHkt`: `Self::Map<X> == &'a X`
    /// - if `Self == RefMutHkt`: `Self::Map<X> == &'a mut X`
    type Map<X: 'a>: 'a + Receiver<'a, X, Hkt = Self>;

    /// Type witness over `ValueHkt`/`RefHkt`/`RefMutHkt`
    const HKT_WIT: HktWit<'a, Self>;
}


/////////////////////////////////////////////////

impl<'a, T: 'a> Receiver<'a, T> for T {
    type Hkt = ValueHkt<'a>;
}

impl<'a, T: 'a> Receiver<'a, T> for &'a T {
    type Hkt = RefHkt<'a>;
}

impl<'a, T: 'a> Receiver<'a, T> for &'a mut T {
    type Hkt = RefMutHkt<'a>;
}

/////////////////////////////////////////////////

/// Marker type for `T`
pub struct ValueHkt<'a>(PhantomData<fn(&'a ()) -> &'a ()>);

/// Marker type for `&T`
pub struct RefHkt<'a>(PhantomData<fn(&'a ()) -> &'a ()>);

/// Marker type for `&mut T`
pub struct RefMutHkt<'a>(PhantomData<fn(&'a ()) -> &'a ()>);


impl<'a> ReceiverHkt<'a> for ValueHkt<'a> {
    type Map<X: 'a> = X;

    const HKT_WIT: HktWit<'a, Self> = HktWit::Value(TypeEq::NEW);
}

impl<'a> ReceiverHkt<'a> for RefHkt<'a> {
    type Map<X: 'a> = &'a X;

    const HKT_WIT: HktWit<'a, Self> = HktWit::Ref(TypeEq::NEW);
}

impl<'a> ReceiverHkt<'a> for RefMutHkt<'a> {
    type Map<X: 'a> = &'a mut X;

    const HKT_WIT: HktWit<'a, Self> = HktWit::RefMut(TypeEq::NEW);
}

/////////////////////////////////////////////////

/// Gets the [`ReceiverHkt`] of `R`
pub type HktOf<'a, R, T> = <R as Receiver<'a, T>>::Hkt;

/// Maps `H` to different types depending on its value:
/// - `HktMap<'a, ValueHkt , T> == T`
/// - `HktMap<'a, RefHkt   , T> == &'a T`
/// - `HktMap<'a, RefMutHkt, T> == &'a mut T`
pub type HktMap<'a, H, T> = <H as ReceiverHkt<'a>>::Map<T>;

/// Maps `R`'s `T` type argument to `U`.
/// 
/// Maps `R` from:
/// - `T` to `U`
/// - `&'a T` to `&'a U`
/// - `&'a mut T` to `&'a mut U`
pub type MapReceiver<'a, R, T, U> = HktMap<'a, HktOf<'a, R, T>, U>;

/////////////////////////////////////////////////


/// Type witness over `ValueHkt`/`RefHkt`/`RefMutHkt`
pub enum HktWit<'a, K> 
where
    K: ReceiverHkt<'a>
{
    /// Type witness of `K == ValueHkt`
    Value(TypeEq<K, ValueHkt<'a>>),

    /// Type witness of `K == RefHkt`
    Ref(TypeEq<K, RefHkt<'a>>),

    /// Type witness of `K == RefMutHkt`
    RefMut(TypeEq<K, RefMutHkt<'a>>),
}


////////////////////////////////////////////////////////

/// Type witness over `T`/`&T`/`&mut T`
pub enum ReceiverWit<'a, R, T> 
where
    R: Receiver<'a, T>
{
    /// Type witness of `R == T`
    Value(TypeEq<R, T>),

    /// Type witness of `R == &'a T`
    Ref(TypeEq<R, &'a T>),

    /// Type witness of `R == &'a mut T`
    RefMut(TypeEq<R, &'a mut T>),
}

impl<'a, R, T> ReceiverWit<'a, R, T>
where
    R: Receiver<'a, T>,
{
    /// Constructs this `ReceiverWit`
    pub const NEW: Self = {

        // this `TypeEq` created with the `Map<T>: Identity<Type = Self>` bound
        // does what the `Map<T> = Self` bound would do if it worked without issues.
        let identity_te: TypeEq<HktMap<'a, R::Hkt, T>, R> = 
            <HktMap<'a, R::Hkt, T> as Identity>::TYPE_EQ;

        identity_te
            .map(MapReceiverWitRArgFn::<'a, T>::NEW)
            .to_right(match HktOf::<R, T>::HKT_WIT {
                HktWit::Value(te) => ReceiverWit::Value(te.map(HktMapFn::<'a, T>::NEW)),
                HktWit::Ref(te) => ReceiverWit::Ref(te.map(HktMapFn::<'a, T>::NEW)),
                HktWit::RefMut(te) => ReceiverWit::RefMut(te.map(HktMapFn::<'a, T>::NEW)),
            })
    };
}

typewit::type_fn! {
    struct MapReceiverWitRArgFn<'a, T>;

    impl<R> R => ReceiverWit<'a, R, T>
    where 
        R: Receiver<'a, T>,
        T: 'a;
}

////////////////////////////////////////////////////////

typewit::type_fn! {
    struct HktMapFn<'a, T>;

    impl<K: ReceiverHkt<'a>> K => K::Map<T>
    where 
        T: 'a;
}

////////////////////////////////////////////////////////

typewit::type_fn! {
    /// A [`TypeFn`](typewit::TypeFn) version of [`MapReceiver`]
    /// which takes the receiver type as an arguments.
    pub struct MapReceiverFn<'a, T, U>;

    impl<R> R => MapReceiver<'a, R, T, U>
    where 
        R: Receiver<'a, T>,
        T: 'a,
        U: 'a;
}

////////////////////////////////////////////////////////

/// By reference conversion from `&&T`/`&&mut T`/`&T` to `&T`
pub const fn as_ref<'a, 'b, P, T>(this: &'b P) -> &'b T
where
    P: Receiver<'a, T>,
    T: 'a,
    'a: 'b,
{
    let func = type_fn::GRef::<'b>::NEW;

    match ReceiverWit::<'a, P, T>::NEW {
        ReceiverWit::Value(te) => te.map(func).to_right(this),
        ReceiverWit::Ref(te) => te.map(func).to_right(this),
        ReceiverWit::RefMut(te) => te.map(func).to_right(this),
    }
}

