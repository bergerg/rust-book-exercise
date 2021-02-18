use std::collections::HashMap;

mod terminal;

use crate::terminal::hr::{hr_term, TermialInput};

fn main() {
    println!("Welcome to Rust HR Terminal!");
    println!("");

    let mut departments: HashMap<String, String> = HashMap::new();

    loop {
        match hr_term() {
            TermialInput::Hire => {},
            TermialInput::CreateDepartment => {},
            TermialInput::ShowDepartments => {},
            TermialInput::Exit => break,
        }

    }
    println!("Goodbye!");
}
