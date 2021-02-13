fn main() {
    let vec1 = vec![5, 32, 1, 10, 18, 3];
    assert_eq!(11.5, mean(&vec1), "failed assertion on mean value of vec1");

}

fn mean(v: &Vec<i32>) -> f32 {
    match v.len() {
        0 => 0.0,
        length => v.iter().sum::<i32>() as f32 / length as f32
    }
}