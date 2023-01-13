fn main() {
    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect)
    );
}

fn area(dimensinos: (u32, u32)) -> u32 {
    match dimensinos {
        (a, b) => a * b
    }
}
