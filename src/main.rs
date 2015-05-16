extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Enter your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read input");

        let guess: u32 = guess.trim().parse()
            .ok()
            .expect("Failed to convert");

        println!("Woah I have the guess! {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Equal => println!("You win!"),
            Ordering::Greater => println!("Too Big")
        }

    }
}
