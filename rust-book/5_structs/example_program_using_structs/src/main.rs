#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let _rect1 = (width1, height1);
    let rect2 = Rectangle {
        width: width1,
        height: height1,
    };

    println!(
        "The area of the {:#?} is {} square pixels.",
        rect2,
        area(&rect2)
    );
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

//fn area(dim: (u32, u32)) -> u32 {
//    dim.0 * dim.1
//}

//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}
