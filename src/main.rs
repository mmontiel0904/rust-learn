mod functions;

use std::io;

use functions::{
    ask_run_again, fahrenheit_celcius, fibonaizer, guessing_game, twelve_days_of_christmas,
};

fn main() {
    loop {
        println!("Choose an option");
        println!("1. Play Guessing Game ⁉️");
        println!("2. Fahrenheit<>Celcius converter 🌡️");
        println!("3. Get the nth Fibonacci number 🔢");
        println!("4. Signme 12 days of chirstmas ❄️");
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
            "3" => {
                fibonaizer();
                if ask_run_again() {
                    continue;
                } else {
                    break;
                }
            }
            "4" => {
                twelve_days_of_christmas();
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
