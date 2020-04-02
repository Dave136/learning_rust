// Require libs
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// Main function
fn main() {

    println!("*** Guess the number! ***");

    // Generate random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Print The secret number, interpolation with "{}"
    // println!("The secret number is: {}", secret_number);

    // Loop as While
    loop {

        println!("Please input your guess: ");

        // Varible declaration
        // let declaration like javascript 
        // mut ---> mutable variable
        //  String::new(), static method to generate new string
        let mut guess = String::new();

        // .read_line() read the input
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // Show error if this exists

        // Pass value and declaration of type u32 (unsigned 32-bit number)
        // match return a value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Good response
            Err(_) => continue, // Error
        };

        println!("You guessed: {}", guess);

        // Comparation
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// Extract this game sample from:
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html