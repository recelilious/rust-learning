use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    let sn = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", sn);

    loop {
        println!("Guess the number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&sn) {
            Ordering::Less => println!("Small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Big!"),
        }
    }
}
