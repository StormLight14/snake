use crate::{Position, Direction};

#[derive(Clone)]
pub struct Snake {
    pub head_pos: Position,
    pub tail_pos: Vec<Position>,
    pub direction: Direction
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            head_pos: Position::new(3, 7),
            tail_pos: vec![Position::new(2, 7), Position::new(1, 7)],
            direction: Direction::Right
        }
    }
}
