
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// difference between enum vs struct ?
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    California,
    Texas,
    Utah,
    Nevada,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let localhost_v4 = IpAddrKind::V4(127, 0, 0, 1);
    let localhost_v6 = IpAddrKind::V6(String::from("127.0.0.1"));
    println!("Enum {:?}, {:?}", localhost_v4, localhost_v6);

    let x = 5;

    // option enum accepts a type and 
    // it can have some or None values
    let y: Option<i8> = None;
    let z: Option<i8> = Some(4);

    let sum = x + y.unwrap_or(0) + z.unwrap_or(0);
    println!(" sum {}", sum);

    value_in_coin(Coin::Quarter(UsState::California));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!(" sum , {} , {} ", six.unwrap(), none.unwrap_or(0));
}

fn plus_one(i: Option<i32>) -> Option<i32> {
    match i {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!(" state, {:?}", state);
            25
        },
    }
}
