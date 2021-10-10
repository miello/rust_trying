#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1  {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        }

        else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    println!("I got the value {}", a);
    a + 2
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from(name)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // Must implement PartialEq and Debug trait
        assert_eq!(4, add_two(2));
        assert_eq!(10, add_two(8));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("World");
        assert!(result.contains("World"), "Greeting did not return properly, value was {}", result);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_should_panic() {
        Guess::new(200);
    }
    
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two not equal to four"))
        }
    }

    #[test]
    #[ignore]
    fn takes_very_long() {
        for i in 0..100000 {
            for j in 0..100000 {
                i + j;
            }
        }
    }

    #[test]
    fn it_internal_adder() {
        assert_eq!(internal_adder(3, 3), 6);
    }
}

// cargo test -> Test all
// cargo test -- --test-threads=1 -> Single Thread usage (Sequencial)
// cargo test -- --ignored -> Test ignored
// cargo test <prefix> -> Run all test that have prefix <prefix>