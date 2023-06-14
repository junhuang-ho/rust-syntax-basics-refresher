use std::ops::Add;

pub fn run() {
    println!("test add {}", get_sum_gen(5, 4));
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
} // ref: https://doc.rust-lang.org/book/ch10-01-syntax.html
