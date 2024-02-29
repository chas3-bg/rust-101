enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3]; //infered type

    v.push(5);
    v.push(6);
    v.push(7);

    let five: &i32 = &v[0];
    println!("The value of five is: {}", five);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("Value of the third element is {third}"),
        None => println!("There is no second third"),
    }
    //some error handling with Option<T>
    let mut v2 = vec![10, 20, 30, 40, 50];

    for i in &mut v2 {
        *i += 5;
    }

    for i in 0..v2.len() + 1 {
        let member: Option<&i32> = v2.get(i);
        match member {
            Some(member) => println!("The value of the {i} element is {member}"),
            None => println!("There is no {i} element"),
        }
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
}
