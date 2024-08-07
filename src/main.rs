
// enum IpAddress {
//     V4(u32, u32, u32, u32), 
//     V6(String),
// }
// struct IpAddr {
//    kind: IpAddress,   
// }
// fn main() {
//     // let four = IpAddress::V4;
//     // let six = IpAddress::V6;
//     // let home = IpAddr {
//     //     kind: IpAddress::V4(String::from("127.0.0.1")),
//     // };
//     // let loopback = IpAddr {
//     //     kind: IpAddress::V6,
//     //     address: String::from("::1"),
//     // };
//     // let home = IpAddress::V4(String::from("127.0.0.1"));
//     let loopback = IpAddress::V6(String::from("::1"));
//     let home = IpAddress::V4(127, 0, 0, 1);
// }


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny, 
    Nickel,
    Dime, 
    Quarter(UsState),   
}
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Монета на счастье!");
//             1},
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("Четвертак из штата {:?}!", state);
//             25
//         },
//     }
// }



fn plus_one(x: Option<i32>)-> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main() {
    let five = Some(5);
    let none = None;
    let result1 = plus_one(five);
    let result2 = plus_one(none);
    println!("{:?}, {:?}", result1, result2);
    let some_value = Some(3);

    if let Some(3) = some_value {
        println!("три!");
    }

    let mut count = 0;
    let coin: Coin = Coin::Quarter(UsState::Alabama);
    match coin {
        Coin::Quarter(state) => println!("Четвертак из штата {:?}!", state),
        _ => count+= 1,
    }
    
}


// enum Message {
//
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// enum Option<T> {
//     Some(T),
//     None,
// }
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);
// impl Message {
//     fn call(&self) {
//     }
// }
// fn main() {
//     let m = Message::Write(String::from("Hello"));
//     m.call();
//     let c = Some(String::from("World"));
//     let a = Some(12);
//     let null: Option<i32> = None;
// }