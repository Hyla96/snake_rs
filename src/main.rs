use std::error::Error;
use std::{io, thread};
use std::time::{Duration, Instant};
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use snake_rs::{common, game, MAP_SIZE, render};
use snake_rs::common::{Direction, Drawable};
use snake_rs::game::Game;
use snake_rs::player::Player;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let game = Game::new("Gabriel".to_string());
    let mut player = Player::new(MAP_SIZE);

    let mut last_frame = common::new_frame();
    render::render(&mut stdout, &last_frame, &last_frame, true);

    player.update(&mut last_frame, &Duration::new(0, 0));

    render::render(&mut stdout, &last_frame, &last_frame, true);

    let mut instant = Instant::now();

    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut new_frame = common::new_frame();

        if event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    KeyCode::Left => player.move_to_direction(Direction::Left),
                    KeyCode::Right => player.move_to_direction(Direction::Right),
                    KeyCode::Up => player.move_to_direction(Direction::Up),
                    KeyCode::Down => player.move_to_direction(Direction::Down),
                    _ => ()
                }
            }
        }

        player.update(&mut new_frame, &delta);

        render::render(&mut stdout, &last_frame, &new_frame, true);

        if player.dead {
            break 'gameloop;
        }

        thread::sleep(Duration::from_millis(1))
    }

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}