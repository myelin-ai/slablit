# slablit

> ⚠️ This crate requires the nightly compiler

Literal for slab creation

```rust
#![feature(decl_macro, macro_at_most_once_rep)]

use slablit::slab;

let (slab, [first, second, third]) = slab![10, 20, 30];
```
