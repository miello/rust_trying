mod my;

use std::{fmt, collections::HashMap};
use std::io::{self, Write};

// Bring to scope everything
// use std::collections::*;
use rand::Rng;

#[derive(Debug)]
enum UsState {
    State1,
    State2,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
    // Handle None
    // match x {
    //     Some(i) => Some(i + 1)
    // }
}

// fn function1() -> fmt::Result {}

// fn function2() -> IoResult<()> {}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    println!("Hello, world!");
    const HELLO: Coin = Coin::Dime;
    const DATA: Coin = Coin::Quarter(UsState::State1);
    println!("{:} {}", value_in_cents(HELLO), value_in_cents(DATA));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let u8_value = 0u8;
    match u8_value {
        1 => println!("1"),
        3 => println!("3"),
        5 => println!("5"),
        7 => println!("7"),
        _ => println!("Not found")
    }

    let some_value = Some(3u8);
    if let Some(3u8) = some_value {
        println!("three float")
    }

    let mut count = 0;
    match DATA {
        Coin::Quarter(state) => println!("Hello Sekai {:?}", state),
        _ => count += 1,
    }
    println!("count = {}", count);

    // Equivalent to
    if let Coin::Quarter(state) = DATA {
        println!("Hello Sekai {:?}", state);
    } else {
        count += 1;
    }
    println!("count = {}", count);

    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 2);

    let rng = rand::thread_rng().gen_range(1..101);
    // lib::hosting::add_to_waitlist();
    my::nested::hosting::hello_world();

    let mut v: Vec<i32> = Vec::new();
    let vValue = vec![1, 2, 3, 4];

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);

    {
        let mut w: Vec<i64> = Vec::new();
    }

    let third: &i32 = &v[2];
    println!("Third element is {}", third);

    match v.get(2) {
        Some(third) => println!("Third element is {}", third),
        None => (),
    }

    if let Some(third) = v.get(999) {
        println!("Third element is {}", third);
    } else {
        println!("Not found")
    }

    for i in &v {
        println!("{}", i);
    }

    v.push(6);
    let first = &v[0];
    println!("{}", first);

    for i in &mut v {
        *i += 50;
    }

    println!("Test: {:?}", v);
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(4.5f64),
        SpreadsheetCell::Text(String::from("Hello World"))
    ];
}
