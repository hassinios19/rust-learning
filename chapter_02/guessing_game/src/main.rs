use std::io;
//input, output

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    //=--> bind something to the variable. On the right of (=) is the result of calling
    //String::new is a function that returns a new instance of a String.
    //::--> indicates that new is an associated function of the String type: Empty string.

    //stdin iis a function from the io module. It allows us to call:
    io::stdin()
        .read_line(&mut guess)
        //if no import of `io` at the beginning: std::io::stdin instead of io:stdin
        //.read_line...--> calls the method read_line. &mut guess gets passed as the argument.
        //&indicates that the argument is a refrence.

        .expect("Failed to read line");
        // Result handler

    println!("You guessed: {guess}");
    //contains user input. {}-->placeholder for value.
}
