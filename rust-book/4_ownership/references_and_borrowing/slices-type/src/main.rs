fn main() {
    let word = String::from("hello world");
    let result = first_word(&word);
    println!("first word: {}", result);
    compare_i32_slice();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
   s
}

fn compare_i32_slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
