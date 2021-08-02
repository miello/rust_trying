use day8::Tweet;

fn largest(number_list: &[i32]) -> i32 {
    let mut large = number_list[0];

    for &number in number_list {
        if number > large {
            large = number;
        }
    }

    return large
}

// Generic function
// Error because of trait
fn new_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut large = list[0];

    for &each in list {
        if large < each {
            large = each;
        }
    }
    
    large
}

// struct Point<T> {
//     x: T,
//     y: T,
// }


// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}


fn main() {
    let number_list = vec![34, 50, 25, 100];

    let mut largest_1 = &number_list[0];

    for number in &number_list {
        if number > largest_1 {
            largest_1 = number;
        }
    }

    println!("{}", largest(&number_list));
    println!("{}", largest_1);

    let int_pos = Point { x: 3, y: 4 };
    let float_pos = Point { x: 1.2, y: 2.0 };
    // let error_pos = Point { x: 3, y: 1.0 };

    // println!("{}", float_pos.distance_from_origin());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: 10.4, y: 5 };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("abc"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
}
