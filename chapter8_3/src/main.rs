use std::collections::HashMap;

fn main() {
    println!("Hello, colletions::hash maps!");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    //accesing values
    let team_1 = String::from("Blue");
    let team_2 = String::from("Yellow");
    let score = scores.get(&team_1).copied().unwrap_or(0);
    let score_yellow = scores.get(&team_2).copied().unwrap_or(0);
    println!("{team_1}: {score}");
    println!("{team_2}: {score_yellow}");

    //replacing a value
    scores.insert(String::from("Blue"), 25);

    //add entry only if the key doesn`t have value
    scores.entry(String::from("blue")).or_insert(50);
    //iterate over hashmap
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
