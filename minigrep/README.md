## Minigrep
In this project, we will create a command line tool that searches for a string in a file and prints all lines that contain that string, and we will call it *minigrep*.

### Accepting Command Line Arguments
First, we need a way to read the arguments that the user passes to the program. We will use the `std::env::args` function to get the command line arguments. This function returns an iterator of the command line arguments. We can use the `collect` method on the iterator to turn it into a data structure that we can use. We will use a `Vec` of `String`s, which is the most common collection of command line arguments.
```Rust
use std::env;

fun main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```
In Rust isn't often that we need to annotate the type of a variable, but in this case, we do to help the `collect` method figure out what kind of collection we want. If we don't annotate the type, we'll get a type error that the `collect` method doesn't know what kind of collection we want.
