use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let rng = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", rng);
    
    loop {
        let mut guess = String::new();
        println!("Input your guess: {}", guess);

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rng) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Hello World");
                break;
            }
        } 
    }
}