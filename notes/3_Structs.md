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
// get
let email = user1.email;

// set (if user1 is mut)
user1.email = String::from("new@mail.com");
```

An instance's fields must be either all mutable or all immutable.

### Instantiating Structs with a Function
We can define a function that takes a struct as an argument and returns an instance of that struct:
```Rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

### String Update Syntax
We can use the shorthand syntax to update a struct's fields if we want to change only some of the fields:
```Rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

### Tuple Structs
A `tuple struct` is a variation of the struct that look similar to tuples, but has the added meaning the struct name provides but doesn't have named fields to help with clarity. Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples.
Example:
```Rust
struct Color(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
}
```

### Unit-Like Structs
A `unit-like struct` is a struct that doesn't have any fields. Unit-like structs can be useful in situations in which you need to implement a trait on some type but don't have any data that you want to store in the type itself. Unit-like structs are useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.
Example:
```Rust
struct Unit;

fn main() {
    let unit = Unit;
}
```

### Ownership of Struct Data
It's possible for structs to store references, but to do so requires the use of *lifetimes*. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

### Methods
#### Defining Methods
To define a method on a struct, we put the `impl` keyword and then the name of the struct. Inside the `impl` block is where we define the methods, which have the same syntax as functions.

### Automatic Referencing and Dereferencing
Rust does dereferencing automatically when we call methods with `&self` or `&mut self` as a parameter. This means we don't have to write `&` or `&mut` before `self` when we call methods. This feature is called *automatic referencing and dereferencing*.

### Associated Functions
Associated functions are similar to static methods in other languages. They're associated with a struct, but don't have access to the struct's data. They're still in the `impl` block, but they don't take `self` as a parameter. Associated functions are often used for constructors that will return a new instance of the struct.
