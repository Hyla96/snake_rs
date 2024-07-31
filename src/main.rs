use std::error::Error;
use std::{io, thread};
use std::time::Duration;
use crossterm::{ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;



    thread::sleep(Duration::from_secs(2));


    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    println!("Hello, world!");

    Ok(())
}