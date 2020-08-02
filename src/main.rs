use rand::Rng;
use std::cmp::Ordering;
use std::io; // import the input/output library in the prelude.


fn main() {
    println!("Guess the number!");
    let mut count = 0u32;
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mut allows to define mutable variable. In Rust variables are immutable by default.
// empty string. Associated function implemented on a type rather than on a particular instance of a String; some langugages call this a static method.
        io::stdin() // can be written as std::io::stdin
            .read_line(&mut guess)
            .expect("Failed to read line");
        count += 1;
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small! Try again!"),
            Ordering::Greater => println!("Too big! Try again!"),
            Ordering::Equal => {
                if count ==  1 {
                    println!("You win after {} attempt. AMAZING!", count);
                    break;
                } else {
                println!("You win after {} attempts!", count);
                break;
            }
            }
        }
    }
}