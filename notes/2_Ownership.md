## Ownership

**Ownership** is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make **memory safety guarantees** *without* needing a *garbage collector*, so it’s important to understand how ownership works.

### Memory and Allocation
In the case of a string literal, we know the contents of the string at compile time and Rust can statically allocate the memory for the string. This is why string literals are fast and efficient. But these properties only come from the string's literal's immutability, we can't put a blob of memory into the binary for each piece of text whose size is unknown at compile time.

With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
 - The memory must be requested from the memory allocator at runtime.
 - We need a way of returning this memory to the allocator when we're done with the string.

The first part is don when we call `String::from`, its implementation requests the memory it needs. This is pretty much universal in programming languages.

Howevere, the second part is different in Rust. 
In languages with a *garbage collector* (*GC*), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
Example:
```Rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
} // this scope is now over, s is dropped and the memory is returned to the allocator
```
There is a natural point at which we can return the memory: when `s` goes out of scope. When a variable goes out of scope, Rust calls a special function called `drop` for us. In this function the author of the type can put the code to return the memory.
Rust calls `drop` automatically at the closing curly bracket.

> **_Note_**: This pattern is sometimes called *Resource Acquisition Is Initialization* in C++, or *RAII* for short. It's a design pattern in which some resource is allocated when a object is created, and released when the object is destroyed.

This pattern has a profound impact on the way Rust code is written.

### Ways Variables and Data Interact: Move
Consider the following code:
```Rust
let s1 = String::from("hello");
let s2 = s1;
```
After `let s2 = s1;` Rust considers `s1` as _no longer valid_. Indeed you cannot use `s1` anymore, and the memory it was pointing to is freed when `s2` goes out of scope. This prevents the ambiguity of having two pointers pointing to the same memory, which may lead to a *double free* error.

If you’ve heard the terms _shallow copy_ and _deep copy_ while working with other languages, the concept of copying the pointer without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of calling it a shallow copy, it’s known as a _move_.

In addition, there's a design choice that's implied by this: Rust will _never_ automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

### Ways Variables and Data Interact: Clone
If we do want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`. This will do a full copy of all the data on the heap for the `String`, and create a new `String` on the heap. This is a deep copy, and it's slower than a shallow copy.
Here's an example:
```Rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

When you see a call to `clone`, you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.

### Stack-Only Data: Copy
There’s another wrinkle we haven’t talked about yet. Take a look at this code:
```Rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```
this code seems to contradict what we just learned: we don’t have a call to `clone`, but `x` is still valid and wasn’t moved into `y`.

The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent `x` from being valid after we create the variable `y`. In other words, there’s no difference between deep and shallow copying here, so calling `clone` wouldn’t do anything different from the usual shallow copying and we can leave it out.

Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack, as integers are. If a type implements the `Copy` trait, variables that use it do not *move*, but rather are trivially copied, making them *still* valid after assignment to another variable.

Rust won’t let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait.

### Ownership and Functions
Let's see how ownership works with functions:
```Rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

If we tried to use `s` after the call to `takes_ownership`, Rust would throw a compile-time error. These static checks protect us from mistakes.

### Return Values and Scope
Let's see how ownership works with functions that return values:
```Rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```
The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again.

Luckily for us, Rust has a feature for using a value without transferring ownership, called *references*.

### References and Borrowing
