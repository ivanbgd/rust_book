/// Module declarations
mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;
pub use crate::back_of_house::kitchen;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

pub fn prepare_meal() {
    kitchen::cook_order();
}
