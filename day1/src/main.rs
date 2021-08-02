use std::io;

fn another_function(x: usize) {
    println!("Hello From Another function {}", x);
}

fn main() {
    let mut x = 8;
    println!("Before, {}", x);

    x = 9;
    println!("After, {}", x);

    let shadow = 6;
    println!("{}", shadow);

    let shadow = shadow * 4;
    println!("{}", shadow);

    let shadow = shadow * 9;
    println!("{}", shadow);

    // Same type shadowing !!!
    // let space = "    ";
    // space = space.len();

    // Need to specify type !!!
    let guess: u32 = "52".parse().expect("Not a number");

    let po = 50;
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let a = 2.0; // Default f64
    let b: f32 = 3.0; // Override f32
    let int = 98_222;
    let hex = 0x222;
    let oct = 0o123;

    let bin = 0b0110_1010;
    let byte = b'a';

    let tup = (500, 43.4, true);

    let (u, v, w) = tup;

    let arr = [1, 2, 3, 4];

    println!("{} {} {}",u , v, w);

    let mut inp = String::new();

    io::stdin().read_line(&mut inp).expect("Failed to read line");

    let inp: usize = inp.trim().parse().expect("Failed to parse to integer");

    let element = arr[inp];

    another_function(inp);
    println!("Hello {}", element);
}
