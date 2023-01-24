fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::<&String, i32>::new();

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    scores.insert(&blue, 10);
    scores.insert(&yellow, 50);

    let score = scores.get(&blue).copied().unwrap_or(0);

    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{key} -> {value}");
    }

    // An example overwritting a value
    scores.insert(&blue, 150);
    println!("{:?}", scores);

    let red = String::from("Red");

    // Insert if a key does not exist
    scores.entry(&blue).or_insert(200);
    scores.entry(&red).or_insert(500);

    println!("{:?}", scores);
}
