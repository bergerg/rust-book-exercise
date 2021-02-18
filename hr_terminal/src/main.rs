use std::collections::HashMap;

mod terminal;

use crate::terminal::hr::{hr_term, TermialInput};
use crate::terminal::departments::Department;
use crate::terminal::personal::Person;

fn main() {
    println!("Welcome to Rust HR Terminal!");
    println!("");

    let mut departments: HashMap<String, Department> = HashMap::new();

    loop {
        match hr_term() {
            TermialInput::Hire => {
                let person: Person = Person::create();
                // todo: link this person to a department
            },
            TermialInput::CreateDepartment => {
                let department: Department = Department::create();
                departments.insert(String::from(&department.name), department);
            },
            TermialInput::ShowDepartments => {},
            TermialInput::Exit => break,
        }

    }
    println!("Goodbye!");
}
