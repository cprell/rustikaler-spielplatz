pub(crate) fn variable_playground() {
    println!("---");
    println!("Variables (Chapter 3)");
    println!("---");

    // const needs a type
    // const can be used for global variables
    const ONE_DAY_IN_SECONDS: u32 = 60 * 60 * 24;

    println!("One Day in Seconds: {ONE_DAY_IN_SECONDS}");

    let mut x = 5;
    println!("Value of x is: {x}");
    x = 6;
    println!("Value of x is: {x}");

    // shadowing

    let y = 6;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y in the outer scope is: {y}");
}
