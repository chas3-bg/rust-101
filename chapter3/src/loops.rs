pub fn ifs() {
    let num1: i8 = 5;
    if num1 > 5 {
        println!("num1 is greater than 5");
    } else if num1 < 5 {
        println!("num1 is less than 5");
    } else {
        println!("num1 is  5")
    }

    let cond = true;
    let num2 = if cond { 10 } else { 20 };
    println!("num2 is {}", num2);
}

pub fn loops() {
    let mut counter: i8 = 0;
    loop {
        if counter < 10 {
            println!("counter is {}", counter);
            counter += 1;
        } else {
            break;
        }
    }
    counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {}", result);
    while counter != 0 {
        println!("counter is {}", counter);
        counter -= 1;
    }
    println!("Liftoff!!");

    println!("This is a for loop");
    let a: [i8; 5] = [1, 2, 3, 4, 5];
    for i in a {
        println!("i is {}", i);
    }
    println!("This is a for loop with range");
    for i in 0..5 {
        println!("i is {}", i);
    }
    println!("LIftoff 2");
}
