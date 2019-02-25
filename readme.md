# slablit

[![Build Status](https://travis-ci.com/myelin-ai/slablit.svg?branch=master)](https://travis-ci.com/myelin-ai/slablit)
[![Latest Version](https://img.shields.io/crates/v/slablit.svg)](https://crates.io/crates/slablit)
[![Documentation](https://docs.rs/slablit/badge.svg)](https://docs.rs/slablit)

Literal for slab creation

```toml
# Cargo.toml
[dependencies]
slablit = "0.3"
```

```rust
use slablit::slab;

let (slab, [first, second, third]) = slab![10, 20, 30];
```
