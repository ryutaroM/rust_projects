mod front_of_house;

pub fn eat_at_restaurant() {
    //absolue
    crate::front_of_house::hosting::add_to_waitlist();

    //relative
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("wheat");
    println!("I'd like {} toast plz", meal.toast);

    //cant compile
    // meal.seasonal_fruit = String::from("blueberrirs");
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

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

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use crate::front_of_house::hosting;

// use self::front_of_house::hosting;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant_v2() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use std::fmt;
// use std::io;

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> io::Result<()> {
    Ok(())
}

use std::fmt::Result as fr;
use std::io::Result as IoResult;

fn function1_v2() -> fr {
    Ok(())
}

fn funtion2_v2() -> IoResult<()> {
    Ok(())
}

fn huga() {
    hosting::huga();
}

fn main() {}

use std::io::{self, Write};
