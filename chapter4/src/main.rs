mod refference_borrowing;

fn main() {
    {
    let s = "Hello from scope 1";
    println!("{}",s);
    }
    println!("This is scope 2 and var s is out of scope!");

    let mut s1 = String::from("Hello from scope 2");
    s1.push_str(" and I am mutable");
    println!("{}", s1);
    let s2 = s1;
    println!("s1 is no longer valid, it is moved to s2"); 
    let s3 = s2.clone();
    //this is a clone, heap memory is coped and s2 and s3 are valid, as they point
    //to different heap memory
    //copy is implemented automatically (as a trait) to items on the stack (ordered, known
    //size, primitive types)
    //same goes for passing args to functions:

    let s4 = String::from("This is a string4");
    takes_ownership(s4);
    println!("s4 is no longer in the main scope");

    let x :i32 = 5;
    println!("{} is in main scope", x);
    make_copy(x);
    println!("{} is still in main scope",x);

    refference_borrowing::calculate_lenght(&s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
    
}

fn make_copy(int: i32) {
    println!("{} in make_copy scope", int)
}