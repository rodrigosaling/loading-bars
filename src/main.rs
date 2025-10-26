use crossterm::{
    cursor,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};
use std::thread::sleep;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    // Clear the screen
    execute!(stdout, Clear(ClearType::All))?;

    // 1. Write the initial text.
    write!(stdout, "This is a long line of text.")?;
    stdout.flush()?;

    // Wait a moment so you can see the initial line
    sleep(std::time::Duration::from_secs(1));

    // 2. Move the cursor back to the start of the line (0,0).
    execute!(stdout, cursor::MoveTo(0, 0))?;

    // 3. Write the new, shorter text.
    write!(stdout, "New text.")?;

    // 4. Clear the line from the cursor position to the end.
    // execute!(Clear(ClearType::UntilNewLine))?;

    // 5. Add a new line so subsequent prints don't get overwritten.
    writeln!(stdout)?;
    writeln!(stdout, "Next line starts here.")?;
    
    Ok(())
}
