use crate::garden::vegetables::Asparagus;

mod garden;

fn main() {
    let plant = Asparagus {};
    println!("My plant is {:?}!", plant);
}
