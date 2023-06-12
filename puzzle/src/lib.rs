#![forbid(unsafe_code)]

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

////////////////////////////////////////////////////////////////////////////////

/// Represents a tile on a board. A tile can either be empty or a number from 1 to 8.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
        if let Some(v) = maybe_value {
            assert!(v > 0 && v <= 8);
            return Self(v);
        }

        Self(0)
    }

    /// Creates an empty tile.
    pub fn empty() -> Self {
        Self(0)
    }

    /// Returns `Some(value)` if tile contains a value, otherwise returns `None`.
    pub fn number(&self) -> Option<u8> {
        if self.0 != 0 {
            return Some(self.0);
        }
        None
    }

    /// Returns true if tile does not contain a value.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Represents a 3x3 board of tiles.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
        let set = tiles.iter().flatten().cloned().collect::<HashSet<_>>();
        assert_eq!(set.len(), 9);

        Self { tiles }
    }

    /// Returns a tile on a given `row` and `col`.
    ///
    /// # Panics
    ///
    /// Panics if `row` or `col` > 2.
    pub fn get(&self, row: usize, col: usize) -> Tile {
        assert!(row <= 2 && col <= 2);

        self.tiles[row][col]
    }

    /// Swaps two given tiles.
    ///
    /// # Panics
    ///
    /// Panics if some of `r1`, `r2`, `c1` or `c2` > 2.
    pub fn swap(&mut self, r1: usize, c1: usize, r2: usize, c2: usize) {
        let tmp = self.get(r1, c1);
        self.tiles[r1][c1] = self.get(r2, c2);
        self.tiles[r2][c2] = tmp;
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
        let mut tiles = [[Tile::empty(); 3]; 3];
        for (i, line) in s.split('\n').take(3).enumerate() {
            for (j, chr) in line.chars().take(3).enumerate() {
                if chr != '.' {
                    tiles[i][j] = Tile((chr as u8) - b'0');
                }
            }
        }
        Self::new(tiles)
    }

    /// Returns a string representation of this board in the following format:
    ///
    /// '''
    /// .12
    /// 345
    /// 678
    /// '''
    pub fn to_string(&self) -> String {
        let mut res = String::with_capacity(12);
        for cur in self.tiles {
            for tile in cur {
                let ch = if tile.is_empty() {
                    '.'
                } else {
                    (b'0' + tile.0) as char
                };
                res.push(ch);
            }
            res.push('\n');
        }

        res
    }

    // You might want to add some more methods here.

    pub fn find_empty(&self) -> (usize, usize) {
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if tile.is_empty() {
                    return (i, j);
                }
            }
        }
        unreachable!("The loop should always return");
    }

    pub fn is_solved(&self) -> bool {
        let mut tiles = [[Tile::empty(); 3]; 3];
        for i in 0..8 {
            tiles[i / 3][i % 3] = Tile((i + 1) as u8);
        }
        return self.tiles == tiles;
    }
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
    if start.is_solved() {
        return Some(Vec::new());
    }

    let mut que = VecDeque::new();
    let mut trace: HashMap<Board, Board> = HashMap::new();
    que.push_back(start.clone());
    trace.insert(start.clone(), start.clone());

    while let Some(board) = que.pop_front() {
        let (x, y) = board.find_empty();

        for h in [usize::MAX, 0, 1] {
            for v in [usize::MAX, 0, 1] {
                if (h == 0) == (v == 0) {
                    continue;
                }

                let new_x = x.wrapping_add(h);
                let new_y = y.wrapping_add(v);
                if new_x >= 3 || new_y >= 3 {
                    continue;
                }
                let mut brd_copy = board.clone();
                brd_copy.swap(x, y, new_x, new_y);

                if brd_copy.is_solved() {
                    return Some(build_solution(&start, &trace, &board, brd_copy.clone()));
                }

                if let Entry::Vacant(ent) = trace.entry(brd_copy.clone()) {
                    ent.insert(board.clone());
                    que.push_back(brd_copy);
                };
            }
        }
    }

    return None;
}

fn build_solution(
    start: &Board,
    trace: &HashMap<Board, Board>,
    board: &Board,
    brd_copy: Board,
) -> Vec<Board> {
    let mut ans = Vec::new();
    ans.push(brd_copy);
    let mut cur = board;
    loop {
        if *cur == *start {
            ans.reverse();
            return ans;
        };
        ans.push(cur.clone());
        cur = trace.get(cur).unwrap();
    }
}
