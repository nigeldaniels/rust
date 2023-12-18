enum IpAddr {
    v4(u8, u8, u8, u8),
    v6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState{
    California,
    Alaska,
    Oregon,
}

fn main() {

    /*
    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1")
    };
    */

    //Concise version
    let home = IpAddr::v4(127, 0, 0, 1);
    let loopback = IpAddr::v6(String::from("::1"));


    let mut value = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Value: {}", value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("values of five {:?}, plus one :{:?} , plus none {:?}", five, six, none)
}

// An enum and a match expression that has the variants of the enum as its patterns
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("This quarter is from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
