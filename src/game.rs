use crate::tiles::*;
use crate::pieces::*;
use crate::input::*;
use std::time::Instant;
use std::ops::Index;
use std::io::Stdout;

pub const WIDTH: u16 = 10;
pub const HEIGHT: u16 = 20;

#[derive(Clone, Copy)]
pub struct MapTile {
    pub tile: Tile,
    pub is_set: bool,
}

pub struct Game<TInput: InputSource, TPTS: PieceTypeSelector> {
    pub state: GameState,
    pub last_move_instant: Instant,
    pub ended: bool,
    pub input: TInput,
    pub piece_type_selector: TPTS,
    pub stdout: Stdout
}

pub struct GameState {
    pub falling_piece: Piece,
    pub map: Map
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

