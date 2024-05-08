use std::fmt;

#[derive(Clone)]
pub enum Square {
    Empty,
    Snake,
    Apple
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let square_char = match self {
            Square::Empty => '#',
            Square::Snake => 'S',
            Square::Apple => 'A'
        };
        write!(f, "{}", square_char)
    }
}
