use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Tile {
    pub x: i16,
    pub y: i16,
}

impl Tile {
    pub const fn new(x: i16, y: i16) -> Tile {
        Tile { x, y }
    }

    pub fn to_screen_space(self) -> Tile {
        Tile {
            x: self.x * 2 + 2,
            y: self.y + 1,
        }
    }
}

impl Add for Tile {
    type Output = Tile;

    fn add(self, rhs: Self) -> Self {
        Tile::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Tile {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Tile {
    type Output = Tile;

    fn sub(self, rhs: Self) -> Self {
        Tile::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Tile {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}