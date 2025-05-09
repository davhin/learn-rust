// Implement XY for ValueWComment<T> where T implements Display
use std::fmt::Display;

struct ValueWComment<T> {
    value: T,
    comment: String,
}

trait XY {
    fn serialize(&self) -> String;
}

impl<T: Display> XY for ValueWComment<T> {
    fn serialize(&self) -> String {
        format!("Value {}, comment {}", self.value, self.comment)
    }
}

fn main() {
    let item = ValueWComment {
        value: 5,
        comment: "blablabla".to_string(),
    };
    println!("{}", item.serialize());
}
