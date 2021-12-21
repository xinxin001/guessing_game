use std::io;
use rand::Rng;
use std::cmp::Ordering;

//Simple guessing the number game from the Rust Programming Language book
fn main() {
    println!("Guess the number");

    //Generate number from 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..101);

    //Keep looping until correct guess
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        //Read user input with error handling
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        //Parse the input into integer
        //Reset loop if input cannot be parsed into integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        
        //Match the secret_number with guess
        //Give hints and end program when guess is correct
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
