use crossterm::{
    cursor,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};
use std::thread::sleep;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    // sleep(std::time::Duration::from_secs(1));
    
    let initial_string = "here starts:";
    let initial_string_length = initial_string.len();
    
    write!(stdout, "{}", initial_string)?;
    // execute!(stdout, cursor::MoveRight(initial_string_length.try_into().unwrap()))?;
    
    sleep(std::time::Duration::from_secs(2));
    write!(stdout, "⭐️")?;
    sleep(std::time::Duration::from_secs(2));
    execute!(stdout, cursor::MoveTo(0,0))?;
    write!(stdout, "H")?;
    
    Ok(())
}
