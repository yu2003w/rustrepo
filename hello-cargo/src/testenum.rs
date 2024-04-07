/*
code examples for enumeration type.
It's really interesting to follow IpAddr as example.
https://github.com/rust-lang/rust/blob/b0170b693ef91460c97ff9ea5e360888466611dd/library/core/src/net/ip_addr.rs#L31
 */

enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn test_enum() {
    let msg = Message::Write(String::from("hello"));
    msg.call();
}

fn test_option() {
    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;
    println!("some number is {}", some_number.unwrap());
}

enum Coins {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn for_value_in_cents(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickle => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
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
        Some(i) => Some(i+1)
    }
}

fn test_match_option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// concise control using if let
fn test_concise_flow() {
    // sample code for using enum
    let cfg_max = Some(5u8);
    match cfg_max {
        Some(max) => println!("The maximum number is configured to {}", max),
        _ => (),
    }

    //another option is to use if let, and if let will lose exhaustive
    if let Some(max) = cfg_max {
        println!("The maximum number is {} with if let", max);
    }

}

