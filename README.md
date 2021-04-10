[![Rust](https://github.com/DevNulPavel/cow_arc/actions/workflows/rust.yml/badge.svg)](https://github.com/DevNulPavel/cow_arc/actions/workflows/rust.yml)

# Purposes 

CowArc can be useful for decreasing memory allocations by sharing immutable memory.

It saves some RAM by sharing immutable values between CowArc clones.
Memory allocates only in case of changing value.

CowArc can be usefull for creating builders.

# Examples

```rust
let source = CowArc::new(vec![1, 2, 3]);

// Still shared memory
let mut changed = cloned.clone();
assert!(std::ptr::eq(source.deref(), changed.deref()) == true);
assert!(changed.eq(&vec![1, 2, 3]));

// New memory allocation
changed.set_val(vec![1, 2, 3, 4]);
assert!(std::ptr::eq(source.deref(), changed.deref()) == false);
assert!(changed.eq(&vec![1, 2, 3, 4]));
```

```rust
let source = CowArc::new(vec![1, 2, 3]);

// Still shared memory
let mut updated = source.clone();
assert!(std::ptr::eq(source.deref(), updated.deref()) == true);
assert!(changed.eq(&vec![1, 2, 3]));

// New memory allocation
updated.update_val(|val|{
        val.push(4);
});
assert!(std::ptr::eq(source.deref(), updated.deref()) == false);
assert!(updated.eq(&vec![1, 2, 3, 4]));
```
