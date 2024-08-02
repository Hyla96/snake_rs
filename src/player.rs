use std::time::Duration;
use rusty_time::Timer;
use crate::{common, PLAYER_POSITION_INTERVAL};
use crate::common::{Direction, Drawable, Frame, Position};
use crate::game::Game;

pub struct Player {
    pub body: Vec<common::Position>,
    pub dead: bool,
    direction: common::Direction,
    timer: Timer,
    map_size: usize,
}


impl Player {
    pub fn new(size: usize) -> Self {
        Self {
            body: Self::new_body(&size),
            direction: Direction::new_random(),
            dead: false,
            timer: Timer::new(Duration::from_millis(PLAYER_POSITION_INTERVAL)),
            map_size: size,
        }
    }

    fn new_body(size: &usize) -> Vec<Position> {
        let head = Position::new_random(*size);
        let body = vec![head];


        body
    }

    pub fn update(&mut self, frame: &mut Frame, delta: &Duration) {
        self.timer.tick(*delta);

        if self.timer.finished() && !self.dead {
            let hit_boundary = self.update_to_direction();

            if hit_boundary {
                self.dead = true
            }

            self.timer.reset();
        }

        self.draw(frame);
    }

    pub fn move_to_direction(&mut self, direction: Direction) {
        match &direction {
            Direction::Down => if self.direction == Direction::Up { return; },
            Direction::Up => if self.direction == Direction::Down { return; },
            Direction::Right => if self.direction == Direction::Left { return; },
            Direction::Left => if self.direction == Direction::Right { return; },
        }

        self.direction = direction;
    }

    fn update_to_direction(&mut self) -> bool {
        match self.direction {
            Direction::Down => self.update_down(),
            Direction::Up => self.update_up(),
            Direction::Right => self.update_right(),
            Direction::Left => self.update_left(),
        }
    }

    fn update_up(&mut self) -> bool {
        if self.body[0].y == 1 {
            return true;
        }
        self.body[0].y -= 1;
        false
    }
    fn update_down(&mut self) -> bool {
        if self.body[0].y == self.map_size - 1 {
            return true;
        }
        self.body[0].y += 1;
        false
    }
    fn update_right(&mut self) -> bool {
        if self.body[0].x == self.map_size - 1 {
            return true;
        }
        self.body[0].x += 1;
        false
    }
    fn update_left(&mut self) -> bool {
        if self.body[0].x == 1 {
            return true;
        }
        self.body[0].x -= 1;
        false
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        let character = match self.direction {
            Direction::Left => '⊏',
            Direction::Down => '⊔',
            Direction::Up => '⊓',
            Direction::Right => '⊐',
        };

        frame[self.body[0].x][self.body[0].y] = character;
    }
}