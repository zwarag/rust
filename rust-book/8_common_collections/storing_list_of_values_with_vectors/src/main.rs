fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let third: Option<&i32> = v.get(100);
    match third {
        Some(third) => println!("The value is {}", third),
        None => println!("No value"),
    }

    // // This does not work as first borrows the first value of v2 and pushing another value would
    // // reallocate the array, therefore v2 would point to nothing.
    // let mut v2 = vec![1,2,3,4,5];
    // let first: Option<&i32> = v2.get(0);
    // v2.push(6);
    // match first {
    //     Some(first) => println!("first one is {}", first),
    //     None => println!("first does not exist anymore"),
    // }
    
    for value in &v {
        println!("{}", value);
    }

    for value in &mut v {
        *value += 50;
        println!("{}", value);
    }

    println!("{:?}", v);

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::value("hello world")),
        SpreadSheetCell::Float(3.14)
    ];


}
