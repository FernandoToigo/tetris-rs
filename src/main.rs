mod tiles;
mod pieces;
mod game;
mod drawing;
#[cfg(test)]
mod tests;
mod input;

use std::time::Duration;
use std::time::Instant;
use tiles::*;
use pieces::*;
use game::*;
use drawing::*;
use input::*;

fn main() {
    loop {
        if !play(CrosstermInput{}) {
            break;
        }
    }
}

// random
// render
// input
// time

fn play<I: InputSource>(input: I) -> bool {
    let mut game = initialize_game(input);
    draw_bounds(&mut game.stdout).unwrap();
    draw_tiles(&mut game.stdout, &game.map).unwrap();
    redraw_piece(&mut game.stdout, &game.falling_piece);
    flush(&mut game.stdout);

    loop {
        if read_input(&mut game) {
            return false;
        }

        if game.ended {
            return true;
        }

        apply_gravity(&mut game);

        if game.ended {
            return true;
        }

        flush(&mut game.stdout);
    }
}

fn read_input<I: InputSource>(game: &mut Game<I>) -> bool {
    let input_read = game.input.read_input();
    match input_read.as_ref() {
        Some(input) => match input {
            InputResult::MoveLeft => move_left(game),
            InputResult::MoveRight => move_right(game),
            InputResult::MoveDown => fall_piece(game),
            InputResult::RotateClockwise => try_rotate_clockwise(game),
            InputResult::RotateCounterClockwise => try_rotate_counterclockwise(game),
            _ => {}
        },
        None => {}
    }
    
    match input_read.as_ref() {
        Some(input) => match input {
            InputResult::ExitGame => true,
            _ => false
        },
        None => false
    }
}

fn move_left<I: InputSource>(game: &mut Game<I>) {
    if can_move_left(game) {
        move_piece(game, Tile::new(-1, 0));
    }
}

fn can_move_left<I: InputSource>(game: &Game<I>) -> bool {
    for tile in &game.falling_piece.tiles {
        if tile.x == 0 {
            return false;
        }

        if game.map[*tile + Tile::new(-1, 0)].is_set {
            return false;
        }
    }

    true
}

fn move_right<I: InputSource>(game: &mut Game<I>) {
    if can_move_right(game) {
        move_piece(game, Tile::new(1, 0));
    }
}

fn can_move_right<I: InputSource>(game: &Game<I>) -> bool {
    for tile in &game.falling_piece.tiles {
        if tile.x + 1 >= WIDTH as i16 {
            return false;
        }

        if game.map[*tile + Tile::new(1, 0)].is_set {
            return false;
        }
    }

    true
}

fn apply_gravity<I: InputSource>(game: &mut Game<I>) {
    if game.last_move_instant.elapsed() > Duration::from_millis(1000) {
        fall_piece(game);
    }
}

fn fall_piece<I: InputSource>(game: &mut Game<I>) {
    if !can_move_down(game) {
        for tile in &mut game.falling_piece.tiles {
            game.map.tiles[tile.x as usize][tile.y as usize].is_set = true;
        }

        clear_lines(game);

        game.falling_piece = create_piece();
        if !are_valid_positions(&game.map, &game.falling_piece.tiles) {
            game.ended = true;
            return;
        }

        game.last_move_instant = Instant::now();
        draw_tiles(&mut game.stdout, &game.map).unwrap();
        redraw_piece(&mut game.stdout, &game.falling_piece);
        return;
    }

    move_piece(game, Tile::new(0, 1));
    game.last_move_instant = Instant::now();
}

fn clear_lines<I: InputSource>(game: &mut Game<I>) {
    for i in 0..HEIGHT as usize {
        let mut all_set = true;
        for x in 0..WIDTH as usize {
            if !game.map.tiles[x][i].is_set {
                all_set = false;
                break;
            }
        }

        if all_set {
            clear_line(&mut game.map, i);
        }
    }
}

fn clear_line(map: &mut Map, line_index: usize) {
    for i in (1..=line_index).rev() {
        for x in 0..WIDTH as usize {
            map.tiles[x][i].is_set = map.tiles[x][i - 1].is_set;
        }
    }

    for x in 0..WIDTH as usize {
        map.tiles[x][0].is_set = false;
    }
}

