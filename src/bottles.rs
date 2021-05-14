use core::fmt;

pub trait VerseTrait {
    fn lyrics(&self, number: i32) -> String;
}

struct BottleVerseFactory;

impl VerseTrait for BottleVerseFactory {
    fn lyrics(&self, number: i32) -> String {
        BottleVerse::new(BottleNumber::of(number)).lyrics()
    }
}

type VerseTemplate = Box<dyn VerseTrait>;

pub struct Bottle {
    verse_template: VerseTemplate,
}

impl Default for Bottle {
    fn default() -> Self {
        Self::new(Box::new(BottleVerseFactory))
    }
}

impl Bottle {
    pub fn new(verse_template: VerseTemplate) -> Self {
        Bottle { verse_template }
    }

    pub fn song(&self) -> String {
        self.verses(99, 0)
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
}

struct BottleVerse {
    number: Box<dyn BottleNumberTrait>,
}

impl BottleVerse {
    fn new(number: Box<dyn BottleNumberTrait>) -> Self {
        BottleVerse { number }
    }

    fn lyrics(&self) -> String {
        let bottle_number = &self.number;

        format!(
            "{capitalized_bottle_number} of beer on the wall, {bottle_number} of beer.
{action}, {next_bottle_number} of beer on the wall.\n",
            capitalized_bottle_number = capitalize(&bottle_number.to_string()),
            action = bottle_number.action(),
            bottle_number = bottle_number,
            next_bottle_number = bottle_number.successor(),
        )
    }
}

#[cfg(test)]
mod verse_tests {
    use super::*;

    #[test]
    fn test_verse_99_bottles() {
        let expected = "99 bottles of beer on the wall, 99 bottles of beer.
Take one down and pass it around, 98 bottles of beer on the wall.\n";
        assert_eq!(Bottle::default().verse(99), expected);
    }

    #[test]
    fn test_verse_3_bottles() {
        let expected = "3 bottles of beer on the wall, 3 bottles of beer.
Take one down and pass it around, 2 bottles of beer on the wall.\n";
        assert_eq!(Bottle::default().verse(3), expected);
    }

    #[test]
    fn test_verse_2_bottles() {
        let expected = "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.\n";
        assert_eq!(Bottle::default().verse(2), expected);
    }

    #[test]
    fn test_verse_1_bottle() {
        let expected = "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n";
        assert_eq!(Bottle::default().verse(1), expected);
    }

    #[test]
    fn test_verse_0_bottles() {
        let expected = "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n";
        assert_eq!(Bottle::default().verse(0), expected);
    }
}

#[cfg(test)]
mod verses_tests {
    use super::*;

    #[test]
    fn test_verses_99_to_98() {
        let expected = "99 bottles of beer on the wall, 99 bottles of beer.
Take one down and pass it around, 98 bottles of beer on the wall.

98 bottles of beer on the wall, 98 bottles of beer.
Take one down and pass it around, 97 bottles of beer on the wall.
";
        assert_eq!(Bottle::default().verses(99, 98), expected);
    }

    #[test]
    fn test_verses_2_to_0() {
        let expected = "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.

1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.

No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
";
        assert_eq!(Bottle::default().verses(2, 0), expected);
    }
}

#[cfg(test)]
mod song_tests {
    use super::*;

