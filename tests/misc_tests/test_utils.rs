macro_rules! assertm {
    ($val:expr, $($tt:tt)*) => {
        match $val {
            x => assert!(matches!(x, $($tt)*), "{x:?}")
        }
    };
} pub(crate) use assertm;


#[track_caller]
pub fn assert_type<Expected>(val: impl Sized) {
    assert_eq!(
        std::any::type_name_of_val(&val),
        std::any::type_name::<Expected>()
    );
}

#[track_caller]
pub fn assert_type_eq<Found, Expected>() {
    assert_eq!(
        std::any::type_name::<Found>(),
        std::any::type_name::<Expected>()
    );
}

macro_rules! if_nonempty {
    (($($cond:tt)+) $($code:tt)*) => { $($code)* };
    (() $($code:tt)*) => {  };
} pub(crate) use if_nonempty;

macro_rules! test_op {
    (
        $self_bound:ident::$assoc_ty:ident<$($args:ident),*> 
        $type_alias:ident 
        $($typefn:ident)?,
        $arg_bound:ident -> $ret_bound:ident, 
        $root_path:ident
        =>
        $(( $first_arg:ty $(,$rem_args:ty)* => $returned:ty ))*
    ) => (
        fn assert_bound<T: $ret_bound>(){}

        #[allow(unused_parens)]
        fn inner<Expected, This: $self_bound, $($args: $arg_bound),*>() {
            assert_bound::<<This as $self_bound>::$assoc_ty<$($args),*>>();

            assert_type_eq::<<This as $self_bound>::$assoc_ty<$($args),*>, Expected>();
            
            assert_type_eq::<$root_path::$type_alias<This $(,$args)*>, Expected>();
      
            crate::misc_tests::test_utils::if_nonempty!{($($typefn)?)
                assert_type_eq::<CallFn<$root_path::type_fns::$($typefn)?, (This $(,$args)*)>, Expected>();
            }
        }

        $(
            inner::<$returned, $first_arg $(,$rem_args)*>();
        )*
    )
} pub(crate) use test_op;


macro_rules! test_nonassoc_op {
    (
        $self_bound:ident
        $type_alias:ident<$($args:ident),*>
        $($typefn:ident)?,
        $arg_bound:ident -> $ret_bound:ident,
        $root_path:ident
        =>
        $(( $($arg_val:ty),* => $returned:ty ))*
    ) => (
        fn assert_bound<T: $ret_bound>(){}

        #[allow(unused_parens)]
        fn inner<Expected, This: $self_bound, $($args: $arg_bound),*>() {
            assert_bound::<$root_path::$type_alias<This $(,$args)*>>();

            assert_type_eq::<$root_path::$type_alias<This $(,$args)*>, Expected>();

            crate::misc_tests::test_utils::if_nonempty!{($($typefn)?)
                assert_type_eq::<CallFn<$root_path::type_fns::$($typefn)?, (This $(,$args)*)>, Expected>();
            }
        }

        $(
            inner::<$returned, $($arg_val),*>();
        )*
    )
} pub(crate) use test_nonassoc_op;
