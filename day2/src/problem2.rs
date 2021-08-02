use std::io;

fn main() {
    println!("Nth fibonacci number: ");

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to read line");

    let mut fact: u16 = inp.trim().parse().expect("Not a unsigned int 16 bit");

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    let result = if fact == 1 {
        0
    } else if fact == 2 {
        1
    } else {
        fact -= 2;
        loop {
            if fact == 0 {
                break c;
            }
            c = a + b;
            a = b;
            b = c;
            fact -= 1;
        }
    };

    println!("{}", result);
}