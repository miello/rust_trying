// pub mod front_of_house;

// use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant_2() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

pub fn serve_order() {

}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub use back_of_house::Breakfast;

pub fn eat_at_restaurant() {
    // // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // // Relative path
    // front_of_house::hosting::add_to_waitlist()

    let mut meal = Breakfast::summer("Butter");

    meal.toast = String::from("Rice");
    println!("{}", meal.toast);
}