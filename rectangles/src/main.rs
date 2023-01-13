#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
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
    println!("Rectangle: {:#?}", rectangle);
    dbg!(&rectangle);
}