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

## Reading a File
Now that we have the command line arguments, we need to read the file that the user wants to search. We will use the `std::fs::read_to_string` function to read the file into a string. This function returns a `Result` type. The `Ok` variant indicates the operation was successful, and inside the `Ok` is the resulting value. The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.
Here is an example of how to use it:
```Rust
let contents = fs::read_to_string(file_path)
    .expect("Something went wrong reading the file");
```

## Refactoring to Improve Modularity and Error Handling
To improve modularity we will move all the logic in a different file, `src/lib.rs`. In the `main.rs` file we will only build the config and call the `run` function from the `lib.rs` file.
```Rust
use minigrep::Config;

// ...

fn main() {
    // ...

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };

    // ...
}
```

## Searching for Matches in Text
To implement the search functionality we will iterate the lines of the file, and include them in the returned vector only if they contain the string we are looking for. We will use the `contains` method of the `String` type to check if the line contains the string we are looking for.
```Rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

## Tests
Here's an example of a test:
```Rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

## Other Notes
### Creating a Vec With a Macro
The syntax `vec![1, 2, 3]` allows to create `Vec`s using the same syntax as array expressions. 
Unlike array expressions, this syntax supports all elements which implement `Clone` and the number of elements doesn't have to be a constant.

### The `?` Operator
Chaining results using match can get pretty untidy; luckily, the `?` operator can be used to make things pretty again. `?` is used at the end of an expression returning a `Result`, and is equivalent to a match expression, where the `Err(err)` branch expands to an early `return Err(From::from(err))`, and the `Ok(ok)` branch expands to an `ok` expression.

### Printing Errors to Standard Error
The `println!` macro prints to standard output, which is the terminal by default. The `eprintln!` macro prints to standard error, which is also the terminal by default. The standard error stream is separate from the standard output stream, so the output from `eprintln!` will not be captured if we redirect the output from `println!`.
