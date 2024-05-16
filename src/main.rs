use std::io;
use std::io::stdout;
use std::thread;
use std::time::Duration;

use crossterm::event::KeyEventKind;
use crossterm::{
    execute,
    event::{read, poll, Event, KeyCode},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    cursor::Hide
};

use game::Game;

pub mod game;
pub mod square;
pub mod apple;
pub mod snake;

#[derive(Copy, Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut game = Game::new();
    execute!(io::stdout(), Hide, terminal::Clear(terminal::ClearType::All))?;

    loop {
        let stdout = stdout();

        game.print_grid(stdout)?;
        thread::sleep(Duration::from_secs_f32(0.15));
        read_input(&mut game).unwrap();
        if game.move_snake() == false {
            exit_game(Some("\nYou Lost!"));
        };
        game.eat_apple();
        game.update_grid();
        if game.head_hit_tail() {
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

fn read_input(game: &mut Game) -> io::Result<()> {
    if let Ok(true) = poll(Duration::from_millis(10)) {

        let event = read()?;
        match event {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                match event.code {
                    KeyCode::Esc | KeyCode::Char('q')  => {
                        exit_game(None);
                    },
                    KeyCode::Left | KeyCode::Char('a') => if game.snake.direction != Direction::Right {game.snake.direction = Direction::Left},
                    KeyCode::Right | KeyCode::Char('d') => if game.snake.direction != Direction::Left {game.snake.direction = Direction::Right},
                    KeyCode::Up | KeyCode::Char('w') => if game.snake.direction != Direction::Down {game.snake.direction = Direction::Up},
                    KeyCode::Down | KeyCode::Char('s') => if game.snake.direction != Direction::Up {game.snake.direction = Direction::Down},
                    _ => {
                    }
                }
            },
            _ => {
            }
        }
    }

    Ok(())
}
