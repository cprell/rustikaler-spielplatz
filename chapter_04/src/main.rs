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
}
