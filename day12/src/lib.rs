#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/// Add one to the number given
/// 
/// # Example
/// ```
/// let arg = 5;
/// let answer = day12::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ``` 
pub fn add_one(x: i32) -> i32 {
    x + 1
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoe = vec![
            Shoe {
                size: 10,
                style: String::from("A2"),
            },
            Shoe {
                size: 13,
                style: String::from("A1")
            },
            Shoe {
                size: 20,
                style: String::from("A3")
            }
        ];

        let in_my_size = shoes_in_size(shoe, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("A2"),
                },
            ]
        )
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1)).map(|(a, b)| a * b).filter(|x| x % 3 == 0).sum();

        assert_eq!(18, sum);
    }
}