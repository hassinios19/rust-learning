# Guessing Game

**How to Process a Guess?**

Ask for input, process it, check comparison to expected form.

``use std::io;``: Rust has a set of items defined in the standart library that it brings into the scope of every program. Set = prelude.
If type is not in prelude--> bring it into scope with use statement. std::io library provides a number of useful features for example to accept a user input.

`main`: Entry point into the program.

**Storing values with variables**
`let mut guess = String::new()`: Let statement to create the variable.

Other example `let phones = 6;`--> In Rust variables are immutable by default.

With `mut`: The variable becomes mutable.

```
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```
the result would be 5 and 2 = 12

Rust does not include a random number functionality in it's standard library yet.
In Cargo.toml we can includee the rand crate which is a library that contains code intended too be used in ooother progrmas and can't be executed on it's own.
The version should be the latest in best cases like 0.10.3