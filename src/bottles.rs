use core::fmt;
use std::any::Any;

pub trait VerseTrait {
    fn lyrics(&self, number: i32) -> String;
}

type VerseTemplate = Box<dyn VerseTrait>;

pub struct CountdownSong {
    verse_template: VerseTemplate,
    max: i32,
    min: i32,
}

impl Default for CountdownSong {
    fn default() -> Self {
        Self::new(Box::new(BottleVerseFactory), 99, 0)
    }
}

impl CountdownSong {
    pub fn new(verse_template: VerseTemplate, max: i32, min: i32) -> Self {
        CountdownSong {
            verse_template,
            max,
            min,
        }
    }

    pub fn song(&self) -> String {
        self.verses(self.max, self.min)
    }

    pub fn verses(&self, upper: i32, lower: i32) -> String {
        let range = (lower..=upper).rev();
        range
            .map(|num| self.verse(num))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn verse(&self, number: i32) -> String {
        self.verse_template.lyrics(number)
    }
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => "".to_string(),
        Some(c) => format!("{}{}", c.to_ascii_uppercase(), chars.as_str()),
    }
}

trait BottleNumberTrait {
    fn quantity(&self) -> String;
    fn action(&self) -> String;
    fn container(&self) -> String;
    fn successor(&self) -> Box<dyn BottleNumberTrait>;
    fn as_any(&self) -> &dyn Any;
}

impl fmt::Display for Box<dyn BottleNumberTrait> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{quantity} {container}",
            quantity = self.quantity(),
            container = self.container()
        )
    }
}

struct BottleNumber {
    number: i32,
}

impl BottleNumber {
    fn new(number: i32) -> Self {
        BottleNumber { number }
    }

    fn of(number: i32) -> Box<dyn BottleNumberTrait> {
        match number {
            0 => Box::new(BottleNumberZero::new(number)),
            1 => Box::new(BottleNumberOne::new(number)),
            6 => Box::new(BottleNumberSix::new(number)),
            _ => Box::new(BottleNumber::new(number)),
        }
    }

    fn quantity(&self) -> String {
        self.number.to_string()
    }

    fn container(&self) -> String {
        "bottles".to_string()
    }

    fn pronoun(&self) -> String {
        "one".to_string()
    }

    fn action(&self) -> String {
        format!(
            "Take {pronoun} down and pass it around",
            pronoun = self.pronoun()
        )
    }

    fn successor(&self) -> Box<dyn BottleNumberTrait> {
        BottleNumber::of(self.number - 1)
    }
}

impl BottleNumberTrait for BottleNumber {
    fn quantity(&self) -> String {
        self.quantity()
    }

    fn action(&self) -> String {
        self.action()
    }

    fn container(&self) -> String {
        self.container()
    }

