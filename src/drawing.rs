use crossterm::terminal::{Clear, ClearType};
use crossterm::style::{self, style, Color, Colorize};
use crossterm::{cursor, QueueableCommand, Result};
use std::io::Write;
use crate::game::*;
use crate::tiles::*;

pub fn draw_bounds(game: &mut Game) -> Result<()> {
    game.stdout
        .queue(Clear(ClearType::All))?
        .queue(cursor::Hide {})?;

    for y in 0..HEIGHT + 2 {
        for x in 0..WIDTH * 2 + 4 {
            if y == 0 || y == HEIGHT + 1 || x <= 1 || x >= WIDTH * 2 + 2 {
                game.stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("█".dark_grey()))?;
            }
        }
    }

    Ok(())
}

pub fn erase_piece(game: &mut Game) {
    draw_piece(game, Color::White).unwrap();
}

pub fn redraw_piece(game: &mut Game) {
    draw_piece(game, Color::Blue).unwrap();
}

fn draw_piece(game: &mut Game, color: Color) -> Result<()> {
    for tile in &game.current_piece.tiles {
        let screen_tile = tile.to_screen_space();
        game.stdout
            .queue(cursor::MoveTo(screen_tile.x as u16, screen_tile.y as u16))?
            .queue(style::PrintStyledContent(style("██").with(color)))?;
    }

    Ok(())
}

pub fn draw_tiles(game: &mut Game) -> Result<()> {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = match game.map.tiles[x as usize][y as usize].is_set {
                true => Color::Red,
                false => Color::White,
            };

            let screen_tile = Tile::new(x as i16, y as i16).to_screen_space();
            game.stdout
                .queue(cursor::MoveTo(screen_tile.x as u16, screen_tile.y as u16))?
                .queue(style::PrintStyledContent(style("██").with(color)))?;
        }
    }

    Ok(())
}

pub fn flush(game: &mut Game) {
    game.stdout.flush().unwrap();
}