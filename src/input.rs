use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use std::time::Duration;

pub enum InputResult {
    MoveLeft,
    MoveRight,
    MoveDown,
    RotateClockwise,
    RotateCounterClockwise,
    ExitGame,
}

pub trait InputSource {
    fn read_input(&self) -> Option<InputResult>;
}

pub struct CrosstermInput {}

impl InputSource for CrosstermInput {
    fn read_input(&self) -> Option<InputResult> {
        match poll(Duration::from_secs(0)) {
            Ok(has_input) => {
                match has_input {
                    true => convert_input(),
                    false => None
                }
            }
            Err(error) => {
                println!("Error reading input poll: {}", error);
                None
            }
        }
    }
}

fn convert_input() -> Option<InputResult> {
    match read() {
        Ok(read_key) => {
            match read_key {
                Event::Key(KeyEvent {
                               code: KeyCode::Left,
                               ..
                           }) => Some(InputResult::MoveLeft),
                Event::Key(KeyEvent {
                               code: KeyCode::Right,
                               ..
                           }) => Some(InputResult::MoveRight),
                Event::Key(KeyEvent {
                               code: KeyCode::Up,
                               ..
                           }) => Some(InputResult::RotateClockwise),
                Event::Key(KeyEvent {
                               code: KeyCode::Char('z'),
                               ..
                           }) => Some(InputResult::RotateCounterClockwise),
                Event::Key(KeyEvent {
                               code: KeyCode::Down,
                               ..
                           }) => Some(InputResult::MoveDown),
                Event::Key(KeyEvent {
                               code: KeyCode::Esc,
                               ..
                           }) => Some(InputResult::ExitGame),
                _ => None,
            }
        }
        Err(error) => {
            println!("Error reading input: {}", error);
            None
        }
    }
}


