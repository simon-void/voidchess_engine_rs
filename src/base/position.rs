use std::ops::Range;
use std::fmt;
use std::str;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub column: u8,
    pub row: u8,
}

impl str::FromStr for Position {
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let mut char_iter = code.chars();
        let column = (char_iter.next().unwrap() as u8) - 97;
        let row = (char_iter.next().unwrap() as u8) - 49;
        let allowed_range: Range<u8> = 0..8;

        if !(allowed_range.contains(&column) & &allowed_range.contains(&row)) {
            panic!("illegal value for Position: {}", code);
        }

        Ok(Position {
            column,
            row,
        })
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", (self.column + 97) as char, (self.row+49) as char)
    }
}