use std::io;
use std::num::ParseIntError;

fn main() {
    println!("Generating the nth Fibonacci number");
    match get_n() {
        Ok(num) => println!("The {}th Fibonacci number is: {}", num, fib(num)),
        Err(_) => println!("Goodbye!"),
    }
}

fn get_n() -> Result<u128, ParseIntError> {
    println!("What is your n today?");

    let mut user_n = String::new();

    io::stdin()
        .read_line(&mut user_n)
        .expect("Failed to read line");

    user_n.trim().parse()
}

fn fib(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        num => fib(num - 1) + fib(num - 2),
    }
}
