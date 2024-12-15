use crate::Int;
use crate::bit::{self, Bit};
use crate::tordering::TOrdering;

use crate::macros::internal_macros::{alt_fn_docs, declare_type_fn};

declare_type_fn!{ NotFn, bit, "Not", Bit::Not<>, Bit }
declare_type_fn!{ AndFn, bit, "And", Bit::And<R>, Bit }
declare_type_fn!{ OrFn, bit, "Or", Bit::Or<R>, Bit }
declare_type_fn!{ XorFn, bit, "Xor", Bit::Xor<R>, Bit }
declare_type_fn!{ BitCmpFn, bit, "BitCmp", Bit::BitCmp<R>, Bit }
declare_type_fn!{ IfTrueFn, bit, "IfTrue", Bit::IfTrue<Then, Else>, __NoBound }
declare_type_fn!{ IfTrueBFn, bit, "IfTrueB", Bit::IfTrueB<Then, Else>, Bit }
declare_type_fn!{ IfTrueIFn, bit, "IfTrueI", Bit::IfTrueI<Then, Else>, Int }
declare_type_fn!{ IfTrueTOFn, bit, "IfTrueTO", Bit::IfTrueTO<Then, Else>, TOrdering }

mod nobound {
    pub trait __NoBound {}
    impl<T: ?Sized> __NoBound for T {}
}
use nobound::__NoBound;

typewit::type_fn!{
    #[doc = alt_fn_docs!("bit", "IfTrue")]
    pub struct IfTrueAltFn<Then, Else>;

    impl<B: Bit> B => bit::IfTrue<B, Then, Else>
}

typewit::type_fn!{
    #[doc = alt_fn_docs!("bit", "IfTrueB")]
    pub struct IfTrueBAltFn<Then, Else>;

    impl<B> B => bit::IfTrueB<B, Then, Else>
    where
        B: Bit,
        Then: Bit,
        Else: Bit,
}

typewit::type_fn!{
    #[doc = alt_fn_docs!("bit", "IfTrueI")]
    pub struct IfTrueIAltFn<Then, Else>;

    impl<B> B => bit::IfTrueI<B, Then, Else>
    where
        B: Bit,
        Then: Int,
        Else: Int,
}
