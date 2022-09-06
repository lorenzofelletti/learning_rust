## Structs
A *struct*, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

### Defining Structs
Here's a simple example of a struct definition that represents a user account on a website:
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

To use a struct after we’ve defined it, we create an *instance* of that struct by specifying concrete values for each of the fields:
```Rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

To get (or set if the struct is `mut`) a specific value from a struct, we can use dot notation:
```Rust
let email = user1.email;
```
