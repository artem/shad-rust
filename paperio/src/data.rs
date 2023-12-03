use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////

pub const MAP_SIZE: i32 = 930;
pub const CELL_SIZE: i32 = 30;
pub const MAP_SIZE_CELLS: i32 = MAP_SIZE / CELL_SIZE;

////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(tag = "type", content = "params", rename_all = "snake_case")]
pub enum Message {
    StartGame(GameParams),
    Tick(World),
    EndGame {},
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct GameParams {
    pub x_cells_count: u32,
    pub y_cells_count: u32,
    pub speed: u32,
    pub width: u32,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct World {
    pub players: HashMap<PlayerId, Player>,
    pub bonuses: Vec<Bonus>,
    pub tick_num: u32,
}

pub type PlayerId = String;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Player {
    pub score: u32,
    pub territory: Vec<Point>,
    pub position: Point,
    pub lines: Vec<Point>,
    pub direction: Option<Direction>,
    pub bonuses: Vec<BonusEffect>,
}

#[derive(Deserialize, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Point(pub i32, pub i32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy, FromPrimitive, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct BonusEffect {
    #[serde(rename = "type")]
    pub type_: BonusType,
    pub ticks: u32,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Bonus {
    #[serde(rename = "type")]
    pub type_: BonusType,
    pub position: Point,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum BonusType {
    #[serde(rename = "n")]
    Nitro,
    #[serde(rename = "s")]
    Slowdown,
    #[serde(rename = "saw")]
    Saw,
}

#[derive(Serialize)]
pub struct CommandMessage {
    pub command: Direction,
    pub debug: String,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Cell(pub i32, pub i32);

////////////////////////////////////////////////////////////////////////////////

impl World {
    pub fn me(&self) -> &Player {
        self.players.get("i").unwrap()
    }

    pub fn iter_enemies(&self) -> impl Iterator<Item = (&PlayerId, &Player)> {
        self.players.iter().filter_map(|(player_id, player)| {
            if player_id != "i" {
                Some((player_id, player))
            } else {
                None
            }
        })
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = Cell> {
        (0..MAP_SIZE_CELLS).flat_map(|x| (0..MAP_SIZE_CELLS).map(move |y| Cell(x, y)))
    }
}

impl Direction {
    pub fn next(self, clockwise: bool) -> Direction {
        let delta = if clockwise { 1 } else { -1 };
        Self::from_i32((self as i32 + delta + 4) % 4).unwrap()
    }

    pub fn opposite(self) -> Direction {
        Self::from_i32((self as i32 + 2) % 4).unwrap()
    }
}

impl Point {
    pub fn to_cell(self) -> Cell {
        Cell(self.0 / CELL_SIZE, self.1 / CELL_SIZE)
    }
}

impl Cell {
    pub fn distance_to(self, other: Cell) -> i32 {
        (other.0 - self.0).abs() + (other.1 - self.1).abs()
    }

    pub fn direction_to(self, other: Cell) -> Direction {
        let (dx, dy) = (other.0 - self.0, other.1 - self.1);
        if dx.abs() > dy.abs() {
            if dx > 0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else if dy > 0 {
            Direction::Up
        } else {
            Direction::Down
        }
    }

    pub fn iter_neighbors(self) -> impl Iterator<Item = Cell> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |(dx, dy)| {
                let neigh = Cell(self.0 + dx, self.1 + dy);
                if neigh.in_bounds() {
                    Some(neigh)
                } else {
                    None
                }
            })
    }

    pub fn adjacent_unchecked(self, dir: Direction) -> Cell {
        match dir {
            Direction::Down => Cell(self.0, self.1 - 1),
            Direction::Up => Cell(self.0, self.1 + 1),
            Direction::Left => Cell(self.0 - 1, self.1),
            Direction::Right => Cell(self.0 + 1, self.1),
        }
    }

    pub fn adjacent(self, dir: Direction) -> Option<Cell> {
        let cell = self.adjacent_unchecked(dir);
        if cell.in_bounds() {
            Some(cell)
        } else {
            None
        }
    }

    pub fn in_bounds(self) -> bool {
        self.0 >= 0 && self.0 < MAP_SIZE_CELLS && self.1 >= 0 && self.1 < MAP_SIZE_CELLS
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_test() {
        let start_game = serde_json::from_str::<Message>(
            r#"{
                "type": "start_game",
                "params": {
                    "x_cells_count": 345,
                    "y_cells_count": 567,
                    "speed": 42,
                    "width": 56
                }
            }"#,
        )
        .unwrap();

        assert_eq!(
            start_game,
            Message::StartGame(GameParams {
                x_cells_count: 345,
                y_cells_count: 567,
                speed: 42,
                width: 56,
            })
        );

        let tick = serde_json::from_str::<Message>(
            r#"{
                "type": "tick",
                "params": {
                    "players": {
                        "1": {
                            "score": 123,
                            "territory": [[0, 0], [0, 1]],
                            "position": [0, 1],
                            "lines": [[1, 0], [1, 1]],
                            "direction": "left",
                            "bonuses": [
                                {
                                    "type": "n",
                                    "ticks": 456
                                }
                            ]
                        }
                    },
                    "bonuses": [
                        {
                            "type": "s",
                            "position": [34, 56]
                        },
                        {
                            "type": "saw",
                            "position": [56, 23]
                        }
                    ],
                    "tick_num": 748
                }
            }"#,
        )
        .unwrap();

        assert_eq!(
            tick,
            Message::Tick(World {
                players: vec![(
                    "1".to_string(),
                    Player {
                        score: 123,
                        territory: vec![Point(0, 0), Point(0, 1)],
                        position: Point(0, 1),
                        lines: vec![Point(1, 0), Point(1, 1)],
                        direction: Some(Direction::Left),
                        bonuses: vec![BonusEffect {
                            type_: BonusType::Nitro,
                            ticks: 456
                        }]
                    }
                )]
                .into_iter()
                .collect(),
                bonuses: vec![
                    Bonus {
                        type_: BonusType::Slowdown,
                        position: Point(34, 56),
                    },
                    Bonus {
                        type_: BonusType::Saw,
                        position: Point(56, 23),
                    }
                ],
                tick_num: 748,
            })
        );

        let end_game =
            serde_json::from_str::<Message>("{\"type\": \"end_game\", \"params\": {}}").unwrap();
        assert_eq!(end_game, Message::EndGame {});
    }
}
