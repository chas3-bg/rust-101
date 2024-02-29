/*
enum Result <T, E> {
    Ok(T),
    Err(E),
}
*/
use std::fs::File;
use std::io::ErrorKind;
//user for error propagation
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    /*
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
    */
    //bit shorter
    /*
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
     */

    //SHORTER!
    /*
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
     */

    //SHORTER!!!!!! but no error explanation or handling
    fs::read_to_string("hello.txt")
}

fn main() {
    println!("Hello, DO NO PANIC sometimes!");

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    //the above is the same as

    let greeting_file1 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //panics, but doesn`t create the file:

    let greeting_file2 = File::open("hello.txt").unwrap();

    //same as the above, but with extra error message

    let greeting_file3 = File::open("hello.txt")
        .expect("Failed to open file hello.txt, we need to include the file");

    println!("Username: {:?}", read_username_from_file());
}
