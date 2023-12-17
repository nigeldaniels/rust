extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {


    println!("Guess the number!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1,101);
        let mut guess = String::new();

        println!("Please input your guess.");
        println!("The secret number is {}", secret_number);

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a number");
                continue
            },
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small "),
            Ordering::Greater => println!("too big"),
            Ordering::Equal =>  {
                println!("you win");
                break;
            },
        }


        println!("the actual number was: {}", secret_number)
    }
}
