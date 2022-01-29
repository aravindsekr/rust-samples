
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![
        String::from("Blue"), 
        String::from("Yellow")
    ];

    let initial_scores = vec![10, 50];

    // into_iter -> tuple, zip -> matching tuples -> collect into hash map
    let scores: HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!(" scores {:?}", scores);

    // Option<&V> - Some(&10) or None
    let score_for_blue = scores.get(&String::from("Blue"));
    println!(" score for blue {:?}", score_for_blue);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
