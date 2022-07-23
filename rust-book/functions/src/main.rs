fn five() -> i32 {
    return 5;
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}

fn main() {
    let x = plus_one(five());
    println!("The value of x is {x}!");
}

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("The value of y is: {y}");
// }

// fn print_labeled_measurement(value: u32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}!");
// }

