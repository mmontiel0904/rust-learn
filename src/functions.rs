use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessing_game() {
    println!("Guess the number");

    let mut won = false;
    let mut attempts: i32 = 0;
    let chances: i32 = 10;
    let secret_number = rand::rng().random_range(1..=100);

    //println!("Secret number is {}", secret_number);

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

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("I cant understand wha you type, try again, you loose one attempt 👆");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                won = true;
                break;
            }
        }
    }

    if won {
        println!("You won 🙌, the correct number is {}", secret_number);
    } else {
        println!("You loose 🥺, the correct number is {}", secret_number);
    }
}

pub fn ask_run_again() -> bool {
    println!("Run another function? Y/N");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    match choice.trim() {
        "Y" => println!("Your input was 'Yes', loading options..."),
        "N" => println!("Your input was 'No' program will finilize now"),
        _ => println!("Unknown input: program will finilize now"),
    }

    choice.trim().eq_ignore_ascii_case("Y")
}
