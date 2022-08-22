fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element!"),
    }

    let v2 = vec![100, 32, 42];
    for i in &v2 {
        println!("val: {}", i);
    }

    let mut v3 = vec![100, 32, 42];
    for i in &mut v3 {
        *i += 50;
        println!("val: {}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    for i in &row {
        println!("The value is {:?}", i);
    }
}
