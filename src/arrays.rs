// arrays are fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let num: [i32; 4] = [45, 88, 2, 43];

    println!("{:?}", num);
    println!("{}", num[1]);

    let mut num2: [i32; 4] = [45, 88, 2, 99];
    println!("{:?}", num2);
    num2[2] = 20;
    println!("{:?}", num2);
    println!("{}", num2.len());
    println!("{}", mem::size_of_val(&num2)); // & means reference

    let slice: &[i32; 4] = &num2;
    println!("slice: {:?}", slice);

    let slice2: &[i32] = &num2[0..2];
    println!("slice: {:?}", slice2);
}
