## Enums
Enums allow you to define a type by enumerating its possible variants.

### Defining an Enum
```Rust
enum IpAddrKind {
    V4,
    V6,
}
```
`IpAddrKind` is now a custom type that we can use elsewhere in our code.

### Enum Values
We can create instances of each variant of `IpAddrKind`:
```Rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

### Defining an Enum with Data
We can put data directly into each enum variant:
```Rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```