    #[test]
    fn test_entire_song() {
        let expected = "99 bottles of beer on the wall, 99 bottles of beer.
Take one down and pass it around, 98 bottles of beer on the wall.

98 bottles of beer on the wall, 98 bottles of beer.
Take one down and pass it around, 97 bottles of beer on the wall.

97 bottles of beer on the wall, 97 bottles of beer.
Take one down and pass it around, 96 bottles of beer on the wall.

96 bottles of beer on the wall, 96 bottles of beer.
Take one down and pass it around, 95 bottles of beer on the wall.

95 bottles of beer on the wall, 95 bottles of beer.
Take one down and pass it around, 94 bottles of beer on the wall.

94 bottles of beer on the wall, 94 bottles of beer.
Take one down and pass it around, 93 bottles of beer on the wall.

93 bottles of beer on the wall, 93 bottles of beer.
Take one down and pass it around, 92 bottles of beer on the wall.

92 bottles of beer on the wall, 92 bottles of beer.
Take one down and pass it around, 91 bottles of beer on the wall.

91 bottles of beer on the wall, 91 bottles of beer.
Take one down and pass it around, 90 bottles of beer on the wall.

90 bottles of beer on the wall, 90 bottles of beer.
Take one down and pass it around, 89 bottles of beer on the wall.

89 bottles of beer on the wall, 89 bottles of beer.
Take one down and pass it around, 88 bottles of beer on the wall.

88 bottles of beer on the wall, 88 bottles of beer.
Take one down and pass it around, 87 bottles of beer on the wall.

87 bottles of beer on the wall, 87 bottles of beer.
Take one down and pass it around, 86 bottles of beer on the wall.

86 bottles of beer on the wall, 86 bottles of beer.
Take one down and pass it around, 85 bottles of beer on the wall.

85 bottles of beer on the wall, 85 bottles of beer.
Take one down and pass it around, 84 bottles of beer on the wall.

84 bottles of beer on the wall, 84 bottles of beer.
Take one down and pass it around, 83 bottles of beer on the wall.

83 bottles of beer on the wall, 83 bottles of beer.
Take one down and pass it around, 82 bottles of beer on the wall.

82 bottles of beer on the wall, 82 bottles of beer.
Take one down and pass it around, 81 bottles of beer on the wall.

81 bottles of beer on the wall, 81 bottles of beer.
Take one down and pass it around, 80 bottles of beer on the wall.

80 bottles of beer on the wall, 80 bottles of beer.
Take one down and pass it around, 79 bottles of beer on the wall.

79 bottles of beer on the wall, 79 bottles of beer.
Take one down and pass it around, 78 bottles of beer on the wall.

78 bottles of beer on the wall, 78 bottles of beer.
Take one down and pass it around, 77 bottles of beer on the wall.

77 bottles of beer on the wall, 77 bottles of beer.
Take one down and pass it around, 76 bottles of beer on the wall.

76 bottles of beer on the wall, 76 bottles of beer.
Take one down and pass it around, 75 bottles of beer on the wall.

75 bottles of beer on the wall, 75 bottles of beer.
Take one down and pass it around, 74 bottles of beer on the wall.

74 bottles of beer on the wall, 74 bottles of beer.
Take one down and pass it around, 73 bottles of beer on the wall.

73 bottles of beer on the wall, 73 bottles of beer.
Take one down and pass it around, 72 bottles of beer on the wall.

72 bottles of beer on the wall, 72 bottles of beer.
Take one down and pass it around, 71 bottles of beer on the wall.

71 bottles of beer on the wall, 71 bottles of beer.
Take one down and pass it around, 70 bottles of beer on the wall.

70 bottles of beer on the wall, 70 bottles of beer.
Take one down and pass it around, 69 bottles of beer on the wall.

69 bottles of beer on the wall, 69 bottles of beer.
Take one down and pass it around, 68 bottles of beer on the wall.

68 bottles of beer on the wall, 68 bottles of beer.
Take one down and pass it around, 67 bottles of beer on the wall.

67 bottles of beer on the wall, 67 bottles of beer.
Take one down and pass it around, 66 bottles of beer on the wall.

66 bottles of beer on the wall, 66 bottles of beer.
Take one down and pass it around, 65 bottles of beer on the wall.

65 bottles of beer on the wall, 65 bottles of beer.
Take one down and pass it around, 64 bottles of beer on the wall.

64 bottles of beer on the wall, 64 bottles of beer.
Take one down and pass it around, 63 bottles of beer on the wall.

63 bottles of beer on the wall, 63 bottles of beer.
Take one down and pass it around, 62 bottles of beer on the wall.

62 bottles of beer on the wall, 62 bottles of beer.
Take one down and pass it around, 61 bottles of beer on the wall.

61 bottles of beer on the wall, 61 bottles of beer.
Take one down and pass it around, 60 bottles of beer on the wall.

60 bottles of beer on the wall, 60 bottles of beer.
Take one down and pass it around, 59 bottles of beer on the wall.

59 bottles of beer on the wall, 59 bottles of beer.
Take one down and pass it around, 58 bottles of beer on the wall.

58 bottles of beer on the wall, 58 bottles of beer.
Take one down and pass it around, 57 bottles of beer on the wall.

57 bottles of beer on the wall, 57 bottles of beer.
Take one down and pass it around, 56 bottles of beer on the wall.

56 bottles of beer on the wall, 56 bottles of beer.
Take one down and pass it around, 55 bottles of beer on the wall.

55 bottles of beer on the wall, 55 bottles of beer.
Take one down and pass it around, 54 bottles of beer on the wall.

54 bottles of beer on the wall, 54 bottles of beer.
Take one down and pass it around, 53 bottles of beer on the wall.

53 bottles of beer on the wall, 53 bottles of beer.
Take one down and pass it around, 52 bottles of beer on the wall.

52 bottles of beer on the wall, 52 bottles of beer.
Take one down and pass it around, 51 bottles of beer on the wall.

51 bottles of beer on the wall, 51 bottles of beer.
Take one down and pass it around, 50 bottles of beer on the wall.

50 bottles of beer on the wall, 50 bottles of beer.
Take one down and pass it around, 49 bottles of beer on the wall.

49 bottles of beer on the wall, 49 bottles of beer.
Take one down and pass it around, 48 bottles of beer on the wall.

48 bottles of beer on the wall, 48 bottles of beer.
Take one down and pass it around, 47 bottles of beer on the wall.

47 bottles of beer on the wall, 47 bottles of beer.
Take one down and pass it around, 46 bottles of beer on the wall.

46 bottles of beer on the wall, 46 bottles of beer.
Take one down and pass it around, 45 bottles of beer on the wall.

45 bottles of beer on the wall, 45 bottles of beer.
Take one down and pass it around, 44 bottles of beer on the wall.

44 bottles of beer on the wall, 44 bottles of beer.
Take one down and pass it around, 43 bottles of beer on the wall.

43 bottles of beer on the wall, 43 bottles of beer.
Take one down and pass it around, 42 bottles of beer on the wall.

42 bottles of beer on the wall, 42 bottles of beer.
Take one down and pass it around, 41 bottles of beer on the wall.

41 bottles of beer on the wall, 41 bottles of beer.
Take one down and pass it around, 40 bottles of beer on the wall.

40 bottles of beer on the wall, 40 bottles of beer.
Take one down and pass it around, 39 bottles of beer on the wall.

39 bottles of beer on the wall, 39 bottles of beer.
Take one down and pass it around, 38 bottles of beer on the wall.

38 bottles of beer on the wall, 38 bottles of beer.
Take one down and pass it around, 37 bottles of beer on the wall.

37 bottles of beer on the wall, 37 bottles of beer.
Take one down and pass it around, 36 bottles of beer on the wall.

36 bottles of beer on the wall, 36 bottles of beer.
Take one down and pass it around, 35 bottles of beer on the wall.

35 bottles of beer on the wall, 35 bottles of beer.
Take one down and pass it around, 34 bottles of beer on the wall.

34 bottles of beer on the wall, 34 bottles of beer.
Take one down and pass it around, 33 bottles of beer on the wall.

33 bottles of beer on the wall, 33 bottles of beer.
Take one down and pass it around, 32 bottles of beer on the wall.

32 bottles of beer on the wall, 32 bottles of beer.
Take one down and pass it around, 31 bottles of beer on the wall.

31 bottles of beer on the wall, 31 bottles of beer.
Take one down and pass it around, 30 bottles of beer on the wall.

30 bottles of beer on the wall, 30 bottles of beer.
Take one down and pass it around, 29 bottles of beer on the wall.

29 bottles of beer on the wall, 29 bottles of beer.
Take one down and pass it around, 28 bottles of beer on the wall.

28 bottles of beer on the wall, 28 bottles of beer.
Take one down and pass it around, 27 bottles of beer on the wall.

27 bottles of beer on the wall, 27 bottles of beer.
Take one down and pass it around, 26 bottles of beer on the wall.

26 bottles of beer on the wall, 26 bottles of beer.
Take one down and pass it around, 25 bottles of beer on the wall.

25 bottles of beer on the wall, 25 bottles of beer.
Take one down and pass it around, 24 bottles of beer on the wall.

24 bottles of beer on the wall, 24 bottles of beer.
Take one down and pass it around, 23 bottles of beer on the wall.

23 bottles of beer on the wall, 23 bottles of beer.
Take one down and pass it around, 22 bottles of beer on the wall.

22 bottles of beer on the wall, 22 bottles of beer.
Take one down and pass it around, 21 bottles of beer on the wall.

21 bottles of beer on the wall, 21 bottles of beer.
Take one down and pass it around, 20 bottles of beer on the wall.

20 bottles of beer on the wall, 20 bottles of beer.
Take one down and pass it around, 19 bottles of beer on the wall.

19 bottles of beer on the wall, 19 bottles of beer.
Take one down and pass it around, 18 bottles of beer on the wall.

18 bottles of beer on the wall, 18 bottles of beer.
Take one down and pass it around, 17 bottles of beer on the wall.

17 bottles of beer on the wall, 17 bottles of beer.
Take one down and pass it around, 16 bottles of beer on the wall.

16 bottles of beer on the wall, 16 bottles of beer.
Take one down and pass it around, 15 bottles of beer on the wall.

15 bottles of beer on the wall, 15 bottles of beer.
Take one down and pass it around, 14 bottles of beer on the wall.

14 bottles of beer on the wall, 14 bottles of beer.
Take one down and pass it around, 13 bottles of beer on the wall.

13 bottles of beer on the wall, 13 bottles of beer.
Take one down and pass it around, 12 bottles of beer on the wall.

12 bottles of beer on the wall, 12 bottles of beer.
Take one down and pass it around, 11 bottles of beer on the wall.

11 bottles of beer on the wall, 11 bottles of beer.
Take one down and pass it around, 10 bottles of beer on the wall.

10 bottles of beer on the wall, 10 bottles of beer.
Take one down and pass it around, 9 bottles of beer on the wall.

9 bottles of beer on the wall, 9 bottles of beer.
Take one down and pass it around, 8 bottles of beer on the wall.

8 bottles of beer on the wall, 8 bottles of beer.
Take one down and pass it around, 7 bottles of beer on the wall.

7 bottles of beer on the wall, 7 bottles of beer.
Take one down and pass it around, 1 six-pack of beer on the wall.

1 six-pack of beer on the wall, 1 six-pack of beer.
Take one down and pass it around, 5 bottles of beer on the wall.

5 bottles of beer on the wall, 5 bottles of beer.
Take one down and pass it around, 4 bottles of beer on the wall.

4 bottles of beer on the wall, 4 bottles of beer.
Take one down and pass it around, 3 bottles of beer on the wall.

3 bottles of beer on the wall, 3 bottles of beer.
Take one down and pass it around, 2 bottles of beer on the wall.

2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.

1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.

No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
";

        assert_eq!(Bottle::default().song(), expected);
    }
}
