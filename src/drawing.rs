use crossterm::terminal::{Clear, ClearType};
use crossterm::style::{self, style, Color, Colorize};
use crossterm::{cursor, QueueableCommand, Result};
use std::io::{Write, Stdout};
use crate::game::*;
use crate::tiles::*;
use crate::pieces::Piece;

impl StdoutDrawing {
    
    fn clear(&mut self) {
        self.stdout
            .queue(Clear(ClearType::All))
            .unwrap();
    }
    
    fn draw_bounds(&mut self) -> Result<()> {
        self.stdout
            .queue(cursor::Hide {})?;

        for y in 0..HEIGHT + 2 {
            for x in 0..WIDTH * 2 + 4 {
                if y == 0 || y == HEIGHT + 1 || x <= 1 || x >= WIDTH * 2 + 2 {
                    self.stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent("█".dark_grey()))?;
                }
            }
        }

        Ok(())
    }

    fn draw_piece(&mut self, piece: &Piece) -> Result<()> {
        for tile in &piece.tiles {
            let screen_tile = tile.to_screen_space();
            self.stdout
                .queue(cursor::MoveTo(screen_tile.x as u16, screen_tile.y as u16))?
                .queue(style::PrintStyledContent(style("██").with(Color::Blue)))?;
        }

        Ok(())
    }

    fn draw_tiles(&mut self, state: &GameState) -> Result<()> {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = match state.map.tiles[x as usize][y as usize].is_set {
                    true => Color::Red,
                    false => Color::White,
                };

                let mut iter = state.falling_piece.tiles.iter();
                if iter.find(|t| t.x == x as i16 && t.y == y as i16) != None {
                    continue;
                }

                let screen_tile = Tile::new(x as i16, y as i16).to_screen_space();
                self.stdout
                    .queue(cursor::MoveTo(screen_tile.x as u16, screen_tile.y as u16))?
                    .queue(style::PrintStyledContent(style("██").with(color)))?;
            }
        }

        Ok(())
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}

pub trait Drawing {
    fn init(&mut self);
    fn draw(&mut self, state: &GameState);
}

pub struct StdoutDrawing {
    pub stdout: Stdout
}

impl Drawing for StdoutDrawing {
    fn init(&mut self) {
        self.clear();
    }

    fn draw(&mut self, state: &GameState) {
        self.draw_bounds().unwrap();
        self.draw_tiles(&state).unwrap();
        self.draw_piece(&state.falling_piece).unwrap();
        self.flush();
    }
}

pub struct NoopDrawing {
}

impl Drawing for NoopDrawing {
    fn init(&mut self) {
    }

    fn draw(&mut self, _: &GameState) {
    }
}