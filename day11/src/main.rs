use std::thread;
use std::time::Duration;

struct Cacher<T>
where 
    T: Fn(u32) -> u32, 
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> 
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn simulated_long_time(intensity: u32) -> u32 {
    println!("Calculated...");
    thread::sleep(Duration::from_secs(2));

    intensity
}

/*
 // Function 
 fn  add_one_v1   (x: u32) -> u32 { x + 1 }

 // Fully annotate closure
 let add_one_v2 = |x: u32| -> u32 { x + 1 };

 // remove type annotate
 let add_one_v3 = |x|             { x + 1 };

 // remove bracket
 let add_one_v4 = |x|               x + 1  ;
*/
fn generate_workout(intensity: u32, random_number: u32) {
    // not need to annotate type
    let mut expansive_closure = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    /* let example_closure = |x| x;

       // Indicate that type annotate for closure is string
       
       let s = example_closure(String::from("hello"));
       let n = example_closure(5);
    */
    // let expansive_result = simulated_long_time(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expansive_closure.value(intensity));
        println!("Next, do {} situps!", expansive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");  
        } else {
            println!("Today, run for {} minutes!", expansive_closure.value(intensity));
        }
    }
}

fn main() {
    let simulated_specific = 10;
    let simulated_random = 5;

    generate_workout(simulated_specific, simulated_random);

    let x = 4;
    let equal_to_x = |z| z == x; // Fn trait -> Borrow x immutably

    // Doesn't work because of overhead prevention
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }

    /*
        Closure Fn trait:

        FnOnce -> take ownership from environment
        FnMut -> Borrowing mutably
        Fn -> Borrowing immutably
    */
    let y = 4;
    assert!(equal_to_x(y));

    let a = vec![1, 2, 3];
    
    let move_equal_to_x = move |z| z == a;
    // println!("What is x: {:?}", a);

    let y = vec![1, 2, 3];

    assert!(move_equal_to_x(y));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn call_with_different_value() {
        let mut c = Cacher::new(|a| a);
        
        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}