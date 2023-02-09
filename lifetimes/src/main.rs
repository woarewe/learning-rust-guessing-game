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

impl Len for str {
    fn len(&self) -> usize {
        self.len()
    }
}

fn longestv2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn new_str<'a>(first: &str, second: &str) -> String {
    // Won't work as well cause the value is owned by a function
    String::from("really long string")
}

fn main() {
    let a = String::from("short");
    let b = String::from("long long");


    println!("The longest is: {}", longest(&a, &b));

    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("short");
        result =  longestv2(string1.as_str(), string2.as_str());
        // Works just fine
        println!("The longest is {}", result);
    }
    // Won't be compiled
    // println!("The longest is {}", result);
}
