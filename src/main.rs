mod tiles;
mod pieces;
mod game;
mod drawing;
#[cfg(test)]
mod tests;
mod input;

use std::time::Duration;
use std::time::Instant;
use std::io::stdout;
use tiles::*;
use pieces::*;
use game::*;
use drawing::*;
use input::*;

fn main() {
    loop {
        if !play(CrosstermInput {}, RandomPieceTypeSelector {}) {
            break;
        }
    }
}

// random
// render
// time

fn play<I: InputSource, PTS: PieceTypeSelector>(input: I, piece_type_selector: PTS) -> bool {
    let mut game = Game::initialize_game(input, piece_type_selector);
    draw_bounds(&mut game.stdout).unwrap();
    draw_tiles(&mut game.stdout, &game.map).unwrap();
    redraw_piece(&mut game.stdout, &game.falling_piece);
    flush(&mut game.stdout);

    loop {
        if game.read_input() {
            return false;
        }

        if game.ended {
            return true;
        }

        game.apply_gravity();

        if game.ended {
            return true;
        }

        flush(&mut game.stdout);
    }
}

impl<I: InputSource, PTS: PieceTypeSelector> Game<I, PTS> {
    pub fn initialize_game(input: I, piece_type_selector: PTS) -> Game<I, PTS> {
        Game {
            map: Game::<I, PTS>::initialize_map(),
            falling_piece: Game::<I, PTS>::create_piece(&piece_type_selector),
            last_move_instant: Instant::now(),
            stdout: stdout(),
            ended: false,
            input,
            piece_type_selector
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

    fn read_input(&mut self) -> bool {
        let input_read = self.input.read_input();
        match input_read.as_ref() {
            Some(input) => match input {
                InputResult::MoveLeft => self.move_left(),
                InputResult::MoveRight => self.move_right(),
                InputResult::MoveDown => self.fall_piece(),
                InputResult::RotateClockwise => self.try_rotate_clockwise(),
                InputResult::RotateCounterClockwise => self.try_rotate_counterclockwise(),
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

    fn move_left(&mut self) {
        if self.can_move_left() {
            self.move_piece(Tile::new(-1, 0));
        }
    }

    fn can_move_left(&self) -> bool {
        for tile in &self.falling_piece.tiles {
            if tile.x == 0 {
                return false;
            }

            if self.map[*tile + Tile::new(-1, 0)].is_set {
                return false;
            }
        }

        true
    }

    fn move_right(&mut self) {
        if self.can_move_right() {
            self.move_piece(Tile::new(1, 0));
        }
    }

    fn can_move_right(&self) -> bool {
        for tile in &self.falling_piece.tiles {
            if tile.x + 1 >= WIDTH as i16 {
                return false;
            }

            if self.map[*tile + Tile::new(1, 0)].is_set {
                return false;
            }
        }

        true
    }

    fn apply_gravity(&mut self) {
        if self.last_move_instant.elapsed() > Duration::from_millis(1000) {
            self.fall_piece();
        }
    }

    fn fall_piece(&mut self) {
        if !self.can_move_down() {
            for tile in &mut self.falling_piece.tiles {
                self.map.tiles[tile.x as usize][tile.y as usize].is_set = true;
            }

            self.clear_complete_lines();

            self.falling_piece = Game::<I, PTS>::create_piece(&self.piece_type_selector);
            if !self.are_valid_positions(&self.falling_piece.tiles) {
                self.ended = true;
                return;
            }

            self.last_move_instant = Instant::now();
            draw_tiles(&mut self.stdout, &self.map).unwrap();
            redraw_piece(&mut self.stdout, &self.falling_piece);
            return;
        }

        self.move_piece(Tile::new(0, 1));
        self.last_move_instant = Instant::now();
    }

    fn clear_complete_lines(&mut self) {
        for i in 0..HEIGHT as usize {
            let mut all_set = true;
            for x in 0..WIDTH as usize {
                if !self.map.tiles[x][i].is_set {
                    all_set = false;
                    break;
                }
            }

            if all_set {
                self.clear_line(i);
            }
        }
    }

    fn clear_line(&mut self, line_index: usize) {
        for i in (1..=line_index).rev() {
            for x in 0..WIDTH as usize {
                self.map.tiles[x][i].is_set = self.map.tiles[x][i - 1].is_set;
            }
        }

        for x in 0..WIDTH as usize {
            self.map.tiles[x][0].is_set = false;
        }
    }

    fn try_rotate_clockwise(&mut self) {
        let mut rotated_piece = self.falling_piece.clone();
        rotate_clockwise(&mut rotated_piece);

        if !self.are_valid_positions(&rotated_piece.tiles) {
            if !self.kick_piece(&rotated_piece, 0) {
                return;
            }
        }

        erase_piece(&mut self.stdout, &self.falling_piece);
        self.falling_piece = rotated_piece;
        redraw_piece(&mut self.stdout, &self.falling_piece);
    }

    fn try_rotate_counterclockwise(&mut self) {
        let mut rotated_piece = self.falling_piece.clone();
        rotate_counterclockwise(&mut rotated_piece);

        if !self.are_valid_positions(&rotated_piece.tiles) {
            if !self.kick_piece(&mut rotated_piece, 1) {
                return;
            }
        }

        erase_piece(&mut self.stdout, &self.falling_piece);
        self.falling_piece = rotated_piece;
        redraw_piece(&mut self.stdout, &self.falling_piece);
    }

    fn kick_piece(&mut self, piece: &Piece, array_offset: usize) -> bool {
        let tests_index = piece.rotation_index * 2 + array_offset;

        match piece.bounding_box_size {
            3 => self.kick_piece_with(SIZE_3_KICK_TESTS[tests_index]),
            4 => self.kick_piece_with(SIZE_4_KICK_TESTS[tests_index]),
            _ => false
        }
    }

    fn kick_piece_with(&mut self, test_delta_tiles: [Tile; 4]) -> bool {
        for test_delta_tile in &test_delta_tiles {
            let mut test_tiles = self.falling_piece.tiles.clone();
            Game::<I, PTS>::move_tiles(&mut test_tiles, *test_delta_tile);

            if self.are_valid_positions(&test_tiles) {
                self.falling_piece.tiles = test_tiles;
                self.falling_piece.origin += *test_delta_tile;
                return true;
            }
        }

        false
    }

    fn move_piece(&mut self, delta: Tile) {
        erase_piece(&mut self.stdout, &self.falling_piece);

        Game::<I, PTS>::move_tiles(&mut self.falling_piece.tiles, delta);

        self.falling_piece.origin = self.falling_piece.origin + delta;
        redraw_piece(&mut self.stdout, &self.falling_piece);
    }

    fn move_tiles(tiles: &mut Vec<Tile>, delta: Tile) {
        for tile in tiles {
            *tile = *tile + delta;
        }
    }

    fn are_valid_positions(&self, tiles: &Vec<Tile>) -> bool {
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

            if self.map[*tile].is_set {
                return false;
            }
        }

        true
    }

    fn can_move_down(&self) -> bool {
        for tile in &self.falling_piece.tiles {
            if tile.y == HEIGHT as i16 - 1 {
                return false;
            }

            if self.map[*tile + Tile::new(0, 1)].is_set {
                return false;
            }
        }

        true
    }
    
    pub fn create_piece(piece_type_selector: &PTS) -> Piece {
        let piece_type = piece_type_selector.select_piece_type(&ALL_PIECES);
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
}
fn rotate_clockwise(piece: &mut Piece) {
    for tile in &mut piece.tiles {
        let delta_from_origin = *tile - piece.origin;
        let new_delta_from_origin = Tile::new(piece.bounding_box_size - 1 - delta_from_origin.y, delta_from_origin.x);
        *tile = piece.origin + new_delta_from_origin;
    }

    piece.rotation_index = (piece.rotation_index + 1) % 4;
}

fn rotate_counterclockwise(piece: &mut Piece) {
    for tile in &mut piece.tiles {
        let delta_from_origin = *tile - piece.origin;
        let new_delta_from_origin = Tile::new(delta_from_origin.y, piece.bounding_box_size - 1 - delta_from_origin.x);
        *tile = piece.origin + new_delta_from_origin;
    }

    piece.rotation_index = (piece.rotation_index + 3) % 4;
}

