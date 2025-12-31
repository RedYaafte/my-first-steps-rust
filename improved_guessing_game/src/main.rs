use rand::Rng;
use std::{cmp::Ordering, io};

struct Game {
    secret_number: u32,
    max_attempts: u32,
    remaining_attempts: u32,
    record: Vec<u32>,
    min_range: u32,
    max_range: u32,
}

impl Game {
    fn new(difficulty: Difficulty) -> Self {
        let (attempts, min, max) = match difficulty {
            Difficulty::Easy => (10, 1, 50),
            Difficulty::Medium => (7, 1, 100),
            Difficulty::Hard => (5, 1, 200),
        };

        Game {
            secret_number: rand::thread_rng().gen_range(min..=max),
            max_attempts: attempts,
            remaining_attempts: attempts,
            record: Vec::new(),
            min_range: min,
            max_range: max,
        }
    }

    fn show_status(&self) {
        println!("\n📊 Game status:");
        println!(
            "  Remaining attempts: {}/{}",
            self.remaining_attempts, self.max_attempts
        );
        println!("  Range: {} - {}", self.min_range, self.max_range);
        println!("  Your previous attempts: {:?}", self.record);
    }

    fn attempt(&mut self, guess: u32) -> ResultAttempt {
        // Range validation
        if guess < self.min_range || guess > self.max_range {
            return ResultAttempt::OutOfRange;
        };

        // Repeat validation
        if self.record.contains(&guess) {
            return ResultAttempt::AlreadyGuessed;
        };

        // Record guess
        self.record.push(guess);
        self.remaining_attempts -= 1;

        match guess.cmp(&self.secret_number) {
            Ordering::Equal => ResultAttempt::Correct,
            Ordering::Less => {
                let difference = self.secret_number - guess;
                if difference <= 5 {
                    ResultAttempt::AlmostTooSmall
                } else {
                    ResultAttempt::TooSmall
                }
            }
            Ordering::Greater => {
                let difference = guess - self.secret_number;
                if difference <= 5 {
                    ResultAttempt::AlmostTooBig
                } else {
                    ResultAttempt::TooBig
                }
            }
        }
    }

    fn lost(&self) -> bool {
        self.remaining_attempts == 0
    }

    fn calculate_score(&self) -> u32 {
        let attempts_used = self.max_attempts - self.remaining_attempts;
        let base_points: u32 = 1000;
        let penalty_for_attempt = 100;

        if attempts_used == 0 {
            return 0;
        };

        base_points.saturating_sub(attempts_used * penalty_for_attempt)
    }
}

enum ResultAttempt {
    Correct,
    TooSmall,
    AlmostTooSmall,
    TooBig,
    AlmostTooBig,
    OutOfRange,
    AlreadyGuessed, // Repeat
}

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

fn main() {
    println!("🎮 Welcome to the Guessing Game!");

    let difficulty = choose_difficulty();
    let mut game = Game::new(difficulty);

    println!(
        "\n🎯 Guess the number between {} and {}",
        game.min_range, game.max_range
    );

    loop {
        game.show_status();

        let guess = match read_number() {
            Some(num) => num,
            None => continue,
        };

        let result = game.attempt(guess);

        match result {
            ResultAttempt::Correct => {
                let points = game.calculate_score();
                println!("\n🎉 CONGRATULATIONS! You guessed the number!");
                println!("🏆 Your score: {} points", points);
                break;
            }
            ResultAttempt::TooSmall => println!("📉 To small"),
            ResultAttempt::AlmostTooSmall => println!("🔥 Almost too small!"),
            ResultAttempt::TooBig => println!("📈 Too big"),
            ResultAttempt::AlmostTooBig => println!("🔥 Almost too big!"),
            ResultAttempt::OutOfRange => {
                println!(
                    "❌ Your guess is out of range! Please guess a number between {} and {}.",
                    game.min_range, game.max_range
                );
                continue;
            }
            ResultAttempt::AlreadyGuessed => {
                println!("❌ You've already guessed that number! Try a different one.");
                continue;
            }
        }
        if game.lost() {
            println!(
                "\n💀 You've run out of attempts! The secret number was {}.",
                game.secret_number
            );
            break;
        }
    }
}

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
