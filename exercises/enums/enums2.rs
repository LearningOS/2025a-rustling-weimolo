// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.



#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Quit,
    Move{x:i32,y:i32},    //  {} 在这里的作用是给变体携带的数据 “命名”，让代码更清晰、更不容易出错，尤其适合数据项较多或需要明确含义的场景

    ChangeColor(i32,i32,i32),
    Echo(String),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
