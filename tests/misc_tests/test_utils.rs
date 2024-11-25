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
