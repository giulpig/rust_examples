/* EXAMPLE 0 - "use" statement*/

// Bring hosting module into scope and rename it
// "pub" at the beginning makes the toagne module (housing)
// accessible to all modules that use this module (crate)
pub use crate::front_of_house::hosting as toagne;

// Bring to scope and make it public for every module that
// import this module
pub use crate::front_of_house::hosting;

// Multiple "use" at once
use std::io::{self, Write};

// Glob operator (star)
use std::io::*;

// Bring ciao module to scope
mod ciao;


///////////////////////////////////////////////////////////////////


/* EXAMPLE 1 - modules and functions */

// This is in the same level as eat_at_restaurant so I dont have
// to make it pub, pub only exposes the module to ancestors
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path, self is redundant
    self::front_of_house::hosting::add_to_waitlist();
}


///////////////////////////////////////////////////////////////////


/* EXAMPLE 2 - use of super */

fn serve_order() {}

// I can access ancestors with super keyword
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

///////////////////////////////////////////////////////////////////


/* EXAMPLE 3 - structs */

mod back_of_house_2 {
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

pub fn eat_at_restaurant_2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house_2::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

///////////////////////////////////////////////////////////////////


/* EXAMPLE 4 - enums, fields are public by default */

mod back_of_house_3 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_3() {
    let order1 = back_of_house_3::Appetizer::Soup;
    let order2 = back_of_house_3::Appetizer::Salad;
}
