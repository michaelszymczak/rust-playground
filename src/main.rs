use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater};
use std::io;

use rand;
use rand::Rng;

fn main() {
    let lower_bound = 1;
    let upper_bound = 20;
    let secret_number: u32 = rand::thread_rng().gen_range(lower_bound..(upper_bound + 1));

    loop {
        let mut guess = String::new();
        println!(
            "Guess a number that is between {} and {}",
            lower_bound, upper_bound
        );
        io::stdin().read_line(&mut guess).expect("error");

        let ordering = compare(&secret_number, &guess);
        if ordering == Equal {
            println!("correct");
            break;
        } else if ordering == Greater {
            println!("too high");
        } else {
            println!("too low");
        }
    }
}

fn compare(secret_number: &u32, guess: &String) -> Ordering {
    let guess: u32 = match guess.trim().parse() {
        Ok(number) => number,
        Err(_) => 0,
    };
    guess.cmp(&secret_number)
}
