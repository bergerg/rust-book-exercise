use std::io;

fn main() {
    loop {
        println!("Please choose convertion:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");

        let mut user_choise = String::new();

        io::stdin()
            .read_line(&mut user_choise)
            .expect("Failed to read line");

        match user_choise.trim().parse() {
            Ok(num) => match num {
                1 => {
                    println!("Converting Celsius to Fahrenheit");
                    break;
                },
                2 => {
                    println!("Converting Fahrenheit to Celsius");
                    break;
                },
                _ =>  {
                    println!("Invalid conversion selected.");
                    continue;
                }
            },
            Err(_) => {
                println!("Please input only the number of the choise");
                continue;
            },
        };
    }

}
