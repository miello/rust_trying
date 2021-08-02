use std::io;

fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");

    // let x = (let y = 6);

    let y = {
        // This is statement
        let x = 3;

        // Return as x + 1 (Note that will be last only)
        // This is expression
        x + 1
    };

    let p = five();
    let q = plus_one(p);

    println!("The value of y is {}, next is {}", y, q);

    // Section 2
    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Failed to read line");

    let number: i32 = number.trim().parse().expect("It's not a number");

    if number == 1 {
        println!("is one");
    }

    if number > 5 {
        println!("More than five");
    } else {
        println!("less than or equal five");
    }

    // Need to have compatible type such as i32 and string is invalid
    let num = if number == 1 {
        true
    } else {
        false
    };

    println!("{}", num);

    // For loop
    let mut counter = 0;
    let result = loop {
        // Rust do not have ++ T_T
        counter += 1;

        if counter == 10 {
            break counter * 20;
        }
    };

    println!("{}", result);

    let mut l = 30;
    while l != 0 {
        println!("{}!", l);
        l -= 1;
    }

    let a = [10, 10, 10, 10];

    for element in a.iter() {
        println!("Inside loop {}", element);
    }

    for n in (1..10).rev() {
        println!("{}!", n);
    }

    // Ownership system
    // :: => get function from namespace

    let mut s = String::from("Hello");

    #[warn(unused_variables)]
    let x = {
        let u = 10;

        // u will be automatic return 
        // when goes out of scope -> drop()
        u
    };

    s.push_str(", World");

    let w = 5;
    let r = w;

    // copy value 5 and bind to "r" -> push two '5' to stack (Work only scalar type)
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}, world!", s2);

    /*
    Maybe ?

    s1  -> ptr -> str1
        -> len
        -> capacity

    s2  -> ptr -> str1
        -> len
        -> capacity
    
    But it is not like this, rust have prevented the double free error 
    so it will move reference from s1 to s2 variable instead (Ownership rule)

    - Value will have only one owner (variable)

    Double free error -> If s1 and s2 is out of scope at the same time then s1 and s2 will be freed at the same memory location (str1)
    */

    let x1 = String::from("Hello");

    // Make a deep copy
    let x2 = x1.clone();

    let s1 = give_ownership();
    
    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);

    let (s4, len) = calculate_length(s3);

    // println!("{}", s3);
}

fn calculate_length(x: String) -> (String, usize) {
    let len = x.len();
    (x, len)
}

fn give_ownership() -> String {
    let some_string = String::from("Ownerrrrr");

    some_string
}

fn takes_and_gives_back(a: String) -> String {
    a
}

fn plus_one(i: i32) -> i32 {
    i + 1

    // Not this
    // i + 1;
}