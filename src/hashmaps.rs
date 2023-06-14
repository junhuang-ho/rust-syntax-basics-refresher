use std::collections::HashMap;

pub fn run() {
    let mut heroes = HashMap::new();
    heroes.insert("keytest", "valuetest");
    heroes.insert("keytest2", "valuetest2");

    println!("OK{:?}", heroes);

    println!("LEN{}", heroes.len());

    for (k, v) in heroes.iter() {
        println!("LO{}-{}", k, v);
    }

    // heroes.contains_key()
    // heroes.get(key)
}
