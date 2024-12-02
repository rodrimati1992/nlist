[![Rust](https://github.com/rodrimati1992/nlist/workflows/Rust/badge.svg)](https://github.com/rodrimati1992/nlist/actions)
[![crates-io](https://img.shields.io/crates/v/nlist.svg)](https://crates.io/crates/nlist)
[![api-docs](https://docs.rs/nlist/badge.svg)](https://docs.rs/nlist/*)


Provides an [inline-allocated list] which statically tracks its length,
and type-based [integer]/[boolean] representations which 
don't require (additional) bounds for operators.

# Example

### Splitting and recombining

This example shows how [`NList`]s can be split and recombined in const,
even if the length is generic, 
so long as the length is known to be greater than the split index.

```rust
use nlist::{NList, Peano, PeanoInt, nlist, peano};

const LIST: NList<u128, Peano!(7)> = transform(nlist![3, 5, 8, 13, 21, 34, 55]);

assert_eq!(LIST, nlist![21, 34, 55, 103, 105, 108, 113]);

type SplitIndex = Peano!(4);

const fn transform<L>(
    list: NList<u128, peano::Add<SplitIndex, L>>,
) -> NList<u128, peano::Add<SplitIndex, L>>
where
    L: PeanoInt,
{
    // if we use `let` to destructure instead of `konst::destructure`,
    // we get a "destructor cannot be evaluated at compile-time" error as of Rust 1.83
    konst::destructure!{(before, after) = list.split_at::<SplitIndex>()}
    
    // math spice: using arithmetic properties to coerce equal generic lengths.
    // 
    // Alternatively, you can pass  `peano::eq().unwrap_eq()` to `coerce_len`
    // for an easier, but panic prone, approach:
    // ```
    // return after.concat(map_add_100(before)).coerce_len(peano::eq().unwrap_eq())
    // ```
    // 
    // coercing `NList<u128, L - 0>` to `NList<u128, L>`
    let coerced_after = after.coerce_len(peano::proofs::sub_identity::<L>());

    coerced_after.concat(map_add_100(before))
        // coercing `NList<u128, L + SplitIndex>` to `NList<u128, SplitIndex + L>`
        .coerce_len(peano::proofs::commutative_add::<L, SplitIndex>())
}

// Adds 100 to all elemenst of an NList
const fn map_add_100<L: PeanoInt>(list: NList<u128, L>) -> NList<u128, L> {
    nlist::rec_map!(list, |elem, rest| (elem + 100, map_add_100(rest)))
}
```

# Crate features

- `"alloc"`(enabled by default): enables methods that take or return [`Vec`] 

# No-std support

`nlist` is `#![no_std]`, it can be used anywhere Rust can be used.

# Minimum Supported Rust Version

`nlist` requires Rust 1.83.0.


[inline-allocated list]: https://docs.rs/nlist/latest/nlist/nlist/struct.NList.html  
[`NList`]: https://docs.rs/nlist/latest/nlist/nlist/struct.NList.html  
[integer]: https://docs.rs/nlist/latest/nlist/peano/trait.PeanoInt.html  
[boolean]: https://docs.rs/nlist/latest/nlist/boolean/trait.Boolean.html  
[`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html