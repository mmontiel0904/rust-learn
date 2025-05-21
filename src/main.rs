mod functions;

use std::io;

use functions::{ask_run_again, fahrenheit_celcius, guessing_game};

fn main() {
    loop {
        println!("Choose an option");
        println!("1. Play Guessing Game");
        println!("2. Fahrenheit<>Celcius converter");

        println!("0. Exit the program");

        // Another Options
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                guessing_game();
                if ask_run_again() {
                    continue;
                } else {
                    break;
                }
            }
            "2" => {
                fahrenheit_celcius();
                if ask_run_again() {
                    continue;
                } else {
                    break;
                }
            }
            "0" => {
                println!("Exiting the program");
                break;
            }
            _ => println!("Invalid input, please try again"),
        };
    }
}
