use bottles::{bottle_verse_template, Bottle};

fn main() {
    println!("{}", Bottle::new(bottle_verse_template).song());
}
