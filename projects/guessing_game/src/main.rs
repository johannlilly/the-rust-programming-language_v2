extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    	.expect("Failed to read line");

    let guess: u32 = guess.trim().parse() // "shadow" the previous value of guess with a new one
    	.expect("Please type a number!"); // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
    	Ordering::Less => println!("Too small!"),
    	Ordering::Greater => println!("Too big!"),
    	Ordering::Equal => println!("You win!"),
    }
}
