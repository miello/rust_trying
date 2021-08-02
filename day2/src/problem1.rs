use std::io;

fn main() {
    let mut tempC = String::new();

    println!("Input the temperature in celcius: ");
    io::stdin().read_line(&mut tempC).expect("");

    let c: f64 = tempC.trim().parse().expect("");

    let f = 9.0 * c / 5.0 + 32.0;
    println!("Converting to falenheit: {}", f);
}