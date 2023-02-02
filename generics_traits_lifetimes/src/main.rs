fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

impl  <T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<X1, Y1>(self, other: Point<X1, Y1>) -> Point<T, Y1> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

impl Point<i32, i32> {
    fn int_x(&self) -> &i32 {
        &self.x
    }
}

fn main() {
   let number_list = vec![34, 50, 25, 101, 60];

   println!("The largest number is {}", largest(&number_list));

   let char_list = vec!['y', 'm', 'a', 'q'];
   println!("The largest char is {}", largest(&char_list));

   let both_integers = Point { x: 5, y: 10 };
   let both_float = Point { x: 1.2, y: 3.0 };
   let integer_and_float = Point { x: 5, y: 6.0 };

   println!("{:?} {:?} {:?}", both_integers, both_float, integer_and_float);

   println!("{:?}", both_integers.x());
   println!("{:?}", both_integers.int_x());


   let chars_and_strings = Point { x: 'a', y: "str" };
   let mixed = both_integers.mixup(chars_and_strings);

   println!("{:?}", mixed);

   // Won't compile
   // println!("{:?}", integer_and_float.int_x());

}
