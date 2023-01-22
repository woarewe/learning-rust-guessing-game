mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaraunt() {
    hosting::add_to_waitlist();
}