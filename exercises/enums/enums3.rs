// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.



enum Message {
    // TODO: implement the message variant types based on their usage below
    ChangeColor(i32,i32,i32),
    Echo(String),
    Move(Point),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) { self.message = s }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message
        // variants
        // Remember: When passing a tuple as a function argument, you'll need
        // extra parentheses: fn function((t, u, p, l, e))
        /*  我的答案
            1.在 Rust 的 match 表达式中，匹配枚举变体时需要使用“枚举名称：：变体名称”的完整路径
            2.枚举变体携带的数据如何在match表达式中被正确捕获和使用。   这是最重要的问题
                简单说：枚举变体像一个 "容器"，装着处理所需的数据，你的代码现在只检查了 "容器类型"，却没把 "容器里的东西" 取出来用
                提取枚举变体携带的数据需要使用模式匹配，具体做法是在枚举变体名称后加上括号，并在括号中定义变量来接收数据。
                例如：Message::ChangeColor(r, g, b) 这里的 r, g, b 就是用来接收 ChangeColor 变体携带的三个 i32 数据
                
        match  message {
            Message::ChangeColor(r,g,b) => self.change_color((255,0,255)),
            Message::Echo(s) => self.echo(String::from("hello world")),
            Message::Move(p) => self.move_position (Point { x: 10, y: 15 }),
            Message::Quit => self.quit(),
        };

        */
        //标准答案
        match message {
            Message::ChangeColor(r, g, b) => {
                self.change_color((r as u8, g as u8, b as u8));
            }
            Message::Echo(s) => {
                self.echo(s);
            }
            Message::Move(p) => {
                self.move_position(p);
            }
            Message::Quit => {
                self.quit();
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(), 
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "hello world");
    }
}
