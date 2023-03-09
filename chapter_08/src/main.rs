fn main() {
    // Vectors
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3]; // rust can infer what values we use

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // v.push("2"); // does not work obviously, has to be of same type

    // reading values

    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // this causes the program to panic, it references a nonexistent element
    // use this when you want the program to crash
    // let does_not_exist = &v[100];

    // this does return None
    let does_not_exist = v.get(100);

    // this does not work
    let mut v = vec![1, 2, 3, 4, 5];

    // we borrow as immutable here
    let first = &v[0];

    // we borrow as mutable
    v.push(6);

    // we can't borrow as mutable while borrowing as immutable

    // iterating
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // or mutable

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // * = dereferencing (see chap 15)
        *i += 50;
    }

    // vectors are one type only, unless you use an enum

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // if you don't know exactly what types will be present at runtime you can use a trait object, see chap. 17

    // dropping a vector drops its elements
    {
        let v = vec![1, 2, 3];
    }
    // out of scope again

    // Strings

    let mut s = String::new();

    // initialize string with value

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // utf-8
    let mut hello = String::from("你好");

    // push_str takes a string slice because we don't necessarily want to take ownership of the parameter
    hello.push_str("beep, boop");

    hello.push('!');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // since this is a macro call - no ownership
    let s = format!("{}-{}-{}", s1, s2, s3);

    // Indexing into Strings

    // str[0] does not work due to how data is saved internally

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    // this works, but if you do &hello[0..1] (less than one char) the program will crash at runtime

    // better:
    for c in "Зд".chars() {
        println!("{}", c);
    }

    // or
    for b in "Зд".bytes() {
        println!("{}", b);
    }

    // Hash maps
}
