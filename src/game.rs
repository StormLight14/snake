use crate::square::Square;
use crate::snake::Snake;
use crate::apple::Apple;
use crate::{Direction, Position};
use rand::{Rng, thread_rng};
use std::io::{self, Write};
use crossterm::{ExecutableCommand, QueueableCommand, terminal, cursor, style::{self, Stylize}};

const LEVEL_SIZE: u8 = 18;

#[derive(Clone)]
pub struct Game {
    pub level_grid: Vec<Vec<Square>>,
    pub snake: Snake,
    pub apples: Vec<Apple>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            level_grid: empty_grid(LEVEL_SIZE),
            snake: Snake::new(),
            apples: vec![Apple::new(Position::new(10, 10))]
        }
    }
    pub fn print_grid(&self, mut stdout: std::io::Stdout) -> io::Result<()> {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        for y in 0..self.level_grid.len() {
            for x in 0..self.level_grid[y].len() {
                let square = &self.level_grid[y][x];
                let styled_content = match square {
                    Square::Apple => format!("{}", square).red().bold(),
                    Square::Snake => format!("{}", square).green().bold().bold(),
                    Square::Empty => format!("{}", square).blue(),
                };
                stdout.queue(cursor::MoveTo((x as u16) * 2, y as u16))?.queue(style::PrintStyledContent(styled_content))?;
            }
        }
        stdout.flush()?;
        Ok(())
    }
    pub fn update_grid(&mut self) {
        self.level_grid = empty_grid(LEVEL_SIZE);

        self.level_grid[self.snake.head_pos.y as usize][self.snake.head_pos.x as usize] = Square::Snake;
        for pos in self.snake.tail_pos.iter() {
            self.level_grid[pos.y as usize][pos.x as usize] = Square::Snake;
        }
        for apple in self.apples.iter() {
            self.level_grid[apple.pos.y as usize][apple.pos.x as usize] = Square::Apple;
        }
    }
    pub fn move_snake(&mut self) -> bool {
        let translate_vector = match self.snake.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        };
        
        if self.snake.just_ate_apple == false {
            self.snake.tail_pos.pop_front();
        } 

        let (to_x, to_y) = (self.snake.head_pos.x as i8 + translate_vector.0, self.snake.head_pos.y as i8 + translate_vector.1);
        if to_x >= 0 && to_x < self.level_grid.len() as i8 && to_y >= 0 && to_y < self.level_grid[0].len() as i8 {
            self.snake.tail_pos.push_back(self.snake.head_pos);
            self.snake.head_pos = Position::new(to_x as u8, to_y as u8);
        } else {
            return false;
        }

        true
    }
    pub fn head_hit_tail(&self) -> bool {
        for pos in self.snake.tail_pos.iter() {
            if *pos == self.snake.head_pos {
                return true;
            }
        }
        return false;
    }
    pub fn eat_apple(&mut self) {
        self.snake.just_ate_apple = false;

        let mut remove_apple = None;
        for (i, apple) in self.apples.iter_mut().enumerate() {
            if self.snake.head_pos == apple.pos {
                remove_apple = Some(i);
            }
        }

        if let Some(remove_apple) = remove_apple {
            self.apples.remove(remove_apple);
            self.snake.just_ate_apple = true;
        
            self.create_apple();
        }
        
    }
    pub fn create_apple(&mut self) {
        let mut rng = thread_rng();
        let mut rand_x: u8;
        let mut rand_y: u8;

        loop {
            rand_x = rng.gen_range(0..self.level_grid.len()) as u8;
            rand_y = rng.gen_range(0..self.level_grid.len()) as u8;

            let pos = Position::new(rand_x, rand_y);
            if pos != self.snake.head_pos && self.snake.tail_pos.contains(&pos) == false {
                self.apples.push(Apple::new(pos));
                break;
            }
        }
    }
}

pub fn empty_grid(size: u8) -> Vec<Vec<Square>> {
    let mut grid: Vec<Vec<Square>> = Vec::new();
    for _y in 0..size {
        let mut row: Vec<Square> = Vec::new();
        for _x in 0..size {
            row.push(Square::Empty);
        }
        grid.push(row);
    }

    grid
}
