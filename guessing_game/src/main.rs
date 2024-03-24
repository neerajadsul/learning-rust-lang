use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=1000);
    // println!("Secret Number is: {}", secret_number);
    let mut attempts: u32 = 0;
    loop {
        println!("Please input your guess: (enter q to cede)");
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        if guess.trim().eq_ignore_ascii_case("q") {
            println!("You lose, number was: {}", secret_number);
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        attempts += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You WIN in {} attempts.", attempts);
                break;
            }
        }
    }

}
