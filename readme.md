# slablit

[![Latest Version](https://img.shields.io/crates/v/slablit.svg)](https://crates.io/crates/slablit)
[![Documentation](https://docs.rs/slablit/badge.svg)](https://docs.rs/slablit)
[![dependency status](https://deps.rs/repo/github/myelin-ai/slablit/status.svg)](https://deps.rs/repo/github/myelin-ai/slablit)

Literal for slab creation

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
slablit = "0.3"
```

## Example

```rust
use slablit::slab;

let (slab, [first, second, third]) = slab![10, 20, 30];
```
