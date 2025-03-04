use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Test1"), 45);
    scores.insert(String::from("Test2"), 60);
    scores.insert(String::from("Test3"), 87);

    println!("Highest score: {}", get_highest_score(&scores));
}

fn get_highest_score(scores: &HashMap<String, i32>) -> i32 {
    let mut highest = scores.values().next().unwrap();
    for value in scores.values() {
        if *value > highest {
            highest = value;
        }
    }
    return highest;
}
