use super::*;
use std::time::Instant;
use crate::time::ManualClock;

#[test]
fn rotate_clockwise_plank_piece() {
    let mut piece = Piece {
        tiles: vec![Tile::new(5, 5), Tile::new(6, 5), Tile::new(7, 5), Tile::new(8, 5)],
        bounding_box_size: 4,
        origin: Tile::new(5, 4),
        rotation_index: 0,
    };

    rotate_clockwise(&mut piece);

    assert_eq!(piece.tiles[0], Tile::new(7, 4));
    assert_eq!(piece.tiles[1], Tile::new(7, 5));
    assert_eq!(piece.tiles[2], Tile::new(7, 6));
    assert_eq!(piece.tiles[3], Tile::new(7, 7));
}

#[test]
fn rotate_counterclockwise_plank_piece() {
    let mut piece = Piece {
        tiles: vec![Tile::new(5, 5), Tile::new(6, 5), Tile::new(7, 5), Tile::new(8, 5)],
        bounding_box_size: 4,
        origin: Tile::new(5, 4),
        rotation_index: 0,
    };

    rotate_counterclockwise(&mut piece);

    assert_eq!(piece.tiles[0], Tile::new(6, 7));
    assert_eq!(piece.tiles[1], Tile::new(6, 6));
    assert_eq!(piece.tiles[2], Tile::new(6, 5));
    assert_eq!(piece.tiles[3], Tile::new(6, 4));
}

#[test]
fn rotate_clockwise_l_piece() {
    let mut piece = Piece {
        tiles: vec![Tile::new(5, 5), Tile::new(6, 5), Tile::new(7, 5), Tile::new(7, 6)],
        bounding_box_size: 3,
        origin: Tile::new(5, 4),
        rotation_index: 2,
    };

    rotate_clockwise(&mut piece);

    assert_eq!(piece.tiles[0], Tile::new(6, 4));
    assert_eq!(piece.tiles[1], Tile::new(6, 5));
    assert_eq!(piece.tiles[2], Tile::new(6, 6));
    assert_eq!(piece.tiles[3], Tile::new(5, 6));
}

#[test]
fn rotate_counterclockwise_l_piece() {
    let mut piece = Piece {
        tiles: vec![Tile::new(5, 5), Tile::new(6, 5), Tile::new(7, 5), Tile::new(7, 6)],
        bounding_box_size: 3,
        origin: Tile::new(5, 4),
        rotation_index: 2,
    };

    rotate_counterclockwise(&mut piece);

    assert_eq!(piece.tiles[0], Tile::new(6, 6));
    assert_eq!(piece.tiles[1], Tile::new(6, 5));
    assert_eq!(piece.tiles[2], Tile::new(6, 4));
    assert_eq!(piece.tiles[3], Tile::new(7, 4));
}

#[test]
fn run_simple_game() {
    let mut game = Game::new(
        ManualInput {},
        RandomPieceTypeSelector {},
        ManualClock {
            now_milliseconds: 0
        }, 
        StdoutDrawing{
            stdout: stdout()
        });
    game.drawing.init();

    let instant = Instant::now();
    while !game.run_frame() {
        let elapsed_millis = instant.elapsed().as_millis();
        game.clock.now_milliseconds = elapsed_millis * 100;
        
        assert!(elapsed_millis < 10_000)
    }
}

