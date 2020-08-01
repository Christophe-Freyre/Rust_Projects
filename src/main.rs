use std::io; // import the input/output library in the prelude.

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // mut allows to define mutable variable. In Rust variables are immutable by default.
// empty string. Associated function implemented on a type rather than on a particular instance of a String; some langugages call this a static method.
    io::stdin() // can be written as std::io::stdin
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}