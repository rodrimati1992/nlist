#[macro_export]
#[doc(hidden)]
macro_rules! __first_ty {
    ($first:ty, $($rem:tt)* ) => {
        $first
    };
}

///////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_0_args {
    ($callback:ident ($($args:tt)*) || $( -> $ret_ty:ty )? $block:block $(,)?) => {
        $crate::$callback! {$($args)* || -> $crate::__first_ty!{$($ret_ty,)? _,} $block}
    };
    ($callback:ident ($($args:tt)*) || $expr:expr $(,)?) => {
        $crate::$callback! {$($args)* || -> _ { $expr }}
    };
    ($callback:ident ($($args:tt)*) $func:expr $(,)?) => {
        $crate::$callback! {$($args)* || -> _ { $func() }}
    };
}  


///////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_2_args {
    (
        $callback:ident ($($args:tt)*)

        |$pat0:tt $(: $ty0:ty)?, $pat1:tt $(: $ty1:ty)? $(,)?| 
        $( -> $ret_ty:ty )?
        $block:block
        $(,)?
    ) => {
        $crate::$callback! {
            $($args)*

            |$pat0: $crate::__first_ty!{$($ty0,)? _,}, $pat1: $crate::__first_ty!{$($ty1,)? _,}| 
            -> $crate::__first_ty!{$($ret_ty,)? _,}
            $block
        }
    };
    (
        $callback:ident ($($args:tt)*)

        |$pat0:tt $(: $ty0:ty)?, $pat1:tt $(: $ty1:ty)? $(,)?| $expr:expr $(,)?
    ) => {
        $crate::$callback! {
            $($args)*

            |$pat0: $crate::__first_ty!{$($ty0,)? _,}, $pat1: $crate::__first_ty!{$($ty1,)? _,}| 
            -> _
            { $expr }
        }
    };
    (
        $callback:ident ($($args:tt)*)

        $func:expr $(,)?
    ) => {
        $crate::$callback! {$($args)* |a: _, b: _| -> _ { $func(a, b) }}
    };
}  



