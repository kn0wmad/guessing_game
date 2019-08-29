use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number");

// create a random number between 0 and 101
// call the gen_range method on the random number generator
// method takes 2 numbers as args and generates a random number between them
    let secret_number = rand::thread_rng().gen_range(0, 101);

// print the secret number (removed for gameplay)
//  println!("The secret number is {}", secret_number);
    loop {
        println!("Please input a guess, between 0 and 100");

    // create mutable variable named guess and bind to a new, empty instance of a String
        let mut guess = String::new();
    // new is an associated "::" function of the String type (Static method)

    // call stdin function from the io module
        io::stdin().read_line(&mut guess)
            .expect("Failed to real line");
    // calls read_line method on the standard input handle to get input from user
    // also passes one argument to read_line - &mut guess
    // read_line takes standard input and places it into a string

    // bind guess to expression guess.trim.parse - trim will remove whitespace
    // u32 can only contain numerical characters
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low, foo"),
            Ordering::Greater => println!("Too high, foo"),
            Ordering::Equal => {
                println!("Fuckin' YAHTZEE!!");
                break;
            }
        }
    }
}
