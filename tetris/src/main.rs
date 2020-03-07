use std::io::{stdout, Write};

use crossterm::{
    cursor::{DisableBlinking, MoveTo},
    execute,
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};

mod tetromino;

use tetromino::{TetrominoType, Tetromino};

fn main() -> Result<()> {
    let mut stdout = stdout();

    // set up our terminal and cursor
    execute!(
        stdout, 
        terminal::Clear(terminal::ClearType::All),
        DisableBlinking,
    )?;

    for y in 0..19 {
        for x in 0..13 {
            if (y == 0 || y == 19 - 1) || (x == 0 || x == 13 - 1) {
                // in this loop we are more efficient by not flushing the buffer.
                stdout
                    .queue(MoveTo(x, y))?
                    .queue(style::PrintStyledContent("█".magenta()))?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}
