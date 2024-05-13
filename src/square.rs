use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Square {
    Empty,
    Snake,
    Apple,
    Border
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let square_char = match self {
            Square::Empty => ' ',
            Square::Snake => 'S',
            Square::Apple => 'O',
            Square::Border => '□' // ■
        };
        write!(f, "{}", square_char)
    }
}
