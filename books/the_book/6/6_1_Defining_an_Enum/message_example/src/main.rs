enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// methods defined on enum
impl Message {
    fn call(&self) {
        // to be defined
    }
}

fn main() {
    let m = Message::Write(String::from("Hello"));
    m.call();
}
