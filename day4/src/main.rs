#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

enum IpAddrKind {
    V4(String),
    V6(String)
}

impl Rectangle {
    // Methods
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }
}

impl Rectangle {
    // Associated Function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

// Option<T>

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };
    
    let rect2 = Rectangle {
        height: 20,
        width: 40,
    };

    // Call the associated function
    let new_square = Rectangle::square(30);

    println!("Hello, world! {}", rect1.area());
    println!("{}", rect1.can_hold(&rect2));
    // Automatic referencing and dereferencing
    // p1.distance(&p2);
    // (&p1).distance(&p2);

    // Enumerate

    let four = IpAddrKind::V4;
    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("127.0.0.1")),
        address: String::from("127.0.0.1"),
    };

    let newMessage = Message::Move {
        x: 10,
        y: 10,
    };

    newMessage.call();
    let pure_number: i8 = 5;
    let some_number: Option<i8> = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    let number: i32 = 20;

    println!("{}", absent_number.is_some());
}
