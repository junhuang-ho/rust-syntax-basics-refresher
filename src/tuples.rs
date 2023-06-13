// tuples group together values of different types
// tuples can have max of 12 elements

pub fn run() {
    let person: (&str, i32, f64) = ("hello", 37, 88.88);

    println!("{}", person.0);
}
