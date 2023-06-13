// variables hold primitive data or ref to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run() {
    let mut name: &str = "hello";
    let mut age: i32 = 37;
    println!("test {} {}", name, age);
    name = "bob";
    age = 38;
    println!("test {} {}", name, age);

    // constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let (test1, test2) = ("hoho", 99);
    println!("test {}, {}", test1, test2);
}
