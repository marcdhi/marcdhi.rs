use std::io; // The io library comes from the standard library, known as std: we will be using that here to read and write input outputs
use rand::Rng; // Library to generate a random number

use std::cmp::Ordering;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is, {secret_number}");

    loop {
        
        println!("Please input your guess: ");
    
        let mut guess = String::new(); //creates a new String instance
    
        //receiving user input
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read that!");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed this : {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {println!("You win");
                                break;}
        }
    }


}
