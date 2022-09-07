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

