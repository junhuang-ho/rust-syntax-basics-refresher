pub fn run() {
    println!("hehe");
    println!("nno {}", 123);
    println!(
        "hehe {test1}, hoho {test2}, lolo {test1}",
        test1 = "yes",
        test2 = "no"
    );
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10); // placeholder traits
    println!("{:?}", (12, true, "hello")); // placeholder for debug traits
    println!("{}", 10 + 10); // basic math
}
