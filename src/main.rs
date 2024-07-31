use std::error::Error;
use std::{io, thread};
use std::time::Duration;
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use snake_rs::{game, MAP_SIZE};
use snake_rs::player::Player;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let game = game::Game::new("Gabriel".to_string());
    let player = Player::new(MAP_SIZE);

    'gameloop: loop {
        // Game logic goes here
        // break 'gameloop;
        if event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    },
                    _ => ()
                }
            }
        }





    }

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    println!("Hello, world!");

    Ok(())
}