fn main() {
    let first = String::from("first");
    assert_eq!("irst-fay", piggify(&first));
    let apple = String::from("apple");
    assert_eq!("apple-hay", piggify(&apple));
    let hay = String::from("hay");
    assert_eq!("ay-hay", piggify(&hay));
    let iter = String::from("iter");
    assert_eq!("iter-hay", piggify(&iter));
}

fn piggify(word: &String) -> String {
    let mut chars = word.chars();
    match chars.nth(0) {
        Some(c) if is_vowel(c) => format!("{}-hay", word),
        Some(c) => format!("{}-{}ay", chars.collect::<String>(), c),
        None => "".to_string(),
    }
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
