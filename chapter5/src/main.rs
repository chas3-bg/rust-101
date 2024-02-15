//mod structs;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64,
}
//using tuple structs with nameless fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//unit structs that holds no data:
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
impl Rectangle {
    fn calculate_area(&self) -> i32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Hello, world!");
    //init shorthand - fn parameters have the same name as struct fields, so use less code
    let user1 = build_user(String::from("user1"), String::from("john@doe.qw"));
    //struct update syntax
    let user2 = User {
        email: String::from("doe@john.qw"),
        ..user1
    };
    //println!("{}; {}", user1.username, user1.email);
    //error: field `username` of struct `User` is moved, so it cannot be borrowed
    println!("{}; {}", user2.username, user2.email);

    let black: Color = Color(0, 0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    dbg!(&rect1);

    println!("The rectangle is {:#?}", rect1);

    //let area1 = calculate_area(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.calculate_area()
    );

    let sq1 = Rectangle::square(3);
    println!(
        "The area of the square is {} square pixels.",
        sq1.calculate_area()
    );
}
