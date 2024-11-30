//! Abstraction over `T`/`&'a T`/`&'a mut T`
//! 
//! # Example
//! 
//! ```rust
//! use nlist::receiver::{self, HktApply, Receiver, ReceiverWit};
//! 
//! assert_eq!(field(Wrapper(3)), 3);
//! assert_eq!(field(&Wrapper(3)), &3);
//! assert_eq!(field(&mut Wrapper(3)), &mut 3);
//! 
//! 
//! struct Wrapper(u32);
//! 
//! /// Projects a `Wrapper/&Wrapper/&mut Wrapper` to its wrapped field
//! const fn field<'a, S>(this: S) -> HktApply<'a, S::Hkt, u32>
//! where
//!     S: Receiver<'a, Wrapper>
//! {
//!     let mapper = receiver::MapReceiverFn::<'a, Wrapper, u32>::NEW;
//!     match ReceiverWit::<'a, S, Wrapper>::NEW {
//!         ReceiverWit::Value(te) => te.map(mapper).to_left(te.to_right(this).0),
//!         ReceiverWit::Ref(te) => te.map(mapper).to_left(&te.to_right(this).0),
//!         ReceiverWit::RefMut(te) => te.map(mapper).to_left(&mut te.to_right(this).0),
//!     }
//! }
//! ```



use core::marker::PhantomData;
use core::fmt::{self, Debug};

use typewit::{type_fn, Identity, TypeEq};


/// Trait that abstracts over `T`/`&'a T`/`&'a mut T`
pub trait Receiver<'a, T: 'a>: Sized {
    /// Marker type for abstractly representing values/references/mutable references
    // 
    // WORKAROUND: 
    // Not using `Apply<T> = Self` because it gives bizarre type equality errors 
    // in downstream uses.
    // If all the tests pass with this bound:
    // ```
    // ReceiverHkt<'a, Apply<T> = Self>
    // ```
    // you can use this bound instead of the current bound
    type Hkt: ReceiverHkt<'a, Apply<T>: Identity<Type = Self>>;

    /// Witness for whether Self is a `T`,`&'a T`, or `&'a mut T`.
    const RECEIVER_WIT :ReceiverWit<'a, Self, T> = {
        // this `TypeEq` created with the `Apply<T>: Identity<Type = Self>` bound
        // does what the `Apply<T> = Self` bound would do if it worked without issues.
        let identity_te: TypeEq<HktApply<'a, Self::Hkt, T>, Self> = 
            <HktApply<'a, Self::Hkt, T> as Identity>::TYPE_EQ;

        identity_te
            .map(MapReceiverWitRArgFn::<'a, T>::NEW)
            .to_right(match HktOf::<Self, T>::HKT_WIT {
                HktWit::Value(te) => ReceiverWit::Value(te.map(HktApplyFn::<'a, T>::NEW)),
                HktWit::Ref(te) => ReceiverWit::Ref(te.map(HktApplyFn::<'a, T>::NEW)),
                HktWit::RefMut(te) => ReceiverWit::RefMut(te.map(HktApplyFn::<'a, T>::NEW)),
            })
    };
}


/// Marker trait for `ValueHkt`/`RefHkt`/`RefMutHkt` marker types
pub trait ReceiverHkt<'a>: Sized {
    /// Maps `X` to different types depending on Self:
    /// - if `Self == ValueHkt`: `Self::Apply<X> == X`
    /// - if `Self == RefHkt`: `Self::Apply<X> == &'a X`
    /// - if `Self == RefMutHkt`: `Self::Apply<X> == &'a mut X`
    type Apply<X: 'a>: 'a + Receiver<'a, X, Hkt = Self>;

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

/// Marker type for an abstract `T`
pub struct ValueHkt<'a>(PhantomData<fn(&'a ()) -> &'a ()>);

/// Marker type for an abstract `&T`
pub struct RefHkt<'a>(PhantomData<fn(&'a ()) -> &'a ()>);

/// Marker type for an abstract `&mut T`
pub struct RefMutHkt<'a>(PhantomData<fn(&'a ()) -> &'a ()>);


impl<'a> ReceiverHkt<'a> for ValueHkt<'a> {
    type Apply<X: 'a> = X;

    const HKT_WIT: HktWit<'a, Self> = HktWit::Value(TypeEq::NEW);
}

impl<'a> ReceiverHkt<'a> for RefHkt<'a> {
    type Apply<X: 'a> = &'a X;

    const HKT_WIT: HktWit<'a, Self> = HktWit::Ref(TypeEq::NEW);
}

impl<'a> ReceiverHkt<'a> for RefMutHkt<'a> {
    type Apply<X: 'a> = &'a mut X;

