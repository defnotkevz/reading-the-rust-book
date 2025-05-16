use std::io; // it is a crate
use rand::Rng;
use std::cmp::Ordering;

//yo, i'm a comment.
/* I am also
 * a better comment */

fn main() {
    println!("Guess the number");

    let mut guess = String::new(); 

    let secret_number = rand::rng().random_range(1..=100);

    println!("Please input your guess:");

    io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
