extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("\nTry to guess what is the secret number");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("\nPlease type your answer:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        println!("\nYou typed {}", guess);

        let guess: u8 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small..."),
            Ordering::Equal => {
                println!("Congratulation !");
                break;
            },
            Ordering::Greater => println!("Too big..."),
        }
        println!("\nTry again.\n----------------------");
    }
}
