//no need for main in modules
pub fn print_numbers() {
    let a: i8 = 8;
    let mut b: i8 = 9;
    println!("a is immutable by default {}", a);
    println!("b is mutable {}", b);
    b = 10;
    println!("b is now {}", b);
}
