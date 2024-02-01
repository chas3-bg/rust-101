use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn generate_random_number() -> u32 {
    let rng = rand::thread_rng().gen_range(0..101);
    rng
}

fn main() {
    println!("WELCOME TO THE GUESSING GAME!");
    println!("Please input your guess");
    let random_int = generate_random_number();
    loop {
        let input: u32 = match get_input().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", input);
        match input.cmp(&random_int) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
