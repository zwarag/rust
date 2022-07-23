use std::io;

fn main() {
    // float
    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32

    // arithmetic
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    // remainder
    let remainder = 43 % 5;

    // bool
    let t = true;
    let f: bool = false; // with explicit type annotation

    // char
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // arrays
    // must be of the same type
    // fix length
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // 5 elements of i32, the also need to be initialized
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // access
    let first = a[0];
    let second = a[1];

    example();
}

fn example() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

