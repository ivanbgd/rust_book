#![allow(unused)]

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route (ip_kind: IpAddrKind) {

}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

// This is included in stdlib, and in prelude!
enum _Option<T> {
    None,
    Some(T),
}

// This works...
struct Result1<T, E> {
    ok: Option<T>,
    err: Option<E>,
}

// ...but this is more idiomatic in Rust in this case.
enum Result2<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {  // Works with &Coin, too.
    match coin {  // The expression (here, `coin`) can be any type, not only Boolean.
        Coin::Penny => {
            println!("Lucky coin!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}.", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}


fn main() {
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    route(ipv4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    let msg = Message::Write(String::from("Hello!"));
    msg.call();

    let some_number = Some(5);
    let some_char: Option<char> = Some('e');
    let absent_number: Option<i32> = None;
    println!("{:?}, {:?}, {:?}", some_number, some_char, absent_number);

    let x: i8 = 5;
    let mut y: Option<i8> = Some(6);
    let z1 = x + y.unwrap();
    let mut z2: i8 = -7;
    if y.is_some() {
        z2 = x + y.unwrap();
    }
    println!("{}, {}", z1, z2);
    y = None;
    if y.is_none() {
        z2 = x + 0;
    }
    println!("{}", z2);
    let z3 = x + y.unwrap_or_default();
    println!("{z3}\n");

    let mut coin = Coin::Quarter(UsState::Alaska);
    println!("{:?} => {}\n", coin, value_in_cents(&coin));
    coin = Coin::Penny;
    println!("{:#?} => {}\n", coin, value_in_cents(&coin));
    coin = Coin::Nickel;
    println!("{:#?} => {}\n", coin, value_in_cents(&coin));

    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}, {:?}", five, six);
    let none = plus_one(None);
    println!("{:?}\n", none);

    let dice_roll = 9;
    // We can use either one of "other" or "_", or even both.
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other=> move_player(other),
        _ => reroll(),
        _ => (), // Nothing will happen. This is the unit value, the empty tuple.
    }

    let optstr: Option<String> = Some(String::from("Hello, World!"));
    match optstr {
        Some(_) => println!("Some"),
        None => println!("None")
    };
    println!("{:?}\n", optstr);
    ;;;
    match &optstr {
        Some(s) => println!("Some: {}", s),
        None => println!("None")
    }
    println!("{:?}\n", optstr);

    println!("{} {:?}", 3u8, Some(3u8));
    let mut config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),  // We match one pattern.
        _ => (),  // We are ignoring the rest.
    }
    // This is shorter, but it loses exhaustive checking that `match` enforces.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // We can add `else` branch.
    config_max = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    else {
        println!("None arm");
    }
    println!();

    let mut count = 0;
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }
    println!("Coin = {:?}; count = {count}\n", coin);
    coin = Coin::Quarter(UsState::Alaska);
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }
    println!("Coin = {:?}; count = {count}\n", coin);
    coin = Coin::Penny;
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}", state);
    }
    else {
        count += 1;
    }
    println!("Coin = {:?}; count = {count}\n", coin);
}
