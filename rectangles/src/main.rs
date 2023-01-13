#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(self: &Self) -> u32 {
        self.width
    }
}

fn main() {
    let scale = 2;
    let rectangle = Rectangle {
        height: dbg!(30 * scale),
        width: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );
    println!("Rectangle width: {:#?}", rectangle.width());
    dbg!(&rectangle);
}