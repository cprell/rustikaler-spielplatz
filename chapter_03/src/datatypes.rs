pub(crate) fn datatype_playground() {
    println!();
    println!("---");
    println!("Datatypes (Chapter 3)");
    println!("---");
    // scalar types
    // integers, floating-point numbers, Booleans, characters

    // integers
    // i{x} and u{x}, where x = 8, 16, 32, 64, 128 and size
    // number meaning length (e.g. 32 bit)
    // size = architecture (64 bit -> 64)
    // i = signed, u = unsigned

    let small_int: i8 = 2_4; // this is valid
    println!("Small number 2_4 is valid and counts as {small_int}");

    // floating point
    // f32 (single precision), f64 (double precision)

    // boolean
    // true, false

    // char
    // e.g. 'z'
    let heart_eyed_cat = '😻';
    println!("Cute Cat: {heart_eyed_cat}");

    // compound types
    // tuples
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    // This does NOT work, there is no function for printing a tuple like that
    // println!("Whole tuple: {tuple}");

    // destructuring tuple
    let (_x, y, _z) = tuple;
    println!("Value of y in tuple is: {y}");

    let last_number_in_tuple = tuple.2;
    println!("Value of z in tuple is: {last_number_in_tuple}");

    // arrays
    let array = [1, 2, 3];
    // allocated on stack, not heap
    // array not as flexible as vector, but generally more useful when number of elements
    // will not need to change

    let _typed_array: [i32; 5] = [1, 2, 3, 4, 5];

    let _length_five_values_three_array = [3; 5];

    let _first_value = array[0];
}
