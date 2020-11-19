use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn get_secret() -> u32 {
    return rand::thread_rng().gen_range(1, 101);
}

fn get_guess() -> u32 {
    loop {
        println!("Enter your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("We both know that {} isn't a number", guess);
                continue;
            },
        };

        return guess;
    }
}

fn main() {
    println!("Welcome to guess the number!!");

    let secret_value = get_secret();
    println!("The secret is: {}", secret_value);

    loop {
        let guess = get_guess();

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_value) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Great work!");
                break;
            }
        }
    }
}
