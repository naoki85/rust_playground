// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32, i32),
// }
//
// impl Message {
//     fn call(&self) {
//         match self {
//             Message::Quit => {
//                 println!("Quit: No data");
//             }
//             Message::Move { x, y } => {
//                 println!("Move to cordinates: x = {}, y = {}", x, y);
//             }
//             Message::Write(text) => {
//                 println!("Write message: {}", text);
//             }
//             Message::ChangeColor(r, g, b, a) => {
//                 println!("Change color to RGBA({}, {}, {}, {})", r, g, b, a)
//             }
//         }
//     }
// }

fn main() {
    // let messages = [
    //     Message::Quit,
    //     Message::Move { x: 10, y: 20 },
    //     Message::Write(String::from("New message")),
    //     Message::ChangeColor(100, 125, 150, 175),
    // ];
    // for message in &messages {
    //     message.call();
    // }
    let some_u8_value = Some(0u8);
    let a = if let Some(3) = some_u8_value {
        "three"
    } else {
        "not three"
    };
    println!("{}", a);
}
