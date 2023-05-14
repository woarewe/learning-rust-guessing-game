#[derive(Debug)]
enum BoxBasedList<T> {
    Cons(T, Box<BoxBasedList<T>>),
    Nil
}

#[derive(Debug)]
struct BoxBasedTriplet<T>(BoxBasedList<T>, BoxBasedList<T>, BoxBasedList<T>);

fn build_box_based_triplet() -> BoxBasedTriplet<u32> {
    let common = BoxBasedList::Cons(1, Box::new(BoxBasedList::Cons(2, Box::new(BoxBasedList::Nil))));
    let second = BoxBasedList::Cons(3, Box::new(BoxBasedList::Nil));
    let third = BoxBasedList::Cons(4, Box::new(BoxBasedList::Nil));
    return BoxBasedTriplet::<u32>(common, second, third);
}

fn main() {
    println!("#{:?}", build_box_based_triplet());
}
