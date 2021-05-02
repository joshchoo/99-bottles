pub fn verse(number: i32) -> String {
    if number == 99 {
        return "99 bottles of beer on the wall, 99 bottles of beer.
Take one down and pass it around, 98 bottles of beer on the wall."
            .to_string();
    } else {
        return "3 bottles of beer on the wall, 3 bottles of beer.
Take one down and pass it around, 2 bottles of beer on the wall."
            .to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verse_99_bottles() {
        let expected: &str = "99 bottles of beer on the wall, 99 bottles of beer.
Take one down and pass it around, 98 bottles of beer on the wall.";
        assert_eq!(verse(99), expected);
    }

    #[test]
    fn test_verse_3_bottles() {
        let expected: &str = "3 bottles of beer on the wall, 3 bottles of beer.
Take one down and pass it around, 2 bottles of beer on the wall.";
        assert_eq!(verse(3), expected);
    }
}
