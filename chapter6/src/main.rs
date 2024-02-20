
enum IPAddrType {
    V4(u8, u8, u8, u8), //you can attach datatype to each variant
    V6(String),
}

fn route (ip_kind: IPAddrType){

}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        println!("{:?}", &self)
    }
}

enum Option<T> {
    None,
    Some(T),
}
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    Arizona,
}


enum Coins{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coins) -> i8 {
    match coin {
    Coins::Penny => 1,
    Coins::Nickel => 5,
    Coins::Dime => 10,
    Coins::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        25
    }
    
}
}
fn main() {
    println!("Hello, chapter 6!");
    let four = IPAddrType::V4;
    let six = IPAddrType::V6;

    let homev4 = IPAddrType::V4(127, 0, 0, 1);
    let loopbackv6 = IPAddrType::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    println!("{}",value_in_cents(Coins::Quarter(UsState::Alaska)));

    let coin = Coins::Penny;
    let mut counter = 0 ;
    if let Coins::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        counter += 1;
    }






}
