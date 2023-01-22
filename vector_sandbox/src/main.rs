fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(7);
    match third {
        Some(third) => println!("The third element is  {third}"),
        None => println!("There is no third element!")
    }

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }


    for i in &v {
        println!("{i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ]
}
