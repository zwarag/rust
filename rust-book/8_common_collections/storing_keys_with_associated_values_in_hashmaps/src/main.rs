use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // let blue = scores.get(&String::from("Blue"));

    // println!("{:?}", blue);

    for (key, value) in &scores {
        println!("key: {};\tvalue {}", key, value);
    }
    
    let key = String::from("key");
    let value = String::from("value");

    let mut map = HashMap::new();
    map.insert(key, &value);

    println!("{:?}", map);
    println!("{}", value);

    scores.insert(String::from("Blue"), 100);
    scores.entry(String::from("Bluee")).or_insert(300);

    let blue = scores.entry(String::from("Bluee"));

    println!("{:?}", blue);


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

