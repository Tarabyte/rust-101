use std::collections::HashMap;

fn main() {
    test_creation();

    test_creation_with_collect();

    test_update();

    test_update_using_previous();
}

fn test_update_using_previous() {
    let text = "hello world wonderful world";

    let mut words = HashMap::new();

    for word in text.split_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", words);
}

fn test_update() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 20);

    scores.insert("Blue".to_string(), 30);

    println!("{:#?}", scores);

    let blue_team = "Blue";

    scores.entry(blue_team.to_string()).or_insert(0);
    scores.entry(blue_team.to_string()).or_insert(100);
    println!("{:#?}", scores);
}

fn test_creation_with_collect() {
    let teams = vec!["Blue", "Red"];
    let scores = vec![10, 20];

    let scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();

    println!("{:#?}", scores);
}

fn test_creation() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 20);

    println!("{:#?}", scores);

    scores.clear();

    println!("{:#?}", scores);
}
