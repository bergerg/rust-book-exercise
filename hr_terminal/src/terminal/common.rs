use std::io;

pub fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");
    String::from(option)
}