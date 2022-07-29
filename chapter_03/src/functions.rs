pub(crate) fn function_playground() {
    println!();
    println!("---");
    println!("Functions (Chapter 3)");
    println!("---");

    another_function("Peter");
    show_amount(6, 'k');
    expression_five();

    let y = {
        let x = 3;
        x + 2
    };

    println!("Value of y is {y}");
}

fn another_function(name: &str) {
    println!("Hallo {name}!");
}

fn show_amount(value: i8, unit: char) {
    println!("It amounts to {value}{unit}.");
}

fn expression_five() -> i32 {
    5
}
