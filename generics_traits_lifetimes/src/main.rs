fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}


fn main() {
   let number_list = vec![34, 50, 25, 101, 60];

   println!("The largest number is {}", largest(&number_list));

   let char_list = vec!['y', 'm', 'a', 'q'];
   println!("The largest char is {}", largest(&char_list));
}
