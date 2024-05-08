use crate::square::Square;
use crate::snake::Snake;
use crate::{Direction, Position};
use std::io::{self, Write};
use crossterm::{ExecutableCommand, QueueableCommand, terminal, cursor, queue, style};

#[derive(Clone)]
pub struct Level {
    pub grid: Vec<Vec<Square>>,
    pub snake: Snake,
}

impl Level {
    pub fn new() -> Self {
        let snake = Snake::new();
        let mut grid: Vec<Vec<Square>> = Vec::new();
        for _y in 0..12 {
            let mut row: Vec<Square> = Vec::new();
            for _x in 0..12 {
                row.push(Square::Empty);
            }
            grid.push(row);
        }
        grid[snake.head_pos.y as usize][snake.head_pos.x as usize] = Square::Snake;
        for pos in snake.tail_pos.iter() {
            grid[pos.y as usize][pos.x as usize] = Square::Snake;
        }
        Level {
            grid,
            snake: Snake::new()
        }
    }
    pub fn print_grid(&self, mut stdout: std::io::Stdout) -> io::Result<()> {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                stdout.queue(cursor::MoveTo(x as u16, y as u16))?.queue(style::Print(format!("{}", self.grid[y][x])))?;
            }
        }
        stdout.flush()?;
        Ok(())
    }
    pub fn move_snake(&mut self) {
        let translate_vector = match self.snake.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        };

        let (to_x, to_y) = (self.snake.head_pos.x as i8 + translate_vector.0, self.snake.head_pos.y as i8 + translate_vector.1);
        if to_x >= 0 && to_x < self.grid.len() as i8 && to_y >= 0 && to_y < self.grid[0].len() as i8 {
            self.snake.head_pos = Position::new(to_x as u8, to_y as u8);
        }

        self.grid[self.snake.head_pos.y as usize][self.snake.head_pos.x as usize] = Square::Snake;
        for pos in self.snake.tail_pos.iter() {
            self.grid[pos.y as usize][pos.x as usize] = Square::Snake;
        }
    }
    pub fn head_hit_tail(&self) -> bool {
        for pos in self.snake.tail_pos.iter() {
            if *pos == self.snake.head_pos {
                return true;
            }
        }
        return false;
    }
}
