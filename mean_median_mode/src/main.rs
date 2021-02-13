fn main() {
    let vec1 = vec![5, 32, 1, 10, 18, 3, 1];
    assert_eq!(10.0, mean(&vec1), "failed assertion on mean value of vec1");
    assert_eq!(5, median(&vec1), "failed assertion on median value of vec1")
}

fn mean(v: &[i32]) -> f32 {
    match v.len() {
        0 => 0.0,
        length => v.iter().sum::<i32>() as f32 / length as f32
    }
}

fn median(v: &[i32]) -> i32 {
    match v.len() {
        0 => 0,
        length => {
            let mut copy = v.to_vec();
            copy.sort();
            copy[length / 2]
        }
    }
}
