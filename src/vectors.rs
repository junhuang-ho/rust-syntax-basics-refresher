// vectors - resizable arrays

use std::mem;

pub fn run() {
    println!("VECTOR");
    let num: Vec<i32> = vec![45, 88, 2, 43];

    println!("{:?}", num);
    println!("{}", num[1]);

    let mut num2: Vec<i32> = vec![45, 88, 2, 99];
    println!("{:?}", num2);
    num2[2] = 20;
    println!("{:?}", num2);
    println!("{}", num2.len());
    println!("{}", mem::size_of_val(&num2)); // & means reference

    let slice: &Vec<i32> = &num2;
    println!("slice: {:?}", slice);

    let slice2: &[i32] = &num2[0..2];
    println!("slice: {:?}", slice2);

    // Vector specific:
    num2.push(5);
    println!("{:?}", num2);
    num2.pop();
    println!("{:?}", num2);

    for x in num2.iter() {
        println!("number {}", x);
    }
    println!("number {:?}", num2);
    for x in num2.iter_mut() {
        *x *= 2; // equivalent syntax to below
    } // this is like js .map()
    println!("number {:?}", num2);
    for x in num2.iter_mut() {
        *x = *x * 2; // equivalent syntax to above
    } // this is like js .map()
    println!("number {:?}", num2);
}
