# Changelog
## 0.3.1
* Updated metadata.
* Simplified creation of empty slabs:
  ```diff
  - let (slab, []): (slab::Slab<i32>, [usize; 0]) = slab![];
  + let (slab, []): (slab::Slab<i32>, _) = slab![];
  ```

## 0.3.0
- Nightly is no longer required
