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

    pub fn update(&mut self) {}

    pub fn move_to_direction(&mut self) {
        match self.direction {
            Direction::Down => self.move_down(),
            Direction::Up => self.move_up(),
            Direction::Right => self.move_right(),
            Direction::Left => self.move_left(),
        }
    }
    
    fn move_up(&mut self) {
        self.body[0].y -= 1;
    }
    fn move_down(&mut self) {
        self.body[0].y += 1;
    }
    fn move_right(&mut self) {
        self.body[0].x += 1;
    }
    fn move_left(&mut self) {
        self.body[0].x -= 1;
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.body[0].x][self.body[0].y] = 'A';
    }
}