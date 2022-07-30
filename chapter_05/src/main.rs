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
}

fn build_person(last_name: String, first_name: String) -> Person {
    Person {
        last_name, // no need to write last_name: last_name
        first_name,
        alive: true,
    }
}
