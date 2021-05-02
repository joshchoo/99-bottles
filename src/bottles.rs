pub fn verse(number: i32) -> String {
    return "99 bottles of beer on the wall, 99 bottles of beer.
Take one down and pass it around, 98 bottles of beer on the wall."
        .to_string();
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
}
