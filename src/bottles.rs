pub fn verse(number: i32) -> String {
    if number == 2 {
        return format!(
            "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.\n",
        );
    }

    format!(
        "{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.\n",
        number,
        number,
        number - 1
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verse_99_bottles() {
        let expected: &str = "99 bottles of beer on the wall, 99 bottles of beer.
Take one down and pass it around, 98 bottles of beer on the wall.\n";
        assert_eq!(verse(99), expected);
    }

    #[test]
    fn test_verse_3_bottles() {
        let expected: &str = "3 bottles of beer on the wall, 3 bottles of beer.
Take one down and pass it around, 2 bottles of beer on the wall.\n";
        assert_eq!(verse(3), expected);
    }

    #[test]
    fn test_verse_2_bottles() {
        let expected: &str = "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.\n";
        assert_eq!(verse(2), expected);
    }
}