    const HKT_WIT: HktWit<'a, Self> = HktWit::RefMut(TypeEq::NEW);
}

/////////////////////////////////////////////////

/// Gets the [`Receiver::Hkt`] of `R`
pub type HktOf<'a, R, T> = <R as Receiver<'a, T>>::Hkt;

/// Maps `H` to different types depending on its value:
/// - `HktApply<'a, ValueHkt , T> == T`
/// - `HktApply<'a, RefHkt   , T> == &'a T`
/// - `HktApply<'a, RefMutHkt, T> == &'a mut T`
pub type HktApply<'a, H, T> = <H as ReceiverHkt<'a>>::Apply<T>;

/// Maps the `T` type argument of `R:`[`Receiver`]`<'a, T>` to `U`.
/// 
/// The different ways this type alias can be used:
/// - `MapReceiver<'a, T        , T, U> == U`
/// - `MapReceiver<'a, &'a T    , T, U> == &'a U`
/// - `MapReceiver<'a, &'a mut T, T, U> == &'a mut U`
pub type MapReceiver<'a, R, T, U> = HktApply<'a, HktOf<'a, R, T>, U>;

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

impl<'a, K> Copy for HktWit<'a, K>
where
    K: ReceiverHkt<'a>
{}

impl<'a, K> Clone for HktWit<'a, K>
where
    K: ReceiverHkt<'a>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, K> Debug for HktWit<'a, K>
where
    K: ReceiverHkt<'a>
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value{..} => fmt.write_str("Value"),
            Self::Ref{..} => fmt.write_str("Ref"),
            Self::RefMut{..} => fmt.write_str("RefMut"),
        }
    }
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

impl<'a, R, T> Copy for ReceiverWit<'a, R, T>
where
    R: Receiver<'a, T>,
{}

impl<'a, R, T> Clone for ReceiverWit<'a, R, T>
where
    R: Receiver<'a, T>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, R, T> Debug for ReceiverWit<'a, R, T>
where
    R: Receiver<'a, T>,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value{..} => fmt.write_str("Value"),
            Self::Ref{..} => fmt.write_str("Ref"),
            Self::RefMut{..} => fmt.write_str("RefMut"),
        }
    }
}

impl<'a, R, T> ReceiverWit<'a, R, T>
where
    R: Receiver<'a, T>,
{
    /// Constructs this `ReceiverWit`
    pub const NEW: Self = R::RECEIVER_WIT;
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
    struct HktApplyFn<'a, T>;

    impl<K: ReceiverHkt<'a>> K => K::Apply<T>
    where 
        T: 'a;
}

////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////

/// By reference conversion from `&&T`/`&&mut T`/`&T` to `&T`
///
/// # Example 
///
/// ```rust
/// use nlist::receiver::{self, HktApply, Receiver, ReceiverWit};
/// 
/// assert_eq!(addup(&(3, 5)), 8);
/// assert_eq!(addup(&&(5, 8)), 13);
/// assert_eq!(addup(&&mut (8, 13)), 21);
/// 
///
/// /// Adds up the fields in a `&&(u32, u32)`, `&&mut (u32, u32)`, or `&(u32, u32)`
/// const fn addup<'a, S>(this: &S) -> u32
/// where
///     S: Receiver<'a, (u32, u32)>
/// {
///     let pair: &(u32, u32) = receiver::as_ref(this);
/// 
///     pair.0 + pair.1
/// }
/// ```

pub const fn as_ref<'a, 'b, T>(this: &'b impl Receiver<'a, T>) -> &'b T
where
    T: 'a,
    'a: 'b,
{
    let func = type_fn::GRef::<'b>::NEW;

    match ReceiverWit::<'a, _, T>::NEW {
        ReceiverWit::Value(te) => te.map(func).to_right(this),
        ReceiverWit::Ref(te) => te.map(func).to_right(this),
        ReceiverWit::RefMut(te) => te.map(func).to_right(this),
    }
}

