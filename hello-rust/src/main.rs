use ferris_says::say;
use std::io::{stdout, BufWriter};


fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    let _ = say(&message, width, &mut writer);

    //note this gets executed before the say function.
    println!("Hello, world!");
}
