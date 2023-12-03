use std::ops::Add;

use crate::data::{Nibble, Word};

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sprite<'a> {
    data: &'a [u8],
}

impl<'a> Sprite<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn iter_pixels(&self) -> impl Iterator<Item = Point> + '_ {
        self.data.iter().enumerate().flat_map(|(y, row)| {
            let y = y as u8;
            (0..8).filter_map(move |x| {
                let val = row & (0x80 >> x);
                if val > 0 {
                    Some(Point { x, y })
                } else {
                    None
                }
            })
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

pub type Key = Nibble;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyEventKind {
    Pressed,
    Released,
}

////////////////////////////////////////////////////////////////////////////////

pub trait Platform {
    fn draw_sprite(&mut self, pos: Point, sprite: Sprite) -> bool;
    fn clear_screen(&mut self);
    fn get_delay_timer(&self) -> Word;
    fn set_delay_timer(&mut self, value: Word);
    fn set_sound_timer(&mut self, value: Word);
    fn is_key_down(&self, key: Key) -> bool;
    fn consume_key_press(&mut self) -> Option<Key>;
    fn get_random_word(&mut self) -> Word;
}
