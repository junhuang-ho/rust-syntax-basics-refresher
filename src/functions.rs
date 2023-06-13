pub fn run() {
    greeting("Hello", "Jane");

    let get_sum = add(5, 5); // Bind function values to variables
    println!("Sum: {}", get_sum);

    // Closure (notice the n3 - not as fn input)
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 // no semicolon (;) means this is what we want to return !!!!
}
