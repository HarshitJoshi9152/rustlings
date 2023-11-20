// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
struct MoveStruct {
    x: u32,
    y: u32
}

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
enum Message {
    Move(MoveStruct),
    Echo(String),
    ChangeColor(Color),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move(MoveStruct { x: 10, y: 30 } ),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(Color( 200, 255, 255 )),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
