// nested paths
// instead of this:
// use std::cmp::Ordering;
// use std::io;

// you can do this:
// use std::{cmp::Ordering, io};

// also possible
// global operator
// use std::collections::*;
mod front_of_house;

// this is done instead of ::hosting::add_to_waitlist, so you know where the function is defined
// use crate::front_of_house::hosting;

// alternative: this is called re-exporting. makes things public
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
}

use rand::Rng;

fn rand() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

// when bringing in structs, enums with use, you do this

use std::collections::HashMap;

fn test() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// when bringing in two items with the same name

use std::fmt;
use std::io;

// fn function1() -> fmt::Result {
// --snip--
// }

// fn function2() -> io::Result<()> {
// --snip--
// }

// or:

use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {
// --snip--
// }

// fn function2() -> IoResult<()> {
// --snip--
// }

pub fn eat_at_restaurant_old() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path

    front_of_house::hosting::add_to_waitlist();

    // not public, does not work
    // front_of_house::hosting::test();
    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
    let order3 = back_of_house::Breakfast::summer("cream");
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
