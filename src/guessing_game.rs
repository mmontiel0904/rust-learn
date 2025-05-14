use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessing_game() {
    println!("Guess the number");

    let secret_number = rand::rng().random_range(1..=100);

    let chances: i32 = 3;
    let mut attempts: i32 = 0;

    println!("Secret number is {}", secret_number);

    while chances > attempts {
        println!(
            "Please input your guess, you have {} chances",
            chances - attempts
        );

        attempts = attempts + 1;

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = guess.trim().parse().expect("Please type a number");

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
