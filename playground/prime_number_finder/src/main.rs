use std::collections::HashMap;

static MAX_NUMBER: i32 = 1000;

fn main() {
    let mut hashmap_index = 0;
    let mut prime_numbers: HashMap<i32, i32> = HashMap::new();

    for candidate in 2..MAX_NUMBER {
        let mut dividers = 0;
        for i in 2..MAX_NUMBER {
            if i >= candidate {
                dividers += 1;
                break;
            }
            dividers += if candidate % i == 0 { 1 } else { 0 };
            if dividers > 1 {
                break;
            }
        }
        if dividers == 1 {
            prime_numbers.insert(hashmap_index, candidate);
            hashmap_index += 1;
        }
    }

    let len = prime_numbers.len();
    println!("There are {} prime numbers!", len);
    //print!("List of Prime Numbers: {{\n\t");
    //for i in 0..len {
    //    print!("{}", prime_numbers.get(&(i as i32)).unwrap());
    //    if i != len - 1 {
    //        print!(", ");
    //    } else {
    //        println!();
    //    }
    //}
    //println!("}}");
}
