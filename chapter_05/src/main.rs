mod refactored_rectangle;

struct Person {
    last_name: String,
    first_name: String,
    alive: bool,
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let person1 = Person {
        last_name: String::from("Testperson"),
        first_name: String::from("Thomas"),
        alive: true,
    };

    // accessing specific values with dot notation
    // only possible if mutable, e.g. let mut person 1 = ...

    // person1.first_name = String::from("Theodor");

    build_person(String::from("Schwein"), String::from("Peter"));

    // struct update syntax ..
    let person2 = Person {
        last_name: String::from("Meier"),
        ..person1
    };
    // after this, person1 is not valid anymore, since first_name moved into person2

    // tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // origin.1 etc.

    // unit-like-structs
    let subject = AlwaysEqual;

    // 5.2

    let width = 30;
    let height = 50;
    println!(
        "Area of rectangle is {} square pixels.",
        calculate_area(width, height)
    );

    // with tuples
    let rectangle = (20, 50);
    println!(
        "Area of rectangle with tuples is {} square pixels.",
        calculate_area_with_tuples(rectangle)
    );

    // with structs
    // struct Rectangle defined above

    let rect = Rectangle {
        width: 30,
        height: 80,
    };

    println!(
        "Area of rectangle with struct is {} square pixels.",
        calculate_area_with_struct(&rect)
    );

    // #[derive(Debug)]
    // we can print structs!

    // normal print all values
    println!("rect is {:?}", rect);
    // pretty print all values
    println!("rect is {:#?}", rect);

    // dbg!
    // this prints to stderr instead of stdout

    let scale = 2;
    let new_rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&new_rect);

    refactored_rectangle::refactored_rectangle();
}

fn build_person(last_name: String, first_name: String) -> Person {
    Person {
        last_name, // no need to write last_name: last_name
        first_name,
        alive: true,
    }
}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}

fn calculate_area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn calculate_area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
