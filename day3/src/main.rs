fn main() {
    // Ownership rules (Part 2)
    // Stack-only data: copy

    // Copy annotation -> It will place data on stack. The variable still usable after assignment
    // but the copy annotation cannot implement if type or any of its part have implemented Drop

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Reference and Borrowing
    let mut newStr = String::from("Hello World");
    let len = calculate_length(&mut newStr);

    println!("{}", len);

    // Prevent data races at compile time

    // Data races will cause when these condition are met
    // Two or more pointer access the same data at the same time
    // At least one of the pointers is being used to write to the data
    // There’s no mechanism being used to synchronize access to the data.
    {
        let s1 = &mut newStr;
    }
    let s1 = &newStr;
    let s2 = &newStr;

    println!("{} {}", s1, s2);

    let s3 = &mut newStr;
    s3.push_str("borrow");
    println!("{}", s3);
    println!("{}", newStr);

    let hello = dangle();
    let mut newData = String::from("Hello World");
    let firstWord = first_word(&newData);

    println!("{}", firstWord);

    // firstWord value is not valid now because of string is empty now

    // String Slices -> Like python
    // same as newData[0:5] in python
    let hello = &newData[0..5];
    let world = &newData[6..11];
    println!("{} {}", hello.len(), world.len());

    let twoOrMore = &newData[2..];
    let sliceFull = &newData[..];

    let mut newStrData = String::from("Hello Sekai");
    let word = first_word(&newStrData); // <- Immutable borrow

    // Error 
    // newStrData.clear();

    println!("{}", word);

    let s = "Hello World"; // <- String slices by Default

    let my_string = String::from("Hello World!!!");
    let word = first_word(&my_string[..]);

    let literal_str = "Hello World";

    // String literals are String slice
    let word = first_word(literal_str);

    // Slices of string literal
    let word = first_word(&literal_str[..]);

}

// s -> s1 -> String
// Parameter Borrowing
fn calculate_length(s: &mut String) -> usize {
    s.push_str("Hello");
    s.len()
}

// Return the borrowed value but the value that are borrowed are free by ownership rules
// (Out of bounds)
fn dangle() -> String {
    let s = String::from("Hello");

    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}