fn try_rotate_clockwise<I: InputSource>(game: &mut Game<I>) {
    let mut rotated_piece = game.falling_piece.clone();
    rotate_clockwise(&mut rotated_piece);

    if !are_valid_positions(&game.map, &rotated_piece.tiles) {
        if !kick_piece(&game.map, &mut rotated_piece, 0) {
            return;
        }
    }

    erase_piece(&mut game.stdout, &game.falling_piece);
    game.falling_piece = rotated_piece;
    redraw_piece(&mut game.stdout, &game.falling_piece);
}

fn rotate_clockwise(piece: &mut Piece) {
    for tile in &mut piece.tiles {
        let delta_from_origin = *tile - piece.origin;
        let new_delta_from_origin = Tile::new(piece.bounding_box_size - 1 - delta_from_origin.y, delta_from_origin.x);
        *tile = piece.origin + new_delta_from_origin;
    }

    piece.rotation_index = (piece.rotation_index + 1) % 4;
}

fn try_rotate_counterclockwise<I: InputSource>(game: &mut Game<I>) {
    let mut rotated_piece = game.falling_piece.clone();
    rotate_counterclockwise(&mut rotated_piece);

    if !are_valid_positions(&game.map, &rotated_piece.tiles) {
        if !kick_piece(&game.map, &mut rotated_piece, 1) {
            return;
        }
    }

    erase_piece(&mut game.stdout, &game.falling_piece);
    game.falling_piece = rotated_piece;
    redraw_piece(&mut game.stdout, &game.falling_piece);
}

fn rotate_counterclockwise(piece: &mut Piece) {
    for tile in &mut piece.tiles {
        let delta_from_origin = *tile - piece.origin;
        let new_delta_from_origin = Tile::new(delta_from_origin.y, piece.bounding_box_size - 1 - delta_from_origin.x);
        *tile = piece.origin + new_delta_from_origin;
    }

    piece.rotation_index = (piece.rotation_index + 3) % 4;
}

fn kick_piece(map: &Map, piece: &mut Piece, array_offset: usize) -> bool {
    let tests_index = piece.rotation_index * 2 + array_offset;

    match piece.bounding_box_size {
        3 => kick_piece_with(&map, piece, SIZE_3_KICK_TESTS[tests_index]),
        4 => kick_piece_with(&map, piece, SIZE_4_KICK_TESTS[tests_index]),
        _ => false
    }
}

fn kick_piece_with(map: &Map, piece: &mut Piece, test_delta_tiles: [Tile; 4]) -> bool {
    for test_delta_tile in &test_delta_tiles {
        let mut test_tiles = piece.tiles.clone();
        move_tiles(&mut test_tiles, *test_delta_tile);

        if are_valid_positions(map, &test_tiles) {
            piece.tiles = test_tiles;
            piece.origin += *test_delta_tile;
            return true;
        }
    }

    false
}

fn move_piece<I: InputSource>(game: &mut Game<I>, delta: Tile) {
    erase_piece(&mut game.stdout, &game.falling_piece);

    move_tiles(&mut game.falling_piece.tiles, delta);

    game.falling_piece.origin = game.falling_piece.origin + delta;
    redraw_piece(&mut game.stdout, &game.falling_piece);
}

fn move_tiles(tiles: &mut Vec<Tile>, delta: Tile) {
    for tile in tiles {
        *tile = *tile + delta;
    }
}

fn are_valid_positions(map: &Map, tiles: &Vec<Tile>) -> bool {
    for tile in tiles {
        
        if tile.y < 0 {
            return false;
        }

        if tile.y >= HEIGHT as i16 {
            return false;
        }

        if tile.x < 0 {
            return false;
        }

        if tile.x >= WIDTH as i16 {
            return false;
        }

        if map[*tile].is_set {
            return false;
        }
    }

    true
}

fn can_move_down<I: InputSource>(game: &Game<I>) -> bool {
    for tile in &game.falling_piece.tiles {
        if tile.y == HEIGHT as i16 - 1 {
            return false;
        }

        if game.map[*tile + Tile::new(0, 1)].is_set {
            return false;
        }
    }

    true
}

