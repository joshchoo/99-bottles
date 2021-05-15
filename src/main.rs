use bottles::{BottleVerseFactory, CountdownSong};

fn main() {
    println!(
        "{}",
        CountdownSong::new(Box::new(BottleVerseFactory), 99, 0).song()
    );
}
