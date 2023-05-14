#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> { 
    fn new(value: T) -> Self {
        Self(value)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(x, *y);
    println!("{:?}", y);

    let mut value = String::from("lol");
    let reference: &mut str = &mut value;
    let another = &value;

    println!("{} {}", reference, another);
}
