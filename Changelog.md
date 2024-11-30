# 0.1

### 0.1.0

Added these items in the root module:
- `Cons` struct
- `Nil` struct
- `NList` struct
- `NListFn` struct (`::typewit::TypeFn` implementor)
. `NList2D` type alias
. `nlist` macro
. `Node` type alias
. `Peano` macro
. `peano` macro
. `rec_all` macro
. `rec_any` macro
. `rec_find_map` macro
. `rec_fold` macro
. `rec_for_each` macro
. `rec_from_fn` macro
. `rec_map` macro
- reexport of `typewit`

Added `boolean` module with these items:

- `And` type alias
- `Boolean`
- `contradiction` function
- `IfTrue` type alias
- `IfTruePI` type alias
- `Not` type alias
- `Or` type alias



Added `peano` module with these items:
- `Add` type alias
- `check_le` function
- `contradiction` function
- `eq` function
- `FromPeano` type alias
- `FromUsize` type alias
- `IfZero`  type alias
- `IfZeroPI`  type alias
- `IntoPeano` trait
- `IntoUsize` trait
- `IsLe`  type alias
- `IsLt`  type alias
- `IsZero`  type alias
- `Max`  type alias
- `Min`  type alias
- `Mul`  type alias
- `PeanoInt` trait
- `PeanoWit` (type witness)
- `PlusOne` struct
- `proofs::add_identity` function
- `proofs::commutative_add` function
- `proofs::commutative_mul` function
- `proofs::compose_sub_lt` function
- `proofs::sub_identity` function
- `SubOneSat`  type alias
- `SubSat`  type alias
- `to_usize` function
- `Zero` struct
- `type_fns::AddFn` struct
- `type_fns::IfZeroFn` struct
- `type_fns::IfZeroPIFn` struct
- `type_fns::IsLeFn` struct
- `type_fns::IsLtFn` struct
- `type_fns::IsZeroFn` struct
- `type_fns::MaxFn` struct
- `type_fns::MinFn` struct
- `type_fns::MulFn` struct
- `type_fns::SubOneSatFn` struct
- `type_fns::SubSatFn` struct
- reexports of `::nlist::{Peano, peano}`
- reexports of `::nlist::peano::type_fns::*`
- reexport of `::typewit::const_marker::Usize`

Added `receiver` module with these items: 
- `as_ref` function
- `HktApply` type alias
- `HktOf` type alias
- `HktWit` enum (type witness)
- `MapReceiver` type alias
- `Receiver` trait
- `ReceiverHkt` trait
- `ReceiverWit` enum (type witness)
- `RefHkt` struct
- `RefMutHkt` struct
- `ValueHkt` struct
- `type_fns::MapReceiverFn` struct (`::typewit::TypeFn` implementor)
- reexports of `::nlist::receiver::type_fns::*`


