#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width() * self.height
    }

    fn width(self: &Self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let scale = 2;
    let rectangle = Rectangle {
        height: dbg!(30 * scale),
        width: 50
    };
    let smaller_one = Rectangle {
        height: 20,
        width: 10
    };
    let bigger_one = Rectangle::square(100);

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );
    println!("Rectangle width: {:#?}", rectangle.width());
    dbg!(&rectangle);

    println!("Can {:?} hold {:?} ? - {}", rectangle, smaller_one, rectangle.can_hold(&smaller_one));
    println!("Can {:?} hold {:?} ? - {}", rectangle, bigger_one, rectangle.can_hold(&bigger_one));
}