use crate::{Position, Direction};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Snake {
    pub head_pos: Position,
    pub tail_pos: VecDeque<Position>,
    pub direction: Direction,
    pub just_ate_apple: bool,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            head_pos: Position::new(4, 7),
            // [last, ..., first]
            tail_pos: VecDeque::from([Position::new(1, 7), Position::new(2, 7), Position::new(3, 7)]),
            direction: Direction::Right,
            just_ate_apple: false
        }
    }
}
