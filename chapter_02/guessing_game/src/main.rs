use std::cmp::Ordering;
//use statement. Ordering = another enum, has variants Less, Greater, Equal
use std::io;
//input, output
use rand::prelude::*;
//module that contains most commonly used parts of rand crate.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
    //rand::rng() gives us the particular random nr generator. random_range is defined by RngExt and part of rand::prelude.
    //1..=100 is a range that includes both ends (1 and 100).

    //loop repeats forever until a `break` (on a win) stops it.
loop {
    println!("Please input your guess.");

    let mut guess = String::new();
    //= --> binds the value on the right to the variable. Here the value is the result of calling
    //String::new is a function that returns a new instance of a String.
    //::--> indicates that new is an associated function of the String type. It returns an empty string.

    //stdin is a function from the io module. It returns a handle to the terminal input, on which we call:
    io::stdin()
        .read_line(&mut guess)
        //if no import of `io` at the beginning: std::io::stdin instead of io::stdin
        //.read_line...--> calls the method read_line. &mut guess gets passed as the argument. It appends the input to guess.
        //& indicates that the argument is a reference. References are immutable by default, so we need &mut to change guess.

        .expect("Failed to read line");
    //read_line returns a Result (Ok or Err). expect() crashes with this message on Err, on Ok it returns the value inside.

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    // shadowing: new "guess" (u32) replaces the old String version
    // trim() removes whitespace + \n from Enter, parse() converts text -> number (returns Result)
    //: u32 tells parse() which number type. match on the Result: Ok(num) -> use the number,
    //Err(_) -> `_` catches any error, continue skips to the next loop round (no crash on bad input).

    println!("You guessed: {guess}");
    //{guess} contains the user input. {}-->placeholder for value.

    //cmp compares guess with secret_number and returns an Ordering. match runs the arm that fits.
    //Both sides must be the same type, so Rust infers secret_number as u32 too.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
            //break exits the loop, so the program ends after a win.
        }
        }

    }
}
