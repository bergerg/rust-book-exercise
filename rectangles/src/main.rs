fn main() {
    let rec1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rec1)
    )
}

fn area(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}