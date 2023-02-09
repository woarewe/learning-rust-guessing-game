trait Len {
    fn len(&self) -> usize;
}

fn longest<'a, T>(first: &'a T, second: &'a T) -> &'a T
where T: Len {
    if first.len() > second.len() {
        return first;
    } else {
        return second;
    }
}

impl Len for String {
    fn len(&self) -> usize {
        self.len()
    }
}

fn main() {
    let a = String::from("short");
    let b = String::from("long long");


    println!("The longest is: {}", longest(&a, &b));
}
