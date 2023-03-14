trait Valueable<T> {
    fn value(&self) -> T;
}

#[derive(Debug)]
struct Person<T> {
    value: T 
}

trait Ageable {
    fn age(&self) -> usize;
}

impl<T: Ageable> Person<T> {
    fn age(&self) -> usize {
        self.value.age()
    }
}

fn main() {
}
