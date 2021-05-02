pub fn verse(number: i32) -> String {
    match number {
        1 => {
            return format!(
                "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n",
            )
        }
        2 => {
            return format!(
                "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.\n",
            )
        }
        _ => {
            return format!(
                "{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.\n",
                number,
                number,
                number - 1
            )
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verse_99_bottles() {
        let expected = "99 bottles of beer on the wall, 99 bottles of beer.
Take one down and pass it around, 98 bottles of beer on the wall.\n";
        assert_eq!(verse(99), expected);
    }

    #[test]
    fn test_verse_3_bottles() {
        let expected = "3 bottles of beer on the wall, 3 bottles of beer.
Take one down and pass it around, 2 bottles of beer on the wall.\n";
        assert_eq!(verse(3), expected);
    }

    #[test]
    fn test_verse_2_bottles() {
        let expected = "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.\n";
        assert_eq!(verse(2), expected);
    }

    #[test]
    fn test_verse_1_bottle() {
        let expected = "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n";
        assert_eq!(verse(1), expected);
    }
}
