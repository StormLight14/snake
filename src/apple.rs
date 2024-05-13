use crate::Position;

#[derive(Clone, PartialEq)]
pub struct Apple {
    pub pos: Position
}

impl Apple {
    pub fn new(pos: Position) -> Self {
        Apple {
            pos
        }
    }
}
