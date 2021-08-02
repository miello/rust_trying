use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::error::Error;

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("hello.txt")?;

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };

    fs::read_to_string("hello.txt")    

    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Red"), 40);

    let team = vec![String::from("Blue"), String::from("Red")];
    let init_score = vec![10, 50];

    let mut new_score: HashMap<_, _> = team.into_iter().zip(init_score.into_iter()).collect();

    let field_name = String::from("Color");
    let field_value = String::from("Red");

    let team_name = String::from("Blue");

    let get_name = scores.get(&team_name);

    new_score.insert(String::from("Yellow"), 100);
    // new_score.entry(String::from("Yellow")).insert(100);
    new_score.entry(String::from("Yellow")).or_insert(200);

    for (key, value) in &new_score {
        println!("{}: {}", key, value);
    }

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    println!("{:?}", get_name);

    let v = vec![1, 2, 3];
    // v[99];

    // let f = File::open("Hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("Hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating file: {:?}", e)
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     }
    // };

    let f = (File::open("Hello.txt")).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem creating the file: {:?}", error);
        }
    });

    // Unwrap will throw panic! if err -> print same message
    // let f = File::open("Hello.txt").unwrap();

    // Easier to find where is panic
    // let f = File::open("Hello.txt").expect("Hello World");
    
    // println!("{:?}", f);

    println!("{:?}", read_username_from_file());

    let f = File::open("hello.txt")?;
    Ok(())

}
