# Guessing Game

**How to Process a Guess?**

Ask for input, process it, and check that the input is in the expected form.

`use std::io;`: Rust has a set of items defined in the standard library that it brings into the scope of every program. Set = prelude.
If type is not in prelude--> bring it into scope with use statement. The io module of the standard library (std::io) provides a number of useful features for example to accept a user input.

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
the result would be x = 5 and y + 2 = 12

Rust does not include a random number functionality in its standard library yet.
In Cargo.toml we can include the rand crate which is a library crate: it contains code intended to be used in other programs and can't be executed on its own. Our project is a binary crate (an executable).
The version should be the latest in best cases like 0.10.3. `cargo build` then downloads rand and its dependencies from crates.io.

**Updating a Crate to Get a New Version**

Cargo.lock freezes versions. Once a version is picked, every build uses exactly that one, even if newer versions come out. That keeps builds reproducible.
cargo update picks again. It ignores Cargo.lock, takes the newest version that still fits the range in Cargo.toml, and writes it into Cargo.lock.
The range: rand = "0.10.1" means ≥ 0.10.1 and < 0.11.0. A new 0.10.2 would be picked, a new 0.999.0 would be ignored.
Bigger jumps happen only by hand. To use a version outside the range (e.g. 0.999), you have to change the number in Cargo.toml yourself. Cargo won’t do it automatically, because big version jumps can break your code.

**Comparing the Guess to the Secret number**

Match --> expression made up of arms.  An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.

`guess.cmp(&secret_number)` compares two values and returns an `Ordering` (enum with variants `Less`, `Greater`, `Equal`). The match checks the arms in order and runs the first one that fits.
Both values must have the same type, so Rust infers `secret_number` as `u32` from the comparison with `guess`.

**Handling errors with Result**

`read_line` and `parse` return a `Result`, an enum with variants `Ok` (success, holds the value) and `Err` (failure, holds the error).
`.expect("...")` crashes the program with that message on `Err`, and returns the value on `Ok`.

**Converting the input: shadowing, trim, parse**

`let guess: u32 = ...` creates a new `guess` that shadows (replaces) the old String one, so we can reuse the name.
`trim()` removes whitespace and the newline from pressing Enter. `parse()` converts the text into a number; `: u32` tells it which type.

**Looping: loop, break, continue**

`loop` repeats forever. `break` exits the loop (after a win). `continue` skips to the next round.
`Err(_) => continue` ignores invalid input instead of crashing: `_` is a catch-all that matches any error.

