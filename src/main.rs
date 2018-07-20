extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    // Getting user input

    println!("Guess the Number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    // println!("Secret number is {}", secret_number );
        loop {
            println!("Please input a number:");
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to Read line bro :-(");
            // let guess: u32 = guess.trim().parse().expect("Please type a number1");
            let guess: u32 = match guess.trim().parse() {
                Ok(gs) => gs,
                Err(_) => continue,
            };
            // the :: shit means associated function, type etc
            // an associated functoin is implemented on a type not on an instance object
            println!("You guessed {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Greater => println!("Too Big!"),
                Ordering::Less => println!("Too Small"),
                Ordering::Equal => {
                    println!("You Win!! ;-)");
                    break;
                },
            }
        }

    // Generating secret number
    // rust standard libray has no random number generator so we use a "crate"
    // configure cargo file

}
