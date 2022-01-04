mod parser;

fn main() {
    let subs = parser::from_file("sintel_en.srt");
    println!("{:?}", subs);
}
