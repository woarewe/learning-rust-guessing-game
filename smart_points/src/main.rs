#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}

impl<T> List<T> {
    pub fn cons(value: T, next: List<T>) -> List<T> {
        Self::Cons(value, Box::new(next))
    }
}

fn main() {
    let list = List::cons(3, List::cons(2, List::cons(1, List::Nil)));
    println!("{:?}", list);
}
