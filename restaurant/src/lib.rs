#![allow(unused)]

use std::collections::HashMap;  // idiomatic - full path for structs, enums, and other items (functions excluded)

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();  // Doesn't have to be public; a child can see it.
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

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("wheat");
    println!("I'd like {} toast, please.", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = crate::back_of_house::Appetizer::Salad;

    // Start using `use`.

    use crate::front_of_house::hosting;  // idiomatic

    hosting::add_to_waitlist();  // idiomatic
    add_to_waitlist();  // unidiomatic

    use crate::front_of_house::hosting::add_to_waitlist;  // unidiomatic

    hosting::add_to_waitlist();  // idiomatic
    add_to_waitlist();  // unidiomatic
    front_of_house::hosting::add_to_waitlist();  // We don't need `use` for this, but it still works.
}

fn deliver_order() {}

use crate::front_of_house::hosting;  // idiomatic

mod customer {
    use crate::front_of_house::hosting;  // idiomatic
    use crate::front_of_house::hosting::add_to_waitlist;  // unidiomatic

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();  // idiomatic
    }

    pub fn eat2() {
        use super::front_of_house::hosting::add_to_waitlist;  // unidiomatic

        add_to_waitlist();  // unidiomatic
    }
}


pub mod point {
  #[derive(Debug)]
  pub struct Point(pub i32, i32);

  impl Point {
    pub fn origin() -> Self { Point(0, 0) }
  }
}

fn play_with_point() {
  let mut p = point::Point::origin();
  p.0 += 1;
  println!("{p:?}");
}

fn play_with_hashmap() {
    let mut map = HashMap::new();  // idiomatic
    map.insert(1, 2);
}

use std::fmt;  // idiomatic
use std::io;  // idiomatic

fn function1() -> fmt::Result { Ok(()) }  // idiomatic
fn function2() -> io::Result<()> { Ok(()) }  // idiomatic

use std::fmt::Result;  // idiomatic
use std::io::Result as IoResult;  // idiomatic

fn function3() -> Result { Ok(()) }  // idiomatic
fn function4() -> IoResult<()> { Ok(()) }  // idiomatic

/// Example that shows how our clients (users of our code) can use a module that we have imported for our own use,
/// and that's the module `hosting` in this example.
///
/// Before this change, external code would have to call the `add_to_waitlist` function by using the path
/// `restaurant::front_of_house::hosting::add_to_waitlist()`. Now that this `pub use` has re-exported
/// the `hosting` module from the root module, external code can now use
/// the path `restaurant::hosting::add_to_waitlist()` instead.
pub mod re_exporting {
    pub use crate::front_of_house::hosting;  // re-exporting

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

use std::cmp::Ordering;
use std::alloc;
use std::array;
use std::array::from_fn;
use std::array::from_mut;

mod nested_imports {
    use std::{cmp::Ordering, alloc};
    use std::array::{self, from_fn, from_mut};
}

use std::borrow::*;  // Be careful with this! I personally am against it.
// The glob operator is often used when testing to bring everything under test into the tests module.
