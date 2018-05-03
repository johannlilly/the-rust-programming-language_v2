// https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html
use std::io; // standard library

fn main() { // entry
    println!("Guess the number!"); // ! macro

    println!("Please input your guess.");

    let mut guess = String::new(); // store user input. let is used to create variables. mut = mutable
    //std::io::stdin
    io::stdin().read_line(&mut guess) // read a mutable (mut) reference (&) of the line previously stored as mutable variable guess
    	.expect("Failed to read line"); // error handling if this instance of io::Result is an Err value. By using expect not writing the error handling, the program crashes upon error. For this program, we want that.

    println!("You guessed: {}", guess);
}
