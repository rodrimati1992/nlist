Provides an [inline-allocated list] which statically tracks its length.

# Example

### Splitting and recombining

This example shows how NLists can be split and recombined in const,
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
    konst::destructure!{(mut before, after) = list.split_at::<SplitIndex>()}
    
    let mut array = before.into_array();
    mutate_array(&mut array);
    before = NList::from_array(array);
    
    // math spice: using arithmetic properties to coerce equal generic lengths.
    // 
    // Alternatively, you can pass  `peano::eq().unwrap_eq()` to `coerce_len`
    // for an easier, but panic prone, approach:
    // ```
    // return after.concat(before).coerce_len(peano::eq().unwrap_eq())
    // ```
    // 
    // coercing `NList<u128, L - 0>` to `NList<u128, L>`
    let coerced_after = after.coerce_len(peano::proofs::sub_identity::<L>());

    coerced_after.concat(before)
        // coercing `NList<u128, L + SplitIndex>` to `NList<u128, SplitIndex + L>`
        .coerce_len(peano::proofs::commutative_add::<L, SplitIndex>())
}

const fn mutate_array(array: &mut [u128; SplitIndex::USIZE]) {
    *array = konst::array::map_!(*array, |x| 100 + x);
}
```

# Crate features

- `"alloc"`(enabled by default): enables methods that take or return [`Vec`] 

# No-std support

`nlist` is `#![no_std]`, it can be used anywhere Rust can be used.

# Minimum Supported Rust Version

`nlist` requires Rust 1.83.0.


[inline-allocated list]: crate::NList
[`NList`]: crate::NList



[inline-allocated list]: https://docs.rs/nlist/latest/nlist/nlist/struct.NList.html  
[NList]: https://docs.rs/nlist/latest/nlist/nlist/struct.NList.html  
[`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html