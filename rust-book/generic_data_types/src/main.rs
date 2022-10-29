fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}


impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T> Point<T> {
    fn mixup<T2>(self, other: Point<T2>) -> Point_Mixed<T, T2> {
        Point_Mixed {
            x: self.x,
            y: other.y,
        }
    }
}


#[derive(Debug)]
struct Point_Mixed<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point_Mixed<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point_Mixed<X2, Y2>) -> Point_Mixed<X1, Y2> {
        Point_Mixed {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    //let number_list = vec![34, 50, 25, 100, 65];
    //let result = largest(&number_list);
    //println!("The largest number is {}", result);

    //let char_list = vec!['y', 'm', 'a', 'q'];
    //let result = largest(&char_list);
    //println!("The largest char is {}", result);

    //let integer = Point {x: 5, y: 10};
    //let float = Point {x: 1.0, y: 4.0};
    //let mixed = Point {x: 5, y: 4.0};

    let p = Point {x:5, y:10};
    println!("p.x = {}", p.x());
    let p2 = Point {x: 2.3, y:6.6};
    println!("p2 distance from origin (0, 0) is {}", p2.distance_from_origin());
    let p3 = Point {x: "Hello", y: "c"};
    //let p4 = p2.mixup(p3);
    let p5 = p.mixup(p3);
    //println!("p4.x = {}, p4.y = {}", p4.x, p4.y);
    println!("{:#?}", p5);
}
