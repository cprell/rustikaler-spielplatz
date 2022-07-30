fn main() {
    // ownership (chap. 4.1)

    {
        let _s = "hello";

        // _s is valid here
    }
    // _s is not valid anymore
    // this variable is on the stack and Rust knows when it can be popped off. (String literal)

    // String type -> Heap
    let _s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1;
    // -> s1 no longer valid, s1 is "moved" onto s2 and s2 points to the same location on the heap

    // if we want to clone we can use
    // let s2 = s1.clone();
    // -> heap data gets copied, this MAY be expensive

    let x = 5;
    let y = x;
    // this is valid, x is valid since size is known at runtime -> stored on stack

    // references and borrowing (chap. 4.2)
    // when passing a reference there is no ownership -> value it points to will not be dropped
    // creating a reference is called borrowing
    // references cannot be changed, unless they are made mutable

    let example_string = String::from("Hello!");
    // mutable reference would be
    // let mut example_string = String::from("Hello!");

    let length = calculate_string_length(&example_string);
    // mutable reference call would be
    // let length = calculate_string_length(&mut example_string); + calculate_string_length should
    // accept &mut String as parameter

    println!("String length of string \"{example_string}\" is {length}");

    // this fails
    // you cannot have 2 mutable references to one value
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // this prevents data races

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_string_length(s: &String) -> usize {
    s.len()
}
