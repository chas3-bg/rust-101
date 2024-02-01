fn main() {
    println!("Hello, world from rust");
    println!("{}", variables(20));
    let b: i8 = 10; //this is immutable
    println!("{b}"); // But b can be shadowed:
    let b: i32 = 123132;
    println!("{b}");
    let mut c: i8 = 10; //this is mutable
}

fn variables(i: i32) -> i32 {
    let a = i;
    a
}
