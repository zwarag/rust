fn main() {
    let s = dangle();
    println!("{}", s);
}

fn dangle() -> String {
    let tmp = String::from("hello");
    tmp
}

// passing mutable references
//fn main() {
//    let mut s = String::from("Hello");
//    change(&mut s);
//    println!("{}", s);
//}
//
//fn change(s: &mut String) {
//    s.push_str(", world");
//}


//fn main() {
//    let s1 = String::from("hello world");
//    let len = calc_length(&s1);
//    // NOTE: s1 has ben passed to calc_length as a reference. Therefore s1 will not be `done` by
//    // `calc_length`.
//    println!("{} is {} characters long", s1, len);
//}
//
//fn calc_length(s: &String) -> usize {
//    s.len()
//}
