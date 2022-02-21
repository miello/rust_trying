
use std::{ops::Deref, cell::RefCell};
use std::rc::Rc;


struct MyBox<T>(T);

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Drop data member {}", self.data);
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

enum Message {
    // No space
    Quit,

    // i32 2 variables
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

use crate::List::{Cons, Nil};

fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let a = Box::new(x);

    assert_eq!(5, *a);

    let test_box = MyBox::new(12);
    assert_eq!(12, *test_box);

    let s = MyBox::new(String::from("Hello"));
    hello(&s);

    let custom1 = CustomSmartPointer {
        data: String::from("Hello World"),
    };

    println!("Create custom1");
    let custom2 = CustomSmartPointer {
        data: String::from("Hello World 2"),
    };

    let a_x = Rc::new(Cons(Rc::new(RefCell::new(5)), Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil)))));
    println!("{}", Rc::strong_count(&a_x));
    
    let a_y = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a_x));
    println!("{}", Rc::strong_count(&a_x));

    let a_z = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a_x));
    println!("{}", Rc::strong_count(&a_x));

    {
        let a_u = Cons(Rc::new(RefCell::new(7)), Rc::clone(&a_x));
        println!("{}", Rc::strong_count(&a_x));
    }

    println!("{}", Rc::strong_count(&a_x));
    drop(custom1);
    println!("Create custom2");

    let value = Rc::new(RefCell::new(5));

    let a_1 = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b_1 = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a_1));
    let c_1 = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a_1));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a_1);
    println!("b after = {:?}", b_1);
    println!("c after = {:?}", c_1);
}
