use rand::Rng;
use std::io;

fn main() {
    println!("🎮 Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("\n Add your number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("📉 Too small!"),
            std::cmp::Ordering::Greater => println!("📈 Too big!"),
            std::cmp::Ordering::Equal => {
                println!("🎉 You win!");
                break;
            }
        };
    }
}
