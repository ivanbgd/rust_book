#![allow(unused)]

struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Color(i32, i32, i32);  // A tuple struct.

#[derive(Debug)]
/// Struct Rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// Calculates area of the rectangle.
    fn area(&self) -> u32 {
        self.width * (*self).height
    }

    fn width(&self) -> u32 {  // getter
        self.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn is_width_positive(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Self {  // associated function
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let a: i32 = 1;
    let b: &i32 = &2;
    let c: *mut i32 = 3 as *mut i32;
    println!("a = {}, b = {} {} {}\n", a, b, &b, *b);  // a = 1, b = 2 2 2

    let mut p = Point { x: 0, y: 0 };
    let x = &mut p.x;
    *x += 1;
    println!("x = {}, y = {}\n", p.x, p.y);

    let c = Color(1, 2, 3);
    println!("c = {:?}", c);
    println!("c = {:#?}\n", &c);

    let r1 = Rectangle { width: 10, height: 20, };
    let r2 = Rectangle { width: 11, height: 20, };
    println!("area = {}\n", area(&r1));
    dbg!(&r1);
    println!("r1 = {:?}", r1);
    println!("&r1 = {:#?}\n", &r1);
    // println!("area2 = {}\n", area2(r1));  // Value r1 moved here.

    println!("r1.area = {}, &r1.area = {}", r1.area(), &r1.area());
    println!("r1.width = {} = {}, {}", r1.width, r1.width(), r1.is_width_positive());  // Value borrowed in `r1.width` after move.
    println!("Can r1 hold r2? {}\n", r1.can_hold(&r2));

    let mut square = Rectangle::square(12);
    println!("Square = {:?}; area = {}", square, square.area());
    square.set_width(10);
    Rectangle::set_width(&mut square, 20);
    println!("Square = {:?}; area = {}. This is not a square!\n", square, Rectangle::area(&square));
}

/// Calculates area of the passed rectangle.
fn area(r: &Rectangle) -> u32 {
    // Parameter r doesn't take ownership of the value. References are non-owning pointers.
    r.width * (*r).height
}

fn area2(r: Rectangle) -> u32 {
    // Parameter r takes ownership of the value.
    r.width * r.height
}
