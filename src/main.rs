use std::io;
use std::cmp::Ordering;
use rand::Rng;

/**
 * Guessing game in rust
 */
fn main() {
    println!("Guess the Number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    println!("You guess: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You Win!"),
        Ordering::Greater => println!("Too big!"),
    }
}
