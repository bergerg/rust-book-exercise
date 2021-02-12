#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rec1 is {:#?}", rec1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    )
}
