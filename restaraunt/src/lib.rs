mod front_of_house {
    pub struct Order {
        id: usize
    }

    impl Order {
        pub fn new(id: usize) -> Self {
            Self { id }
        }
    }

    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn server_order() {}

        pub fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    let a: front_of_house::Order;
    // The below line won't compile and I am so happy about that
    // let b = front_of_house::Order { id: 34 }; 
    a = front_of_house::Order::new(34);
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        use super::*;

        cook_order();
        deliver_order();
    }

    fn cook_order() {}
}