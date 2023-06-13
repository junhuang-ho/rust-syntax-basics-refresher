// primitive str = immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - use when need to modify own string data
//          (eg push/pop new strings to/from it like an array)

pub fn run() {
    let hello1: &str = "hello"; // str
    let mut hello2: String = String::from("hello"); // String

    println!("{} {}", hello1, hello2);
    println!("{} {}", hello1.len(), hello2.len());

    hello2.push('C');

    println!("{} {}", hello1, hello2);
    println!("{} {}", hello1.len(), hello2.len());

    hello2.push_str("momo");

    println!("{} {}", hello1, hello2);
    println!("{} {}", hello1.len(), hello2.len());

    println!(
        "{} {}, {} {}",
        hello2.capacity(),
        hello2.is_empty(),
        hello2.contains("Cmo"),
        hello2.replace("omo", "ioio")
    );

    for token in hello2.split('o') {
        println!("{}", token);
    }

    let mut s = String::with_capacity(5);
    s.push('a');
    s.push('b');
    assert_eq!(2, s.len());
    println!("PO {} | {}", s, s.capacity());
    s.push('a');
    s.push('b');
    println!("PO {} | {}", s, s.capacity());
    s.push('a');
    s.push('b');
    println!("PO {} | {}", s, s.capacity());
}
