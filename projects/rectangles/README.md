## Rectangles

### Using Derived Traits
TO print an instance of `Rectangle` while we're debugging we should implement the `Debug` trait:
```Rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

### Self Parameter Shortcut
In the signature for a structure's methods `&self` is a shorthand for `self: &Self`. Whithin an `impl` block, the type `Self` is the type that the `impl` block is for.
