## Intro
In the following paragraphs we will explain the implementation of the guessing game.

## Cargo
Cargo is Rust’s build system and package manager.

To check cargo version: `cargo --version`.

## Creating a Project with Cargo
Run the following:
```Bash
$ cargo new hello_cargo
$ cd hello_cargo
```

This creates a new directory named `hello_cargo` containg two files: `Cargo.toml` and `main.rs` inside.
It has also initialized a new Git repository along with a `.gitignore` file; you can override this behavior using the `--vcs` flag (use `cargo new--help` to learn more).

`Cargo.toml` is a TOML (Tom's Obvious, Minimal Language) file used to configure Cargo.

The `[package]` section indicates that the following statements are configuring a package.

The `[dependencies]` section marks the start of the section in which you can list all the project's dependencies.

## Building and Running a Cargo Project
### Cargo Build
To build a cargo project run the following:
```Bash
$ cargo build
```
This command creates an executable file in `target/debug/hello_cargo` rather than in your current directory.

Running `cargo build` for the first time also causes Cargo to create a new file at the top level: `Cargo.lock`. This file keeps track of the exact versions of dependencies in your project.

### Cargo Run
We just built a project with `cargo build` and ran it with `./target/debug/hello_cargo`, but we can also use `cargo run` to compile the code and then run the resulting executable all in one command.

### Cargo Check
Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable.

Often, `cargo check` is much faster than `cargo build`, because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using check will speed up the process of letting you know if your project is still compiling.

### Building for Release
When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in `target/release` instead of `target/debug`. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile.

### Work on an Existing Project
To work on any existing projects, you can use the following commands to check out the code using Git, change to that project’s directory, and build:

```Bash
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

## Generating a Random Number
### Using a crate to Get More Functionality
To generate a random number between 1 and 100, we can use the `rand` crate.

In `Cargo.toml` we can add the following dependency:
```toml
[dependencies]
rand = "0.8.3"
```
The number 0.8.3 means that any version of `rand` that is 0.8.3 or higher, but below 0.9.0, can be used.

Cargo considers these versions to have public APIs compatible with version 0.8.3, and this specification ensures you’ll get the latest patch release that will still compile with the code. Any version 0.9.0 or greater is not guaranteed to have the same API as what the following examples use.

### Generating a Random Number With the Rand Crate
```Rust
use rand::Rng;
// --snip--
let secret_number = rand::thread_rng().gen_range(1..=100)
```
First we import the `Rng` trait from the `rand` crate.

Then we use the `thread_rng` method to get a reference to a random number generator (that is local to the current thread of execution and seeded by the OS).

Then we use the `gen_range` method to get a random number between 1 and 100. This method takes a range expression as an argument, and returns a random number between the start and end of the range. The kind of range expression we’re using here takes the form `start..=end` and is inclusive on the lower and upper bounds

### Parsing and Comparing the Guess and the Secret Number
```Rust
let guess: u32 = match guess.trim().parse().expect("Please enter a number!");
```
The above line parses the guess as a u32. If the parse fails, we print an error message and return early.

The `parse` method on strings converts a string to another type. Here, we use it to convert from a string to a number. We need to tell Rust the exact number type we want by using `let guess: u32`. The colon (`:`) after guess tells Rust we’ll annotate the variable’s type.

To compare the guess and the secret number, we use the match expression.
First, we need to import the `std::cmp::Ordering` enum.

The `Ordering` type is an enum that has the variants `Less`, `Equal`, and `Greater`.

Then, we can write the following:
```Rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!");
}
```
We use a match expression to decide what to do next based on which variant of the `Ordering` enum was returned from the call to `cmp`.

A `match` expression is made up of *arms*. An arm consists of a *pattern* to match against, and a *block* of code to run if the value given to `match` fits that arm’s pattern.

## Looping
The `loop` keyword creates an infinite loop.
```Rust
loop {
    println!("Please input your guess.");

    // --snip--

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}
```
