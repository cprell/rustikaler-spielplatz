#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self here is short for self: &Self
    // this is a method! it has &self as first parameter
    // it needs an instance of Rectangle to work with
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function
    // they do not need an instance of the type Rectangle
    // often used as constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// having multiple impl blocks is valid syntax

impl Rectangle {}

pub(crate) fn refactored_rectangle() {
    println!();
    println!("---");
    println!("Refactored Rectangle (Chapter 5.3)");
    println!("---");
    println!();

    let rect1 = Rectangle {
        width: 50,
        height: 80,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the refactored rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // associated function, also: String::from is one

    let square = Rectangle::square(5);
}
