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
