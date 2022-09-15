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
}
