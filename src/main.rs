use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    let secret = rand::thread_rng().gen_range(1..=100);
    
    let mut count = 0;

    let mut exit_count = 0;

    let password = "Keira";

    let pass = String::new();

    if pass == password {
        println!("The secret number is: {secret}");
    };


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                count += 1;
                exit_count+= 1;
                println!("You are not following the rules. Stop and type numbers or I will self destruct");
                println!("Your current Exit count is equal to {exit_count}");
                println!("Your current guess count is equal to {count}");
                        if exit_count >= 5 {
                            println!("You failed to follow the rules, I will now self destruct.");
                            break;
                        };
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("Too Small!");
                count += 1;
                if secret-guess <= 10 {
                    println!("You are less than 10 away.")
                };
                println!("Your current guess count is equal to {count}");
            },
            Ordering::Greater => {
                println!("Too Big!");
                count += 1;
                if guess-secret <= 10 {
                    println!("You are less than 10 away.")
                };
                println!("Your current guess count is equal to {count}");
            },
            Ordering::Equal => {
                count += 1;
                println!("You Win!\nThe secret number was {secret}.");
                println!("You took {count} attempts to guess it.");

                println!("\n\nWould you like to close the application? (Y/N)");
                let end = String::new();
                if end == "Y" || end == "y" {
                    break;
                };
            }
        }
    }
}