use crate::terminal::personal::Person;

#[derive(Debug)]
pub struct Department {
    pub name: String,
    personal: Vec<Person>
}
