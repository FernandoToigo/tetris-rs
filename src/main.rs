mod tiles;
mod pieces;
mod game;
mod drawing;
#[cfg(test)]
mod tests;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use std::time::Duration;
use std::time::Instant;
use tiles::*;
use pieces::*;
use game::*;
use drawing::*;

fn main() {
    loop {
        if !play() {
            break;
        }
    }
}

fn play() -> bool {
    let mut game = initialize_game();
    draw_bounds(&mut game).unwrap();
    draw_tiles(&mut game).unwrap();
    redraw_piece(&mut game);
    flush(&mut game);

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

        flush(&mut game);
    }
}

fn read_input(game: &mut Game) -> bool {
    if poll(Duration::from_secs(0)).unwrap() {
        match read().unwrap() {
            Event::Key(KeyEvent {
                           code: KeyCode::Left,
                           ..
                       }) => {
                move_left(game);
            }
            Event::Key(KeyEvent {
                           code: KeyCode::Right,
                           ..
                       }) => {
                move_right(game);
            }
            Event::Key(KeyEvent {
                           code: KeyCode::Up,
                           ..
                       }) => {
                try_rotate_clockwise(game);
            }
            Event::Key(KeyEvent {
                           code: KeyCode::Char('z'),
                           ..
                       }) => {
                try_rotate_counterclockwise(game);
            }
            Event::Key(KeyEvent {
                           code: KeyCode::Down,
                           ..
                       }) => {
                fall_piece(game);
            }
            Event::Key(KeyEvent {
                           code: KeyCode::Esc,
                           ..
                       }) => {
                return true;
            }
            _ => (),
        }
    }

    false
}

fn move_left(game: &mut Game) {
    if can_move_left(game) {
        move_piece(game, Tile::new(-1, 0));
    }
}

fn can_move_left(game: &Game) -> bool {
    for tile in &game.current_piece.tiles {
        if tile.x == 0 {
            return false;
        }

        if game.map[*tile + Tile::new(-1, 0)].is_set {
            return false;
        }
    }

    true
}

fn move_right(game: &mut Game) {
    if can_move_right(game) {
        move_piece(game, Tile::new(1, 0));
    }
}

fn can_move_right(game: &Game) -> bool {
    for tile in &game.current_piece.tiles {
        if tile.x + 1 >= WIDTH as i16 {
            return false;
        }

        if game.map[*tile + Tile::new(1, 0)].is_set {
            return false;
        }
    }

    true
}

fn apply_gravity(game: &mut Game) {
    if game.last_move_instant.elapsed() > Duration::from_millis(1000) {
        fall_piece(game);
    }
}

fn fall_piece(game: &mut Game) {
    if !can_move_down(game) {
        for tile in &mut game.current_piece.tiles {
            game.map.tiles[tile.x as usize][tile.y as usize].is_set = true;
        }

        clear_lines(game);

        game.current_piece = create_piece();
        if !are_valid_positions(game, &game.current_piece.tiles) {
            game.ended = true;
            return;
        }

        game.last_move_instant = Instant::now();
        draw_tiles(game).unwrap();
        redraw_piece(game);
        return;
    }

    move_piece(game, Tile::new(0, 1));
    game.last_move_instant = Instant::now();
}

fn clear_lines(game: &mut Game) {
    for i in 0..HEIGHT as usize {
        let mut all_set = true;
        for x in 0..WIDTH as usize {
            if !game.map.tiles[x][i].is_set {
                all_set = false;
                break;
            }
        }

        if all_set {
            clear_line(game, i);
        }
    }
}

fn clear_line(game: &mut Game, line_index: usize) {
    for i in (1..=line_index).rev() {
        for x in 0..WIDTH as usize {
            game.map.tiles[x][i].is_set = game.map.tiles[x][i - 1].is_set;
        }
    }

    for x in 0..WIDTH as usize {
        game.map.tiles[x][0].is_set = false;
    }
}

fn try_rotate_clockwise(game: &mut Game) {
    let mut rotated_piece = game.current_piece.clone();
    rotate_clockwise(&mut rotated_piece);

    if !are_valid_positions(&game, &rotated_piece.tiles) {
        if !kick_piece(&game, &mut rotated_piece, 0) {
            return;
        }
    }

    erase_piece(game);
    game.current_piece = rotated_piece;
    redraw_piece(game);
}

fn rotate_clockwise(piece: &mut Piece) {
    for tile in &mut piece.tiles {
        let delta_from_origin = *tile - piece.origin;
        let new_delta_from_origin = Tile::new(piece.bounding_box_size - 1 - delta_from_origin.y, delta_from_origin.x);
        *tile = piece.origin + new_delta_from_origin;
    }

    piece.rotation_index = (piece.rotation_index + 1) % 4;
}

fn try_rotate_counterclockwise(game: &mut Game) {
    let mut rotated_piece = game.current_piece.clone();
    rotate_counterclockwise(&mut rotated_piece);

    if !are_valid_positions(&game, &rotated_piece.tiles) {
        if !kick_piece(&game, &mut rotated_piece, 1) {
            return;
        }
    }

    erase_piece(game);
    game.current_piece = rotated_piece;
    redraw_piece(game);
}

fn rotate_counterclockwise(piece: &mut Piece) {
    for tile in &mut piece.tiles {
        let delta_from_origin = *tile - piece.origin;
        let new_delta_from_origin = Tile::new(delta_from_origin.y, piece.bounding_box_size - 1 - delta_from_origin.x);
        *tile = piece.origin + new_delta_from_origin;
    }

    piece.rotation_index = (piece.rotation_index + 3) % 4;
}

fn kick_piece(game: &Game, piece: &mut Piece, array_offset: usize) -> bool {
    let tests_index = piece.rotation_index * 2 + array_offset;

    if piece.bounding_box_size == 3 {
        return kick_piece_with(&game, piece, SIZE_3_KICK_TESTS[tests_index]);
    } else if piece.bounding_box_size == 4 {
        return kick_piece_with(&game, piece, SIZE_4_KICK_TESTS[tests_index]);
    }

    false
}

fn kick_piece_with(game: &Game, piece: &mut Piece, test_delta_tiles: [Tile; 4]) -> bool {
    for test_delta_tile in &test_delta_tiles {
        let mut test_tiles = piece.tiles.clone();
        move_tiles(&mut test_tiles, *test_delta_tile);

        if are_valid_positions(game, &test_tiles) {
            piece.tiles = test_tiles;
            piece.origin += *test_delta_tile;
            return true;
        }
    }

    false
}

fn move_piece(game: &mut Game, delta: Tile) {
    erase_piece(game);

    move_tiles(&mut game.current_piece.tiles, delta);

    game.current_piece.origin = game.current_piece.origin + delta;
    redraw_piece(game);
}

fn move_tiles(tiles: &mut Vec<Tile>, delta: Tile) {
    for tile in tiles {
        *tile = *tile + delta;
    }
}

fn are_valid_positions(game: &Game, tiles: &Vec<Tile>) -> bool {
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

        if game.map[*tile].is_set {
            return false;
        }
    }

    true
}

fn can_move_down(game: &Game) -> bool {
    for tile in &game.current_piece.tiles {
        if tile.y == HEIGHT as i16 - 1 {
            return false;
        }

        if game.map[*tile + Tile::new(0, 1)].is_set {
            return false;
        }
    }

    true
}

