enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

// this is not possible with the struct
enum IpAddrDifferent {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let v4_addr = IpAddrKind::V4;
    let v6_addr = IpAddrKind::V6;

    route(v4_addr);

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // shorter:

    let home_improved = IpAddr::V4(String::from("127.0.0.1"));
    // the name of each enum variant that we define also becomes a function that constructs an instance of the enum.
    // That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.

    let home_different = IpAddrDifferent::V4(127, 0, 0, 1);

    impl Message {
        fn call(&self) {
            // method body would be defined here
            println!("{:?}", self);
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option enum (from stdlib)
    // Rust has no null
    //
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let some_number = Some(5);
    let some_string = Some("A string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // This does NOT work, since types are different!
    // let sum = x + y;

    // https://doc.rust-lang.org/std/option/enum.Option.html

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // exhaustive -> you have to list all possible cases in match, unless you do this catch-all...
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
    fn move_player(num_spaces: u8) {}

    // Concise Control Flow with if let
    // The if let syntax lets you combine if and let into a less verbose way to handle values
    // that match one pattern while ignoring the rest.

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // this is the same as

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        // ...
    }

    // Shorter:
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // If you have a situation in which your program has logic that is too verbose to express using a match, remember that if let is in your Rust toolbox as well.
    // example with else

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    // this is the same as

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}

fn route(ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
