## Variables and Mutability
In Rust, variables are immutable by default.
```Rust
let x = 5;
x = 6;
```
The above code will not compile. To make it compile, we need to make the variable `x` mutable using the `mut` keyword (`let mut x = 5;`).

The Rust compiler guarantees that when you state a value won’t change, it really won’t change, so you don’t have to keep track of it yourself. Your code is thus easier to reason through.
Adding mut also conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable’s value.

### Constants
Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.
Constants aren't just immutable by default—they're always immutable.

You declare constants using the `const` keyword, and the type of the value **must** be annotated.

Constants can be declared in any scope, including the global scope. That makes the useful for values that many parts of the program need to know.

Here's an example of a constant declaration:
```Rust
const MAX_POINTS: u32 = 100_000;
```

Rust’s naming convention for constants is to use all uppercase with underscores between words.

Constants are valid for the entire time a program runs, within the scope they were declared in.

### Shadowing
Shadowing is the act of redeclaring a variable with a different type.

Rustaceans say that the first variable is *shadowed* by the second, which means that the second variable is what the compiler will see when you use the name of the variable.
In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
We can shadow a variable by using the same variable’s name and repeating the use of the `let` keyword as follows:
```Rust
let x = 5;
let x = x + 1;
{
    let x = x * 2;
    println!("inner x: {}", x);
}
println!("outer x: {}", x);
```
This program first binds `x` to a value of 5. Then it creates a new variable `x` by repeating `let x =`, taking the original value and adding 1 so the value of `x` is then 6. Then, within an *inner* scope created with the curly brackets, the third `let` statement also shadows `x` and creates a new variable, multiplying the previous value by 2 to give `x` a value of 12. When *that scope is over, the inner shadowing ends* and `x` returns to being 6.

Thus when run, the above code will print `inner x: 12` and `outer x: 6`.

Shadowing is different from marking a variable as `mut`, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.

The other difference between `mut` and shadowing is that because we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name.

## Data Types
Every value in Rust is of a certain *data type*, which tells Rust what kind of data is being specified so it knows how to work with that data. We’ll look at two data type subsets: scalar and compound.

Rust is a *statically typed* language, which means that the compiler must know the type of every value at compile time.
In many cases, Rust will infer the type of a value from its context, but sometimes we must add type annotations to tell the compiler what type a value will have.

### Scalar Types
A **scalar** type represents a single value. Rust has four primary scalar types:
- integers
- floating-point numbers
- booleans
- characters.

#### Integers
Integers can be either signed or unsigned, and are usually either `i8`, `i16`, `i32`, `i64`, `isize`, `u8`, `u16`, `u32`, `u64`, `usize`.

### Floating-Point Numbers
Floating-point numbers are represented by the `f32` and `f64` types.

### Compound Types
*Compound types* can group multiple values together into one type. Rust has two primitive compound types:
- tuples
- arrays.

### Tuples
Tuples are a general way of grouping together a number of values with a variety of types into one compound type.
Tuples have fixed length: once declared, they can’t grow or shrink in size.
Example of how to declare a tuple:
```Rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```
The variable `tup` binds to the entire tuple, because a tuple is considered a single compound element.

To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
```Rust
let (x, y, z) = tup;

println!("The value of y is: {}", y);
```
The above code creates a tuple and binds it to the variable tup. It then uses a pattern with `let` to take tup and turn it into three separate variables, `x`, `y`, and `z`. This is called **destructuring**.

We can also access a tuple element directly by using a period (`.`) followed by the index of the value we want to access. For example:
```Rust
let five_hundred = tup.0;

let six_point_four = tup.1;

let one = tup.2;
```

The tuple without any values has a special name, *unit*. This value and its corresponding type are both written `()` and represent an *empty value* or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

### Arrays

## Functions

## Control Flow

## Comments
