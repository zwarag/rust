#[derive(Debug)]
enum UsState {
    Albama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Oh look a penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quater from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option <i32> {
    match x {
        Option::None => None,
        Option::Some(y) => Some(y + 1)
    }
}

fn plus_n(x: Option<i32>, n: i32) -> Option <i32> {
    match x {
        Option::None => None,
        Option::Some(y) => Some(y + n)
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Quater(UsState::Alaska)));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let ten = plus_n(six, 4);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);
    println!("{:?}", ten);
}
