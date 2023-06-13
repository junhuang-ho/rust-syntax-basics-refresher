/**
 * Primitive Types
 * Integers: u8, i8, u16, i16, ... u128, i128
 * (u=unsiged (no neg values), i=signed, number= how many bits they take up in memory)
 * (i32 is default)
 *
 * Floats: f32, f64 (f64 is default)
 * Boolean: bool
 * Characters: char
 * Tuples
 * Arrays
 *
 * Rust is a statically typed lang, which means that it must know the
 * types of all variables at compile time, however, the compiler can usually
 * infer the type from the value and how it is used
 */

pub fn run() {
    const X: u32 = 11;
    let y = 12.3;
    let z: i64 = 454545454545;
    println!("test {} | {} | {}", X, y, z);

    println!("Max i32 {}", std::i32::MAX);

    let is_active = true;
    let is_greater = 10 > 15;
    let one_char = 'a';
    println!("TEST {} | {} | {}", is_active, is_greater, one_char);
}
