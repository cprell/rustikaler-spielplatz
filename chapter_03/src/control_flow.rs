pub(crate) fn control_flow_playground() {
    println!();
    println!("---");
    println!("Control flow (Chapter 3)");
    println!("---");

    let number = 3;

    if number < 5 {
        println!("if cond true");
    } else if number == 3 {
        println!("else if condition true")
    } else {
        println!("else true");
    }

    let condition = true;
    let _conditional_number = if condition { 5 } else { 6 };

    let mut loop_number = 0;
    let result = loop {
        loop_number += 1;
        if loop_number == 5 {
            break loop_number * 2;
        }
    };

    println!("Result: {result}");

    // labeling loops
    // e.g. for breaking out of the outer loop in the inner loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut while_counter = 3;
    while while_counter != 0 {
        println!("While Counter: {while_counter}");
        while_counter -= 1;
    }

    // looping through collections with for
    let array = [10, 20, 30, 40, 50];

    for el in array {
        println!("Current element: {el}");
    }

    // alternative to while counters

    for num in (1..4).rev() {
        println!("{num}");
    }
}
