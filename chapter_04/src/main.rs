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

    // slicing

    let mut s = String::from("Hello world");
    let word = first_word_old(&s);
    // s.clear();
    println!("The first word is: {word}");

    string_slicing_shenanigans();

    // array slicing
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn calculate_string_length(s: &String) -> usize {
    s.len()
}

fn first_word_old(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // b = byte literal
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_slicing_shenanigans() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // b = byte literal
            return &s[0..i];
        }
    }

    &s[..]
}
