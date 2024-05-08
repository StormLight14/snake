use std::io;
use std::io::stdout;
use std::thread;
use std::time::Duration;

use crossterm::event::KeyEventKind;
use crossterm::{
    event::{read, poll, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use level::Level;

pub mod level;
pub mod square;
pub mod snake;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    x: u8,
    y: u8
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Position {
            x,
            y
        }
    }
}

#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut level = Level::new();

    loop {
        let stdout = stdout();
        level.print_grid(stdout)?;
        thread::sleep(Duration::from_secs_f32(0.1));
        read_input(&mut level).unwrap();
        level.move_snake();
        if level.head_hit_tail() {
            exit_game(Some("\nYou lost!"));
        }
    }
}

fn exit_game(message: Option<&str>) {
    disable_raw_mode().unwrap();
    if let Some(message) = message {
        println!("{}", message);
    }
    std::process::exit(0);
}

fn read_input(level: &mut Level) -> io::Result<()> {
    if let Ok(has_event) = poll(Duration::from_secs_f32(0.05)) {
        if has_event == false {
            return Ok(())
        }
        let event = read()?;
        match event {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                match event.code {
                    KeyCode::Esc | KeyCode::Char('q')  => {
                        exit_game(None);
                    },
                    KeyCode::Left | KeyCode::Char('a') => level.snake.direction = Direction::Left,
                    KeyCode::Right | KeyCode::Char('d') => level.snake.direction = Direction::Right,
                    KeyCode::Up | KeyCode::Char('w') => level.snake.direction = Direction::Up,
                    KeyCode::Down | KeyCode::Char('s') => level.snake.direction = Direction::Down,
                    _ => {}
                }
            },
                _ => {}
        }
    }
    Ok(())
}
