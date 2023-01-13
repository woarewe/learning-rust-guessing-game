#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}


fn main() {
    let rectangle = Rectangle {
        height: 30,
        width: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle)
    );
    println!("Rectangle: {:#?}", rectangle);
}

fn area(rectangle: &Rectangle) -> u32 {
   rectangle.width * rectangle.height
}
