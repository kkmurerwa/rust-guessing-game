use std::io;
use rand::Rng;
use std::cmp::Ordering;

/// The following are the game rules for the game
/// The user gets 10 tries to guess the correct number where they can earn points. If the user guesses the correct
/// number in the first try, they get 10 points. If the user gets it in the second try, they get 9 points. If they guess
/// the number on the tenth try, they get 1 point. No points for any guesses past 10. Incorrect input is not considered
/// a guess.
///
fn main() {
    println!("Guess the number! You have 10 tries to get it right");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess_count = 0;

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            },
        };

        guess_count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too small!"),
            Ordering::Greater => println!("{guess} is too big!"),
            Ordering::Equal => {
                let points = calculate_points(guess_count);
                println!("You win with {points} point(s)");
                break;
            },
        }

        if guess_count >= 10 {
            println!("Sorry. You don't have any more guesses");
            break;
        }

        println!("You have {} guesses left", 10-guess_count);
    }
}

fn calculate_points(guesses: i32) -> i32 {
    let points = match guesses{
        1..=10 => 11 - guesses,
        _ => 0,
    };

    return points;
}