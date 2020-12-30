use crate::tiles::*;
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

pub trait PieceTypeSelector {
    fn select_piece_type<'a>(&self, available_piece_types: &'a[PieceType; 7]) -> &'a PieceType;
}

pub struct RandomPieceTypeSelector {}

impl PieceTypeSelector for RandomPieceTypeSelector {
    fn select_piece_type<'a>(&self, available_piece_types: &'a[PieceType; 7]) -> &'a PieceType {
        let mut rng = rand::thread_rng();
        available_piece_types.choose(&mut rng).unwrap()
    }
}

