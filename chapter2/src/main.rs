use rand::Rng;
use std::io;

fn main() {
    // generate number from 1 to 100
    let generated_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();
        println!("Guess the number: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        if guess < generated_number {
            println!("Too low!");
        } else if guess > generated_number {
            println!("Too high!");
        } else {
            println!("You win!");
            break;
        }
    }
}
