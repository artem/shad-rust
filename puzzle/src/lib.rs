#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet, VecDeque};

////////////////////////////////////////////////////////////////////////////////

/// Represents a tile on a board. A tile can either be empty or a number from 1 to 8.
pub struct Tile(u8);

impl Tile {
    /// Creates a new tile.
    ///
    /// # Arguments
    ///
    /// * `maybe_value` - Some(1..=8) or None.
    ///
    /// # Panics
    ///
    /// Panics if value is 0 or > 8.
    pub fn new(maybe_value: Option<u8>) -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    /// Creates an empty tile.
    pub fn empty() -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    /// Returns `Some(value)` if tile contains a value, otherwise returns `None`.
    pub fn number(&self) -> Option<u8> {
        // TODO: your code here.
        unimplemented!()
    }

    /// Returns true if tile does not contain a value.
    pub fn is_empty(&self) -> bool {
        // TODO: your code here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Represents a 3x3 board of tiles.
pub struct Board {
    tiles: [[Tile; 3]; 3],
}

impl Board {
    /// Creates a new `Board` from a 3x3 matrix if `Tile`s.
    ///
    /// # Panics
    ///
    /// Panics if `tiles` contains more than one instance if some tile.
    pub fn new(tiles: [[Tile; 3]; 3]) -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    /// Returns a tile on a given `row` and `col`.
    ///
    /// # Panics
    ///
    /// Panics if `row` or `col` > 2.
    pub fn get(&self, row: usize, col: usize) -> Tile {
        // TODO: your code here.
        unimplemented!()
    }

    /// Swaps two given tiles.
    ///
    /// # Panics
    ///
    /// Panics if some of `r1`, `r2`, `c1` or `c2` > 2.
    pub fn swap(&mut self, r1: usize, c1: usize, r2: usize, c2: usize) {
        // TODO: your code here.
        unimplemented!()
    }

    /// Parses `Board` from string.
    ///
    /// # Arguments
    ///
    /// * `s` must be a string in the following format:
    ///
    /// '''
    /// .12
    /// 345
    /// 678
    /// '''
    ///
    /// # Panics
    ///
    /// Panics of `s` is the wrong format or does not represent a valid `Board`.
    pub fn from_string(s: &str) -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    /// Returns a string representation of this board in the following format:
    ///
    /// '''
    /// .12
    /// 345
    /// 678
    /// '''
    pub fn to_string(&self) -> String {
        // TODO: your code here.
        unimplemented!()
    }

    // You might want to add some more methods here.
}

////////////////////////////////////////////////////////////////////////////////

/// Returns the shortest sequence of moves that solves this board.
/// That is, a sequence of boards such that each consecutive board can be obtained from
/// the previous one via a single swap of an empty tile with some adjacent tile,
/// and the final board in the sequence is
///
/// '''
/// 123
/// 456
/// 78.
/// '''
///
/// If the board is unsolvable, returns `None`. If the board is already solved,
/// returns an empty vector.
pub fn solve(start: Board) -> Option<Vec<Board>> {
    // TODO: your code here.
    unimplemented!()
}
