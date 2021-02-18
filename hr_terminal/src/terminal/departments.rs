use super::common::input;
use crate::terminal::personal::Person;

#[derive(Debug)]
pub struct Department {
    pub name: String,
    personal: Vec<Person>
}

impl Department {
    pub fn create() -> Department {
        Department {
            name: input("Please enter a name for the new department"),
            personal: vec![],
        }
    }
}
