#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        if self.width < rect2.width {
            return false
        };
        if self.height < rect2.height {
            return false
        };
        true
    }
    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 60
    };
    let rect3 = Rectangle {
        width: 40,
        height: 10
    };
    println!("Can {:?} hold {:?} ::: {}", rect1, rect2, rect1.can_hold(&rect2));
    println!("Can {:?} hold {:?} ::: {}", rect1, rect3, rect1.can_hold(&rect3));
    println!("Can {:?} hold {:?} ::: {}", rect1, rect1, rect1.can_hold(&rect1));
    println!("{:#?}", Rectangle::square(32));
    //println!(
    //    "The area of the rectangle is {} square pixels.",
    //    rect1.area()
    //);
}
