use super::*;
use std::time::Instant;
use crate::time::{ManualClock, ManualClockInstant};

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

fn create_test_game<F>(next_input_func: F) -> Game<
    ManualInput<F>, 
    ManualPieceTypeSelector, 
    ManualClockInstant, 
    ManualClock, 
    NoopDrawing> where F: FnMut() -> Option<InputResult> {
   Game::new(
        ManualInput { next_input_func },
        ManualPieceTypeSelector { piece_index: 0 },
        ManualClock { now_milliseconds: 0 },
        NoopDrawing {})
}

fn _create_test_visual_game<F>(next_input_func: F) -> Game<
    ManualInput<F>, 
    ManualPieceTypeSelector, 
    ManualClockInstant, 
    ManualClock, 
    StdoutDrawing>  where F: FnMut() -> Option<InputResult> {
    Game::new(
        ManualInput { next_input_func },
        ManualPieceTypeSelector { piece_index: 0 },
        ManualClock { now_milliseconds: 0 },
        StdoutDrawing{ stdout: stdout() })
}

#[test]
fn lost_game() {
    let mut game = create_test_game(|| None);
    
    let instant = Instant::now();
    
    loop {
        match game.run_frame() {
            FrameResult::GameInProgress => {},
            FrameResult::PlayerLost => break,
            _ => assert!(false)
        }
        
        let elapsed_millis = instant.elapsed().as_millis();
        game.clock.now_milliseconds = elapsed_millis * 1000;
    }
}

#[test]
fn quit_game() {
    let quit_game_input_func = || Some(InputResult::ExitGame);
    let mut game = create_test_game(quit_game_input_func);
    
    let result = game.run_frame();
    
    assert_eq!(result, FrameResult::GameQuitRequested);
}

#[test]
fn multiple_inputs_on_single_frame() {
    let mut input_queue = Vec::new();
    input_queue.push(InputResult::MoveLeft);
    input_queue.push(InputResult::MoveLeft);
    input_queue.push(InputResult::MoveLeft);

    let mut game = create_test_game(|| input_queue.pop());
    
    game.run_frame();
    
    let mut falling_piece_tiles = game.state.falling_piece.tiles.iter();
    assert!(falling_piece_tiles.any(|&tile| tile == Tile { x: 0, y: 1 }));
    assert!(falling_piece_tiles.any(|&tile| tile == Tile { x: 1, y: 1 }));
    assert!(falling_piece_tiles.any(|&tile| tile == Tile { x: 2, y: 1 }));
    assert!(falling_piece_tiles.any(|&tile| tile == Tile { x: 3, y: 1 }));
}

#[test]
fn clear_single_line() {
    let mut input_queue = InputList { 0: Vec::new() };
    input_queue.push_many(InputResult::MoveLeft, 3);
    input_queue.push_many(InputResult::MoveDown, 19);
    input_queue.push_many(InputResult::MoveRight, 3);
    input_queue.push_many(InputResult::MoveDown, 19);
    input_queue.push(InputResult::RotateClockwise);
    input_queue.push(InputResult::MoveLeft);
    input_queue.push_many(InputResult::MoveDown, 17);
    input_queue.push(InputResult::RotateClockwise);
    input_queue.push_many(InputResult::MoveLeft, 2);
    input_queue.push_many(InputResult::MoveDown, 17);

    let mut game = create_test_game(|| input_queue.pop_front());
    
    game.run_frame();
    
    assert_line(&game.state, 19, [false, false, false, false, true, true, false, false, false, false]);
}

#[test]
fn clear_multiple_lines() {
    let mut input_queue = InputList { 0: Vec::new() };
    input_queue.push(InputResult::RotateClockwise);
    input_queue.push_many(InputResult::MoveLeft, 6);
    input_queue.push_many(InputResult::MoveDown, 17);
    input_queue.push(InputResult::RotateClockwise);
    input_queue.push_many(InputResult::MoveLeft, 5);
    input_queue.push_many(InputResult::MoveDown, 17);
    input_queue.push(InputResult::RotateClockwise);
    input_queue.push_many(InputResult::MoveLeft, 4);
    input_queue.push_many(InputResult::MoveDown, 17);
    input_queue.push(InputResult::RotateClockwise);
    input_queue.push_many(InputResult::MoveLeft, 3);
    input_queue.push_many(InputResult::MoveDown, 17);
    input_queue.push(InputResult::RotateClockwise);
    input_queue.push_many(InputResult::MoveLeft, 2);
    input_queue.push_many(InputResult::MoveDown, 17);
    input_queue.push(InputResult::RotateClockwise);
    input_queue.push_many(InputResult::MoveLeft, 1);
    input_queue.push_many(InputResult::MoveDown, 17);
    input_queue.push(InputResult::RotateClockwise);
    input_queue.push_many(InputResult::MoveDown, 17);
    input_queue.push(InputResult::RotateClockwise);
    input_queue.push_many(InputResult::MoveRight, 1);
    input_queue.push_many(InputResult::MoveDown, 17);
    input_queue.push(InputResult::RotateClockwise);
    input_queue.push_many(InputResult::MoveRight, 2);
    input_queue.push_many(InputResult::MoveDown, 17);
    input_queue.push(InputResult::RotateClockwise);
    input_queue.push_many(InputResult::MoveRight, 3);
    input_queue.push_many(InputResult::MoveDown, 17);

    let mut game = create_test_game(|| input_queue.pop_front());
    
    game.run_frame();

    assert_line(&game.state, 16, [false, false, false, false, false, false, false, false, false, false]);
    assert_line(&game.state, 17, [false, false, false, false, false, false, false, false, false, false]);
    assert_line(&game.state, 18, [false, false, false, false, false, false, false, false, false, false]);
    assert_line(&game.state, 19, [false, false, false, false, false, false, false, false, false, false]);
}

fn assert_line(state: &GameState, index: usize, is_set_values: [bool; WIDTH as usize]) {
    for i in 0..WIDTH as usize {
        assert_eq!(state.map.tiles[i][index].is_set, is_set_values[i]);
    }
}

struct InputList(Vec<InputResult>);

impl InputList {
    fn pop_front(&mut self) -> Option<InputResult> {
        match self.0.len() {
            0 => None,
            _ => Some(self.0.remove(0))
        }
    }
    
    fn push(&mut self, input: InputResult) {
        self.0.push(input);
    }
    
    fn push_many(&mut self, input: InputResult, count: u32) {
        for _ in 0..count {
            self.0.push(input);
        }
    }
}