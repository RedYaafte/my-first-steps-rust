use std::io;

fn main() {
    println!("=== Calculator CLI ===");

    let num1 = read_num("Add a first number");
    let operation = read_operation();
    let num2 = read_num("Add a second number");

    match calculate(num1, num2, &operation) {
        Ok(result) => println!("Result: {} {} {} = {}", num1, operation, num2, result),
        Err(error) => println!("{}", error),
    }
}

fn read_num(message: &str) -> f64 {
    loop {
        println!("{}", message);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn read_operation() -> String {
    loop {
        println!("Enter the operation (+, -, *, /):");
        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");
        let operation = operation.trim().to_string();

        if ["+", "-", "*", "/"].contains(&operation.as_str()) {
            return operation;
        } else {
            println!("Invalid operation. Please use one of +, -, *, /.");
        }
    }
}

fn calculate(num1: f64, num2: f64, operation: &str) -> Result<f64, String> {
    match operation {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Error: Division by zero is not allowed.".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operation.".to_string()),
    }
}
