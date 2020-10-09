use crate::tiles::*;
use crate::game::*;
use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct Piece {
    pub tiles: Vec<Tile>,
    pub origin: Tile,
    pub bounding_box_size: i16,
    pub rotation_index: usize,
}

pub struct PieceType {
    pub tiles: [Tile; 4],
    pub origin: Tile,
    pub bounding_box_size: i16,
}

impl PieceType {
    const fn new(tiles: [Tile; 4], origin: Tile, bounding_box_size: i16) -> PieceType {
        PieceType {
            tiles,
            origin,
            bounding_box_size,
        }
    }
}

pub static SIZE_3_KICK_TESTS: [[Tile; 4]; 8] = [
    [Tile::new(-1, 0), Tile::new(-1, -1), Tile::new(0, 2), Tile::new(-1, 2)],
    [Tile::new(1, 0), Tile::new(1, 1), Tile::new(0, -2), Tile::new(1, -2)],
    [Tile::new(1, 0), Tile::new(1, 1), Tile::new(0, -2), Tile::new(1, -2)],
    [Tile::new(-1, 0), Tile::new(-1, -1), Tile::new(0, 2), Tile::new(-1, 2)],
    [Tile::new(1, 0), Tile::new(1, -1), Tile::new(0, 2), Tile::new(1, 2)],
    [Tile::new(-1, 0), Tile::new(-1, 1), Tile::new(0, -2), Tile::new(-1, -2)],
    [Tile::new(-1, 0), Tile::new(-1, 1), Tile::new(0, -2), Tile::new(-1, -2)],
    [Tile::new(1, 0), Tile::new(1, -1), Tile::new(0, 2), Tile::new(1, 2)]
];

pub static SIZE_4_KICK_TESTS: [[Tile; 4]; 8] = [
    [Tile::new(-2, 0), Tile::new(1, 0), Tile::new(-2, 1), Tile::new(1, -2)],
    [Tile::new(2, 0), Tile::new(-1, 0), Tile::new(2, -1), Tile::new(-1, 2)],
    [Tile::new(-1, 0), Tile::new(2, 0), Tile::new(-1, -2), Tile::new(2, 1)],
    [Tile::new(1, 0), Tile::new(-2, 0), Tile::new(1, 2), Tile::new(-2, -1)],
    [Tile::new(2, 0), Tile::new(-1, 0), Tile::new(2, -1), Tile::new(-1, 2)],
    [Tile::new(-2, 0), Tile::new(1, 0), Tile::new(-2, 1), Tile::new(1, -2)],
    [Tile::new(1, 0), Tile::new(-2, 0), Tile::new(1, 2), Tile::new(-2, -1)],
    [Tile::new(-1, 0), Tile::new(2, 0), Tile::new(-1, -2), Tile::new(2, 1)]
];

pub static ALL_PIECES: [PieceType; 7] = [
    //Plank
    PieceType::new(
        [
            Tile::new(0, 0),
            Tile::new(1, 0),
            Tile::new(2, 0),
            Tile::new(3, 0),
        ],
        Tile::new(0, -1),
        4),
    //J
    PieceType::new(
        [
            Tile::new(0, 0),
            Tile::new(0, 1),
            Tile::new(1, 1),
            Tile::new(2, 1)
        ],
        Tile::new(0, 0),
        3),
    //L
    PieceType::new(
        [
            Tile::new(2, 0),
            Tile::new(0, 1),
            Tile::new(1, 1),
            Tile::new(2, 1)
        ], Tile::new(0, 0),
        3),
    //SQUARE
    PieceType::new(
        [
            Tile::new(0, 0),
            Tile::new(1, 0),
            Tile::new(0, 1),
            Tile::new(1, 1)
        ], Tile::new(0, 0),
        2),
    //S
    PieceType::new(
        [
            Tile::new(0, 1),
            Tile::new(1, 1),
            Tile::new(1, 0),
            Tile::new(2, 0)
        ], Tile::new(0, 0),
        3),
    //Z
    PieceType::new(
        [
            Tile::new(0, 0),
            Tile::new(1, 0),
            Tile::new(1, 1),
            Tile::new(2, 1)
        ], Tile::new(0, 0),
        3),
    //T
    PieceType::new(
        [
            Tile::new(1, 0),
            Tile::new(0, 1),
            Tile::new(1, 1),
            Tile::new(2, 1)
        ], Tile::new(0, 0),
        3)
];

pub fn create_piece() -> Piece {
    let piece_type = get_random_piece_type();
    let mut tiles = piece_type.tiles.to_vec();
    let start_x = WIDTH as i16 / 2 - (piece_type.bounding_box_size as f32 / 2f32).ceil() as i16;

    for tile in &mut tiles {
        tile.x += start_x;
    }

    Piece {
        tiles,
        origin: piece_type.origin + Tile::new(start_x, 0),
        bounding_box_size: piece_type.bounding_box_size,
        rotation_index: 0,
    }
}

fn get_random_piece_type() -> &'static PieceType {
    let mut rng = rand::thread_rng();
    ALL_PIECES.choose(&mut rng).unwrap()
}

