use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("🎮 Guess the number!");
    println!("You have 7 attempts to guess a number between 1 and 100.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut remaining_attempts = 7;
    let mut record: Vec<u32> = Vec::new();

    loop {
        println!("📊 Remaining attempts: {}", remaining_attempts);
        println!("Your previous attempts: {:?}", record);
        println!("\nAdd a number: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess: u32 = match input.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("❌ Please enter a number between 1 and 100!");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("❌ Please enter a valid number!");
                continue;
            }
        };

        if record.contains(&guess) {
            println!(
                "⚠️ You've already guessed {}! Try a different number.",
                guess
            );
            continue;
        }

        record.push(guess);
        remaining_attempts -= 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                let difference = secret_number - guess;
                if difference <= 5 {
                    println!("🔥 Very close! Just a bit higher!");
                } else {
                    println!("📉 Too small!");
                }
            }
            Ordering::Greater => {
                let difference = guess - secret_number;
                if difference <= 5 {
                    println!("🔥 Very close! Just a bit lower!");
                } else {
                    println!("📈 Too big!");
                }
            }
            Ordering::Equal => {
                println!("🎉 You win! in {} attempts", 7 - remaining_attempts);
                return;
            }
        };
        if remaining_attempts == 0 {
            println!(
                "💥 You've run out of attempts! The number was {}. Better luck next time!",
                secret_number
            );
            return;
fn choose_difficulty() -> Difficulty {
    println!("\nChoose the difficulty");
    println!("1. Easy (1-50, 10 attempts)");
    println!("2. Medium (1-100, 7 attempts)");
    println!("3. Hard (1-200, 5 attempts)");

    loop {
        if let Some(option) = read_number() {
            match option {
                1 => return Difficulty::Easy,
                2 => return Difficulty::Medium,
                3 => return Difficulty::Hard,
                _ => println!("❌ Please select a valid option (1, 2, or 3)!"),
            }
        }
    }
}

fn read_number() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("❌ Please enter a valid number!");
            None
        }
    }
}
