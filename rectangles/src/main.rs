#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}


fn main() {
    let scale = 2;
    let rectangle = Rectangle {
        height: dbg!(30 * scale),
        width: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle)
    );
    println!("Rectangle: {:#?}", rectangle);
    dbg!(&rectangle);
}

fn area(rectangle: &Rectangle) -> u32 {
   rectangle.width * rectangle.height
}
