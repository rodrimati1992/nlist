use crate::PeanoInt;
use crate::boolean::{self, Boolean};

use crate::macros::internal_macros::{alt_fn_docs, declare_type_fn};

declare_type_fn!{ NotFn, boolean, "Not", Boolean::Not<>, Boolean }
declare_type_fn!{ AndFn, boolean, "And", Boolean::And<R>, Boolean }
declare_type_fn!{ OrFn, boolean, "Or", Boolean::Or<R>, Boolean }
declare_type_fn!{ XorFn, boolean, "Xor", Boolean::Xor<R>, Boolean }
declare_type_fn!{ IfTrueFn, boolean, "IfTrue", Boolean::IfTrue<Then, Else>, __NoBound }
declare_type_fn!{ IfTrueBFn, boolean, "IfTrueB", Boolean::IfTrueB<Then, Else>, Boolean }
declare_type_fn!{ IfTruePIFn, boolean, "IfTruePI", Boolean::IfTruePI<Then, Else>, PeanoInt }

mod nobound {
    pub trait __NoBound {}
    impl<T: ?Sized> __NoBound for T {}
}
use nobound::__NoBound;

typewit::type_fn!{
    #[doc = alt_fn_docs!("boolean", "IfTrue")]
    pub struct IfTrueAltFn<Then, Else>;

    impl<B: Boolean> B => boolean::IfTrue<B, Then, Else>
}

typewit::type_fn!{
    #[doc = alt_fn_docs!("boolean", "IfTrueB")]
    pub struct IfTrueBAltFn<Then, Else>;

    impl<B> B => boolean::IfTrueB<B, Then, Else>
    where
        B: Boolean,
        Then: Boolean,
        Else: Boolean,
}

typewit::type_fn!{
    #[doc = alt_fn_docs!("boolean", "IfTruePI")]
    pub struct IfTruePIAltFn<Then, Else>;

    impl<B> B => boolean::IfTruePI<B, Then, Else>
    where
        B: Boolean,
        Then: PeanoInt,
        Else: PeanoInt,
}
