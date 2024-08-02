use rand::{Rng};
use crate::MAP_SIZE;

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new_random(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(size / 5..size - size / 5);
        let y = rng.gen_range(size / 5..size - size / 5);
        Position {
            x,
            y,
        }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let n: u32 = rng.gen_range(0..3);

        match n {
            0 => Direction::Left,
            1 => Direction::Up,
            2 => Direction::Down,
            _ => Direction::Right,
        }
    }
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

pub type Frame = [[char; MAP_SIZE + 2]; MAP_SIZE + 2];

pub fn new_frame() -> Frame {
    let mut frame = [[' '; MAP_SIZE + 2]; MAP_SIZE + 2];

    for x in 0..(MAP_SIZE + 2) {
        for y in 0..(MAP_SIZE + 2) {
            if x == 0 || x == MAP_SIZE + 1 || y == 0 || y == MAP_SIZE + 1 {
                frame[x][y] = '#'
            }
        }
    }

    frame
}