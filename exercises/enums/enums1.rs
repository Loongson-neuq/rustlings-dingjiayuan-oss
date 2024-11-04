// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move(i32, i32),
    ChangeColor(u8, u8, u8),
}

fn main() {
    let messages = [
        Message::Quit,
        Message::Echo("Hello".to_string()),
        Message::Move(3, 4),
        Message::ChangeColor(255, 0, 0),
    ];
    for message in messages.iter() {
        println!("{:?}", message);
    }
}
