
// semicolon will load from other file
mod front_of_house;

// bringing into current scope and reexporting
// so external modules can access this scope
pub use crate::front_of_house::hosting;
pub use crate::front_of_house::serving;

use crate::front_of_house::hosting::add_to_waitlist;
use crate::front_of_house::serving::serve_order;
// use self::front_of_house::hosting;

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // private
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        // associated function
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn cook_order() {

    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::serving::serve_order();
        super::serve_order();
    }
}

// careful about using same name for two types
// use std::fmt::Result;
// use std::io::Result as IoResult;
use std::collections::HashMap;

// we can bring multiple items into scope with single line
// same as use std::io, std::io::Write
// use std::io::{self, Write};
// use std::{cmp::Ordering, io};

// glob operator
// use std::collections::*;

pub fn eat_at_restaurant() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // relative path sibling to eat_at_resturant
    // front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    // change the toast
    meal.toast = String::from("Wheat");
    println!("I'd like to have {:?} meal", meal);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!(" order {:?}, {:?}", order1, order2);

    // already hosting in scope
    hosting::add_to_waitlist();

    // alredy add_to_waitlist in scope
    add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1,2);
}

