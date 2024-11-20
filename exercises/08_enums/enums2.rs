#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move{x: i32, y: i32}, // object struct
    Echo(String), // String type
    ChangeColor(i32, i32, i32), // tuple struct
    Quit // no defined type
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move{ x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit
    ];

    for message in &messages {
        message.call();
    }
}