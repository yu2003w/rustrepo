/*
code examples for enumeration type.
It's really interesting to follow IpAddr as example.
https://github.com/rust-lang/rust/blob/b0170b693ef91460c97ff9ea5e360888466611dd/library/core/src/net/ip_addr.rs#L31
 */

enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn test_enum() {
    let msg = Message::Write(String::from("hello"));
    msg.call();
}

fn test_option() {
    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;
}