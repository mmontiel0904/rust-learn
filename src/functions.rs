use rand::Rng;
use std::cmp::Ordering;
use std::io::{self};

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

pub fn fahrenheit_celcius() {
    fn from_f_deg() {
        let f_deg: f64;

        println!("Please type a Farenhit degrees to convert");

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            match input.trim().parse::<f64>() {
                Ok(num) => {
                    f_deg = num;
                    break;
                }
                Err(_) => {
                    println!("Failed to read a number, please try again");
                    continue;
                }
            }
        }

        let c_deg = (f_deg - 32.00) * (5.00 / 9.00);

        println!("{f_deg} fahrenheit is equal to {c_deg} celcius degrees")
    }

    fn from_c_deg() {
        let c_deg: f64;

        println!("Please type a Celcius degrees to convert");

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            match input.trim().parse::<f64>() {
                Ok(num) => {
                    c_deg = num;
                    break;
                }
                Err(_) => {
                    println!("Failed to read a number, please try again");
                    continue;
                }
            }
        }

        let f_deg = (c_deg * (9.00 / 5.00)) + 32.00;

        println!("{c_deg} celcius is equal to {f_deg} fahrenheit degrees")
    }

    loop {
        let mut convertion_type = String::new();

        println!("Please select the conversion Type");
        println!("'1' for convert Celcius to fahrenheit");
        println!("'2' for fahrenheit to Celcius");
        io::stdin()
            .read_line(&mut convertion_type)
            .expect("Failed to read the input");

        match convertion_type.trim() {
            "1" => {
                from_c_deg();
                break;
            }
            "2" => {
                from_f_deg();
                break;
            }
            _ => {
                println!("Wrong input, try again");
                continue;
            }
        }
    }
}
