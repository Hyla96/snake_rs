use std::time::Duration;
use rusty_time::Timer;
use crate::{PLAYER_POSITION_INTERVAL};
use crate::common::{Direction, Drawable, Frame, Position};

pub struct Player {
    pub body: Vec<Position>,
    pub dead: bool,
    direction: Direction,
    timer: Timer,
    map_size: usize,
}


impl Player {
    pub fn new(size: usize) -> Self {
        let dir = Direction::new_random();
        Self {
            body: Self::new_body(&size, &dir),
            direction: dir,
            dead: false,
            timer: Timer::new(Duration::from_millis(PLAYER_POSITION_INTERVAL)),
            map_size: size,
        }
    }

    fn new_body(size: &usize, direction: &Direction) -> Vec<Position> {
        let head = Position::new_random(*size);
        let mut body = vec![head];

        for i in 1..4 {
            let pos: (usize, usize);

            if *direction == Direction::Right {
                pos = (body[0].x - i, body[0].y);
            } else if *direction == Direction::Left {
                pos = (body[0].x + i, body[0].y);
            } else if *direction == Direction::Up {
                pos = (body[0].x, body[0].y + i);
            } else {
                pos = (body[0].x, body[0].y - i);
            };

            body.push(Position::new(pos.0, pos.1))
        }

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


    fn reposition_body_elements(&mut self, new_head_position: Position) {
        let mut prev = new_head_position;

        for i in 0..self.body.len() {
            let tmp = self.body[i].clone();
            self.body[i] = prev;
            prev = tmp;
        }
    }
    fn update_up(&mut self) -> bool {
        if self.body[0].y == 1 {
            return true;
        }

        self.reposition_body_elements(Position {
            x: self.body[0].x,
            y: self.body[0].y - 1,
        });

        false
    }
    fn update_down(&mut self) -> bool {
        if self.body[0].y == self.map_size - 1 {
            return true;
        }

        self.reposition_body_elements(Position {
            x: self.body[0].x,
            y: self.body[0].y + 1,
        });

        false
    }
    fn update_right(&mut self) -> bool {
        if self.body[0].x == self.map_size - 1 {
            return true;
        }

        self.reposition_body_elements(Position {
            x: self.body[0].x + 1,
            y: self.body[0].y,
        });

        false
    }
    fn update_left(&mut self) -> bool {
        if self.body[0].x == 1 {
            return true;
        }

        self.reposition_body_elements(Position {
            x: self.body[0].x - 1,
            y: self.body[0].y,
        });

        false
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        let head_character = match self.direction {
            Direction::Left => '⊏',
            Direction::Down => '⊔',
            Direction::Up => '⊓',
            Direction::Right => '⊐',
        };

        let body_character = 'o';

        for (i, e) in self.body.iter().enumerate() {
            if i == 0 {
                frame[self.body[0].x][self.body[0].y] = head_character;
            } else {
                frame[self.body[i].x][self.body[i].y] = body_character;
            }
        }
    }
}