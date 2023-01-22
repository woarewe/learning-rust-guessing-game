mod front_of_house {
    pub struct Order {}

    impl Order {
        pub fn new() -> Self {
            Self {}
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
    a = front_of_house::Order::new();
}