use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("*** Guess the number ***");

    let secret_number = rand::thread_rng().gen_range(1..20);
    // println!("Secret number is: {}", secret_number);

    loop {
        println!("Type a number!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line!");

        // println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Please select a number! = {}", err);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("*** You win ***");
                break;
            }
        }
    }
}
