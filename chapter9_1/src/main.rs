fn main() {
    println!("Hello, PANIC!");
    let v = vec![1, 2, 3];
    v[3];
    //panic!("Crash and burn")
}
