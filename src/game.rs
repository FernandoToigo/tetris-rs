use crate::tiles::*;
use crate::pieces::*;
use crate::input::*;
use std::time::Instant;
use std::io::Stdout;
use std::ops::Index;
use std::io::stdout;

pub const WIDTH: u16 = 10;
pub const HEIGHT: u16 = 20;

#[derive(Clone, Copy)]
pub struct MapTile {
    pub tile: Tile,
    pub is_set: bool,
}

pub struct Game<TInput: InputSource> {
    pub map: Map,
    pub falling_piece: Piece,
    pub last_move_instant: Instant,
    pub ended: bool,
    pub input: TInput,
    pub stdout: Stdout
}

pub struct Map {
    pub tiles: [[MapTile; HEIGHT as usize]; WIDTH as usize],
}

impl Index<Tile> for Map {
    type Output = MapTile;
    fn index(&self, tile: Tile) -> &MapTile {
        &self.tiles[tile.x as usize][tile.y as usize]
    }
}

pub fn initialize_game<TInput: InputSource>(input: TInput) -> Game<TInput> {
    Game {
        map: initialize_map(),
        falling_piece: create_piece(),
        last_move_instant: Instant::now(),
        stdout: stdout(),
        ended: false,
        input
    }
}

fn initialize_map() -> Map {
    let mut tiles: [[MapTile; HEIGHT as usize]; WIDTH as usize] = [[MapTile {
        tile: Tile::new(0, 0),
        is_set: false,
    }; HEIGHT as usize]; WIDTH as usize];

    for x in 0usize..WIDTH as usize {
        for y in 0usize..HEIGHT as usize {
            tiles[x][y].tile = Tile {
                x: x as i16,
                y: y as i16,
            }
        }
    }

    Map { tiles }
}

