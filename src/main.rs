use std::io::{stdin};
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Halu! This is a guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret Number: {}", secret_number);

    loop {
        println!("Please, type a number: ");
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess input");
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                continue; 
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed low!"),
            Ordering::Greater => println!("You guessed high!"),
            Ordering::Equal => {
                println!("You guessed CORRECTLY!");
                println!("Do you want to play again? (y/n)");
                
                let mut repeat = String::new();
                stdin()
                    .read_line(&mut repeat)
                    .expect("Failed to read repeat input");
                
                let repeat = match repeat.as_str().trim() {
                    "y" => true,
                    "n" => false,
                    _ => panic!("unexpected answer")
                };
                println!("Repeat: {}", repeat);
                match repeat {
                    true => {
                        println!("Hello again!");
                        main()
                    }
                    false => {
                        println!("Halu! Thank you for playing!");
                        break;
                    }
                }
                
            }
        
        }
    }
    
}
