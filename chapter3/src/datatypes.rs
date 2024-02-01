pub fn do_math(a: i32, b: i32) {
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);
}

pub fn bools() {
    println!("Those are bools:");
    let boo: bool = true;
    let boo2: bool = false;
    println!("Those are booleans:\n {}\n {}", boo, boo2);
}

pub fn strings() {
    let c: char = 'c';
    let cat: char = '😺';
    println!("Those are char:");
    println!("c = {}", c);
    println!("cat = {}", cat);

    let word: String = String::from("Stringy");
    println!("Those are strings: {}", word);
}

pub fn tuples() {
    let tup: (i32, f64, u8, char, String) = (500, 6.4, 1, 'c', "string".to_string());
    println!("This is tuple: {:?}", tup);
    println!(
        "Those are tuples elements: {} {} {} {} {}",
        tup.0, tup.1, tup.2, tup.3, tup.4
    );
    let (w, v, x, y, z) = tup;
    println!(
        "Those are deconstructed tuple values: {} {} {} {} {}",
        w, v, x, y, z
    );
}

pub fn arrays() {
    //arrays are placed on the stack
    let arr: [i32; 3] = [1, 2, 3];
    println!("This is array: {:?}", arr);
    println!("This is the first element of the array: {}", arr[0]);
}
