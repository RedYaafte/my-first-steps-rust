use std::io;
fn main() {
    let tu: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = tu.0;
    println!("Five hundred: {_five_hundred}");
    let _six_point_four = tu.1;
    println!("Six point four: {_six_point_four}");
    let _one = tu.2;
    println!("One: {_one}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");
    print!("---------------------------------- \n");
    another_function();
}

fn another_function() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
