use crate::tiles::*;
use crate::pieces::*;
use crate::input::*;
use crate::time::*;
use std::ops::Index;
use crate::drawing::Drawing;

pub const WIDTH: u16 = 10;
pub const HEIGHT: u16 = 20;

#[derive(Clone, Copy)]
pub struct MapTile {
    pub tile: Tile,
    pub is_set: bool,
}

pub struct Game<TInput: InputSource, TPTS: PieceTypeSelector, TCI: ClockInstant, TC: Clock<TCI>, TD: Drawing> {
    pub state: GameState,
    pub clock: TC,
    pub last_move_instant: TCI,
    pub ended: bool,
    pub input: TInput,
    pub piece_type_selector: TPTS,
    pub drawing: TD
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

