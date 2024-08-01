use std::error::Error;
use std::{io, thread};
use std::time::Duration;
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use snake_rs::{common, game, MAP_SIZE, render};
use snake_rs::common::Drawable;
use snake_rs::player::Player;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let game = game::Game::new("Gabriel".to_string());
    let mut player = Player::new(MAP_SIZE);

    let mut last_frame = common::new_frame();
    render::render(&mut stdout, &last_frame, &last_frame, true);

    player.draw(&mut last_frame);

    render::render(&mut stdout, &last_frame, &last_frame, true);

    'gameloop: loop {
        // Game logic goes here
        // break 'gameloop;
        let mut new_frame = common::new_frame();
        if event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    KeyCode::Left => player.moveLeft(),
                    KeyCode::Right => player.moveRight(),
                    KeyCode::Up => player.moveUp(),
                    KeyCode::Down => player.moveDown(),
                    _ => ()
                }
            }
        }


        player.move_to_direction();
        player.draw(&mut new_frame);
        render::render(&mut stdout, &last_frame, &new_frame, true);

        thread::sleep(Duration::from_millis(1000))
    }

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    println!("Hello, world!");

    Ok(())
}