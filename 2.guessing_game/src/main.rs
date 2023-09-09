use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {        
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
        
        // Rust allows us to shadow the previous value of guess with a new one.
        // this is to ensure the types are same before comparison function below..
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");   // A. initial approach [expect expression], error handling
        
        // B. [match expression] continue the execution even if data type is wrong, not breaking the execution. 
        let guess: u32 = match guess.trim().parse() { 
            Ok(num) => num,
            Err(_) => continue,
        }; 
        
        match guess.cmp(&secret_number) { // The cmp method compares two values and can be called on anything that can be compared. 
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  // adding break when the user guess correctly, take note need to embed the code in {} 
            }
        }
    }
}