    fn successor(&self) -> Box<dyn BottleNumberTrait> {
        self.successor()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct BottleNumberZero {
    bottle_number: BottleNumber,
}

impl BottleNumberZero {
    fn new(number: i32) -> Self {
        BottleNumberZero {
            // Use composition instead of inheritance to re-use BottleNumber's behaviours
            bottle_number: BottleNumber::new(number),
        }
    }

    fn quantity(&self) -> String {
        "no more".to_string()
    }

    fn action(&self) -> String {
        "Go to the store and buy some more".to_string()
    }

    fn container(&self) -> String {
        self.bottle_number.container()
    }

    fn successor(&self) -> Box<dyn BottleNumberTrait> {
        BottleNumber::of(99)
    }
}

impl BottleNumberTrait for BottleNumberZero {
    fn quantity(&self) -> String {
        self.quantity()
    }

    fn action(&self) -> String {
        self.action()
    }

    fn container(&self) -> String {
        self.container()
    }

    fn successor(&self) -> Box<dyn BottleNumberTrait> {
        self.successor()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct BottleNumberOne {
    bottle_number: BottleNumber,
}

impl BottleNumberOne {
    fn new(number: i32) -> Self {
        BottleNumberOne {
            bottle_number: BottleNumber::new(number),
        }
    }

    fn quantity(&self) -> String {
        self.bottle_number.quantity()
    }

    fn action(&self) -> String {
        format!(
            "Take {pronoun} down and pass it around",
            pronoun = self.pronoun()
        )
    }

    fn pronoun(&self) -> String {
        "it".to_string()
    }

    fn container(&self) -> String {
        "bottle".to_string()
    }

    fn successor(&self) -> Box<dyn BottleNumberTrait> {
        self.bottle_number.successor()
    }
}

impl BottleNumberTrait for BottleNumberOne {
    fn quantity(&self) -> String {
        self.quantity()
    }

    fn action(&self) -> String {
        self.action()
    }

    fn container(&self) -> String {
        self.container()
    }

    fn successor(&self) -> Box<dyn BottleNumberTrait> {
        self.successor()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct BottleNumberSix {
    bottle_number: BottleNumber,
}

impl BottleNumberSix {
    fn new(number: i32) -> Self {
        BottleNumberSix {
            bottle_number: BottleNumber::new(number),
        }
    }

    fn quantity(&self) -> String {
        '1'.to_string()
    }

    fn action(&self) -> String {
        self.bottle_number.action()
    }

    fn container(&self) -> String {
        "six-pack".to_string()
    }

    fn successor(&self) -> Box<dyn BottleNumberTrait> {
        self.bottle_number.successor()
    }
}

impl BottleNumberTrait for BottleNumberSix {
    fn quantity(&self) -> String {
        self.quantity()
    }

    fn action(&self) -> String {
        self.action()
    }

    fn container(&self) -> String {
        self.container()
    }

    fn successor(&self) -> Box<dyn BottleNumberTrait> {
        self.successor()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct BottleVerseFactory;

impl VerseTrait for BottleVerseFactory {
    fn lyrics(&self, number: i32) -> String {
        BottleVerse::new(BottleNumber::of(number)).lyrics()
    }
}

struct BottleVerse {
    bottle_number: Box<dyn BottleNumberTrait>,
}

impl BottleVerse {
    fn new(number: Box<dyn BottleNumberTrait>) -> Self {
        BottleVerse {
            bottle_number: number,
        }
    }

    fn lyrics(&self) -> String {
        format!(
            "{capitalized_bottle_number} of beer on the wall, {bottle_number} of beer.
{action}, {next_bottle_number} of beer on the wall.\n",
            capitalized_bottle_number = capitalize(&self.bottle_number.to_string()),
            action = self.bottle_number.action(),
            bottle_number = self.bottle_number,
            next_bottle_number = self.bottle_number.successor(),
        )
    }
}

#[cfg(test)]
mod countdown_song {
    use super::*;

    struct VerseFake;

    impl VerseTrait for VerseFake {
        fn lyrics(&self, number: i32) -> String {
            format!("This is verse {number}.\n", number = number)
        }
    }

    #[test]
    fn verse() {
        let expected = "This is verse 500.\n";

        assert_eq!(
            CountdownSong::new(Box::new(VerseFake), 99, 0).verse(500),
            expected
        );
    }

    #[test]
    fn verses() {
        let expected = "This is verse 99.

This is verse 98.

This is verse 97.
";

        assert_eq!(
            CountdownSong::new(Box::new(VerseFake), 99, 0).verses(99, 97),
            expected
        );
    }

    #[test]
    fn song() {
        let expected = "This is verse 47.

This is verse 46.

This is verse 45.

This is verse 44.

This is verse 43.
";

        assert_eq!(
            CountdownSong::new(Box::new(VerseFake), 47, 43).song(),
            expected
        );
    }
}

pub fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

#[cfg(test)]
mod bottle_number {
    use super::*;

    mod of {
        use super::*;
        use std::any::TypeId;

        #[test]
        fn returns_correct_struct_type() {
            {
                let res = BottleNumber::of(0);
                let actual_id = (&*res.as_any()).type_id();

                assert_eq!(actual_id, TypeId::of::<BottleNumberZero>());
            }
            {
                let res = BottleNumber::of(1);
                let actual_id = (&*res.as_any()).type_id();

                assert_eq!(actual_id, TypeId::of::<BottleNumberOne>());
            }
            {
                let res = BottleNumber::of(6);
                let actual_id = (&*res.as_any()).type_id();

                assert_eq!(actual_id, TypeId::of::<BottleNumberSix>());
            }
            {
                let res = BottleNumber::of(99);
                let actual_id = (&*res.as_any()).type_id();

                assert_eq!(actual_id, TypeId::of::<BottleNumber>());
            }
        }
    }
}

#[cfg(test)]
mod bottle_verse {
    use super::*;

    mod lyrics {
        use super::*;

        #[test]
        fn verse_general_rule_upper_bound() {
            let expected = "99 bottles of beer on the wall, 99 bottles of beer.
Take one down and pass it around, 98 bottles of beer on the wall.\n";
            assert_eq!(BottleVerseFactory.lyrics(99), expected);
        }

        #[test]
        fn verse_general_rule_lower_bound() {
            let expected = "3 bottles of beer on the wall, 3 bottles of beer.
Take one down and pass it around, 2 bottles of beer on the wall.\n";
            assert_eq!(BottleVerseFactory.lyrics(3), expected);
        }

        #[test]
        fn verse_7() {
            let expected = "7 bottles of beer on the wall, 7 bottles of beer.
Take one down and pass it around, 1 six-pack of beer on the wall.\n";
            assert_eq!(BottleVerseFactory.lyrics(7), expected);
        }

        #[test]
        fn verse_6() {
            let expected = "1 six-pack of beer on the wall, 1 six-pack of beer.
Take one down and pass it around, 5 bottles of beer on the wall.\n";
            assert_eq!(BottleVerseFactory.lyrics(6), expected);
        }

        #[test]
        fn verse_2() {
            let expected = "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.\n";
            assert_eq!(BottleVerseFactory.lyrics(2), expected);
        }

        #[test]
        fn verse_1() {
            let expected = "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n";
            assert_eq!(BottleVerseFactory.lyrics(1), expected);
        }

        #[test]
        fn verse_0() {
            let expected = "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n";
            assert_eq!(BottleVerseFactory.lyrics(0), expected);
        }
    }
}
