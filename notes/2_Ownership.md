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
