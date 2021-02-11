use std::io;
use std::num::ParseFloatError;

fn main() {
    loop {
        println!("Please choose convertion:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("");
        println!("To exit type 0");

        let mut user_choise = String::new();

        io::stdin()
            .read_line(&mut user_choise)
            .expect("Failed to read line");

        match user_choise.trim().parse() {
            Ok(num) => match num {
                0 => {
                    println!("Goodbye!");
                    break;
                },
                1 => {
                    println!("Converting Celsius to Fahrenheit");
                    match get_temp_from_user() {
                        Ok(temp) => println!("{} degrees Celsius are {} in Fahrenheit", temp, cel_to_fahr(temp)),
                        Err(_) => println!("invalid temperature!"),
                    }
                    break;
                },
                2 => {
                    println!("Converting Fahrenheit to Celsius");
                    match get_temp_from_user() {
                        Ok(temp) => println!("{} degrees Fahrenheit are {} in Celsius", temp, fahr_to_cel(temp)),
                        Err(_) => println!("invalid temperature!"),
                    }
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

fn get_temp_from_user() -> Result<f32, ParseFloatError> {
    println!("Please input temprature to convert:");
    let mut user_temp = String::new();

    io::stdin()
        .read_line(&mut user_temp)
        .expect("Failed to read line");

    user_temp.trim().parse()
}

fn cel_to_fahr(t: f32) -> f32 {
    t * 9f32 / 5f32 + 32f32
}

fn fahr_to_cel(t: f32) -> f32 {
    (t - 32f32) * 5f32 / 9f32
}