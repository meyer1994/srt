use std::fs;

mod parser;

fn main() {
    let data = fs::read_to_string("sintel_en.srt").unwrap();
    let subs = parser::from_str(&data);
    println!("{:?}", subs);
}
