use crate::common;
use crate::common::{Direction, Drawable, Frame, Position};
use crate::game::Game;

pub struct Player {
    pub body: Vec<common::Position>,
    pub direction: common::Direction,
}


impl Player {
    pub fn new(map_size: usize) -> Self {
        Self {
            body: vec![Position::new_random(map_size)],
            direction: Direction::new_random(),
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.body[0].x][self.body[0].y] = 'A';
    }
}