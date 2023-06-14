use rand::{self, Rng};
// use std::cmp::Ordering;
// use std::fs::File;
use std::io::stdin; // , BufRead, BufReader, ErrorKind, Write

pub fn run() {
    println!("hehe");
    let mut name = String::new();
    stdin().read_line(&mut name).expect("didn't receive input");

    println!("hello {}", name);

    let rand_num = rand::thread_rng().gen_range(1..101);

    println!("hello RAND {}", rand_num);

    // ternary operator
    let is_vote = if rand_num > 50 { true } else { false };
    println!("is_vote {}", is_vote);

    // match
    let age2 = 70;
    match age2 {
        1..=18 => println!("first"),
        21 | 70 => println!("second"),
        _ => println!("everything else / default"), // if not matching, this is fallback
    }
}

// note: putting an underscore infront of a variable will make rust ignore if its unused
