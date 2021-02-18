use super::common::input;

#[derive(Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
    age: u8
}

impl Person {
    pub fn create() -> Person {
        Person {
            first_name: input("Please enter first name"),
            last_name: input("Please enter last name"),
            age: input("Please enter age").trim().parse().expect("Age must be a number"),
        }
    }

    pub fn from(person: &Person) -> Person {
        Person {
            first_name: String::from(&person.first_name),
            last_name: String::from(&person.last_name),
            age: person.age
        }
    }
}

