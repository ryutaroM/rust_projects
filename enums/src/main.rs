fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrE::V4(String::from("127.0.0.1"));
    let loopback = IpAddrE::V6(String::from("::1"));

    let home = IpAddrOther::V4(127, 0, 0, 1);
    let loopback = IpAddrOther::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_num = Some(1);
    let some_string = Some("a string");

    let absent_num: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;

    value_in_cents_v3(CoinV2::Penny);

    //using enum in enum
    //Yay!
    value_in_cents_v3(CoinV2::Quarter(UsState::Alabama));
    main2();
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_type: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrE {
    V4(String),
    V6(String),
}

enum IpAddrOther {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //dummy
    }
}
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_v2(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc...
}

enum CoinV2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_v3(coin: CoinV2) -> u32 {
    match coin {
        CoinV2::Penny => 1,
        CoinV2::Nickel => 5,
        CoinV2::Dime => 10,
        CoinV2::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn main2() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);

    let some_u8_value = 3u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = CoinV2::Penny;
    let mut count = 0;

    match coin {
        CoinV2::Quarter(s) => println!("State quarter from {:?}!", s),
        _ => count += 1,
    }

    let coin = CoinV2::Penny;
    let mut count = 0;

    if let CoinV2::Quarter(s) = coin {
        println!("State quarter from {:?}!", s);
    } else {
        count += 1;
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// left not covered route
// fn plus_one_bug(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(n) => Some(n + 1),
//     }
// }
