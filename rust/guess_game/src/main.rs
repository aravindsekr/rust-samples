
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..11);

    loop {
        // ask for user input
        println!("Please enter your guess. ");

        // create a mutable empty string
        let mut guess = String::new();

        // read and append it to mutable reference,
        // RESULT type which has enum ok and Err and expect will pass the 
        // failure message to err method
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // type convert to number shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        // process the input
        println!("You guessed: {}", guess);

        // match expression with different arms or enums
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
