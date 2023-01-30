fn largest(list: &[i32]) -> &i32 {
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
}
