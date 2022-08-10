struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(name: String, email: String) -> User {
    User {
        email,
        username: name,
        active: true,
        sign_in_count: 0,
    }
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);
    println!("{}", black.0);
}


//fn main() {
//    let user = build_user(String::from("harrys"), String::from("harrys@example.com"));
//    let user2 = User {
//        email: String::from("user3@example.com"),
//        ..user
//    };
//    // println!("{}", user.username); // does not work anymore as user.username has been borrowed
//    // by user2.username
//}

//fn main() {
//    let mut user2 = User {
//        email: String::from("test2@example.com"),
//        username: String::from("user2"),
//        active: true,
//        sign_in_count: 3,
//    };
//
//    user2.email = String::from("user2@example.com");
//}































//fn main() {
//    let user1 = User {
//        email: String::from("test@example.com"),
//        username: String::from("zwarag"),
//        active: true,   
//        sign_in_count: 1
//    };
//}
