use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("My plant is {:?}!", plant);
    println!("My plant is {:?}", garden::Animal {});
}
